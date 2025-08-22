pub struct Solution;

pub mod parser_module {
    pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

    pub trait Parser<'a, Output> {
        fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

        fn map<F, Output2>(self, map_fn: F) -> impl Parser<'a, Output2>
        where
            Self: Sized,
            F: Fn(Output) -> Output2,
        {
            move |input| {
                let (next_input, output) = self.parse(input)?;
                Ok((next_input, map_fn(output)))
            }
        }

        fn pair<Output2, P2>(self, other: P2) -> impl Parser<'a, (Output, Output2)>
        where
            Self: Sized,
            P2: Parser<'a, Output2>,
        {
            move |input1| {
                let (input2, output1) = self.parse(input1)?;
                let (next_input, output2) = other.parse(input2)?;
                Ok((next_input, (output1, output2)))
            }
        }

        fn and<Output2, P2>(self, other: P2) -> impl Parser<'a, Output2>
        where
            Self: Sized,
            P2: Parser<'a, Output2>,
        {
            move |input| {
                let (next_input, _) = self.parse(input)?;
                other.parse(next_input)
            }
        }

        fn and_then<Output2, F, P2>(self, f: F) -> impl Parser<'a, Output2>
        where
            Self: Sized,
            F: Fn(Output) -> P2,
            P2: Parser<'a, Output2>,
        {
            move |input| {
                let (next_input, output1) = self.parse(input)?;
                f(output1).parse(next_input)
            }
        }

        fn or<P2>(self, other: P2) -> impl Parser<'a, Output>
        where
            Self: Sized,
            P2: Parser<'a, Output>,
        {
            move |input| self.parse(input).or_else(|_| other.parse(input))
        }
    }

    impl<'a, F, Output> Parser<'a, Output> for F
    where
        F: Fn(&'a str) -> ParseResult<'a, Output>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
            self(input)
        }
    }

    pub fn left<'a, P1, P2, O1, O2>(parser1: P1, parser2: P2) -> impl Parser<'a, O1>
    where
        P1: Parser<'a, O1>,
        P2: Parser<'a, O2>,
    {
        parser1.pair(parser2).map(|(o1, _)| o1)
    }

    pub fn right<'a, P1, P2, O1, O2>(parser1: P1, parser2: P2) -> impl Parser<'a, O2>
    where
        P1: Parser<'a, O1>,
        P2: Parser<'a, O2>,
    {
        parser1.pair(parser2).map(|(_, o2)| o2)
    }

    pub fn one_or_more<'a, P, O>(parser: P) -> impl Parser<'a, Vec<O>>
    where
        P: Parser<'a, O>,
    {
        move |input| {
            let mut results = Vec::new();
            let mut current_input = input;

            while let Ok((next_input, output)) = parser.parse(current_input) {
                results.push(output);
                current_input = next_input;
            }

            if results.is_empty() {
                Err(input)
            } else {
                Ok((current_input, results))
            }
        }
    }

    pub fn zero_or_more<'a, P, O>(parser: P) -> impl Parser<'a, Vec<O>>
    where
        P: Parser<'a, O>,
    {
        move |input| {
            let mut results = Vec::new();
            let mut current_input = input;

            while let Ok((next_input, item)) = parser.parse(current_input) {
                results.push(item);
                current_input = next_input;
            }

            Ok((current_input, results))
        }
    }

    pub fn pred<'a, P, O, F>(parser: P, predicate: F) -> impl Parser<'a, O>
    where
        P: Parser<'a, O>,
        F: Fn(&O) -> bool,
    {
        move |input| {
            let (next_input, output) = parser.parse(input)?;
            if predicate(&output) {
                Ok((next_input, output))
            } else {
                Err(input)
            }
        }
    }

    pub fn any_char<'a>(input: &'a str) -> ParseResult<'a, char> {
        if let Some(first_char) = input.chars().next() {
            Ok((&input[first_char.len_utf8()..], first_char))
        } else {
            Err(input)
        }
    }

    pub fn given_char<'a>(c: char) -> impl Parser<'a, char> {
        pred(any_char, move |&x| x == c)
    }

    pub fn parens<'a, O, P>(parser: P) -> impl Parser<'a, O>
    where
        P: Parser<'a, O>,
    {
        let open_paren = given_char('{');
        let close_paren = given_char('}');

        move |input| {
            let (next_input, _) = open_paren.parse(input)?;
            let (next_input, output) = parser.parse(next_input)?;
            let (next_input, _) = close_paren.parse(next_input)?;

            Ok((next_input, output))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_any_char() {
            let input = "abc";
            let result = any_char.parse(input);
            assert_eq!(result, Ok(("bc", 'a')));
        }

        #[test]
        fn test_many() {
            let input = "abc";
            let parser = zero_or_more(any_char);
            let result = parser.parse(input);
            assert_eq!(result, Ok(("", vec!['a', 'b', 'c'])));
        }

        #[test]
        fn test_pred() {
            let input = "abc12";
            let parser = zero_or_more(pred(any_char, |&c| c.is_ascii_alphabetic()));
            let result = parser.parse(input);
            assert_eq!(result, Ok(("12", vec!['a', 'b', 'c'])));
        }
    }
}

use std::collections::BTreeSet;

use parser_module::{ParseResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Word(String),
    Union(Vec<Expr>),
    Concat(Vec<Expr>),
}

impl Expr {
    fn is_empty(&self) -> bool {
        match self {
            Expr::Word(_) => false,
            Expr::Union(v) => v.is_empty(),
            Expr::Concat(v) => v.is_empty(),
        }
    }

    fn eval(&self) -> BTreeSet<String> {
        match self {
            Expr::Word(w) => {
                let mut set = BTreeSet::new();
                set.insert(w.clone());
                set
            }
            Expr::Union(v) => {
                let mut set = BTreeSet::new();
                for expr in v {
                    set.extend(expr.eval());
                }
                set
            }
            Expr::Concat(v) => {
                let mut current_set = BTreeSet::new();
                current_set.insert("".to_string());

                for expr in v {
                    let new_set = expr.eval();
                    let mut temp_set = BTreeSet::new();
                    for prefix in &current_set {
                        for suffix in &new_set {
                            temp_set.insert(format!("{prefix}{suffix}"));
                        }
                    }
                    current_set = temp_set;
                }

                current_set
            }
        }
    }
}

fn parser_word<'a>(input: &'a str) -> ParseResult<'a, Expr> {
    fn letter<'a>(input: &'a str) -> ParseResult<'a, char> {
        parser_module::pred(parser_module::any_char, |&c| c.is_ascii_alphabetic()).parse(input)
    }

    fn word<'a>(input: &'a str) -> ParseResult<'a, String> {
        parser_module::one_or_more(letter)
            .parse(input)
            .map(|(next_input, v)| (next_input, v.into_iter().collect::<String>()))
    }

    word.map(Expr::Word).parse(input)
}

fn parser_union<'a>(input: &'a str) -> ParseResult<'a, Expr> {
    fn inner<'a>(input: &'a str) -> ParseResult<'a, Expr> {
        let mut v = vec![];
        let (mut left_input, first) = parser_expr(input)?;
        v.push(first);
        while let Ok((next_input, ch)) = parser_module::given_char(',')
            .and(parser_expr)
            .parse(left_input)
        {
            v.push(ch);
            left_input = next_input;
        }
        Ok((left_input, Expr::Union(v)))
    }

    parser_module::parens(inner).parse(input)
}

fn parser_expr<'a>(input: &'a str) -> ParseResult<'a, Expr> {
    fn case1<'a>(input: &'a str) -> ParseResult<'a, Expr> {
        let (input, ch) = parser_word(input)?;
        let (input, expr) = parser_expr_empty(input)?;
        if expr.is_empty() {
            Ok((input, ch))
        } else {
            Ok((input, Expr::Concat(vec![ch, expr])))
        }
    }

    fn case2<'a>(input: &'a str) -> ParseResult<'a, Expr> {
        let (input, output1) = parser_union(input)?;
        let (input, output2) = parser_expr_empty(input)?;
        if output2.is_empty() {
            Ok((input, output1))
        } else {
            Ok((input, Expr::Concat(vec![output1, output2])))
        }
    }

    let parser = case1.or(case2);
    parser.parse(input)
}

fn parser_expr_empty<'a>(input: &'a str) -> ParseResult<'a, Expr> {
    parser_module::zero_or_more(parser_expr)
        .parse(input)
        .map(|(next_input, mut v)| {
            (
                next_input,
                if v.len() == 1 {
                    v.pop().unwrap()
                } else {
                    Expr::Concat(v)
                },
            )
        })
}

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let res = parser_expr(&expression);
        assert!(res.is_ok());
        let (remaining, expr) = res.unwrap();
        assert!(remaining.is_empty());

        expr.eval().into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "{a,b}{c,{d,e}}";
        let result = parser_expr(input);
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        // println!("expr: {:#?}", expr);
        assert_eq!(remaining, "");
        assert_eq!(
            expr,
            Expr::Concat(vec![
                Expr::Union(vec![Expr::Word("a".into()), Expr::Word("b".into())]),
                Expr::Union(vec![
                    Expr::Word("c".into()),
                    Expr::Union(vec![Expr::Word("d".into()), Expr::Word("e".into())])
                ])
            ])
        );
    }

    #[test]
    fn sample1() {
        let expression = "{a,b}{c,{d,e}}".to_string();
        let result = ["ac", "ad", "ae", "bc", "bd", "be"];
        let output = Solution::brace_expansion_ii(expression);
        assert!(result.len() == output.len());
        for (o, r) in output.iter().zip(result.iter()) {
            assert!(o == r);
        }
    }

    #[test]
    fn sample2() {
        let expression = "{{a,z},a{b,c},{ab,z}}".to_string();
        let result = ["a", "ab", "ac", "z"];
        let output = Solution::brace_expansion_ii(expression);
        assert!(result.len() == output.len());
        for (o, r) in output.iter().zip(result.iter()) {
            assert!(o == r);
        }
    }

    #[test]
    fn test_issue() {
        let input = "{a,z}";
        let result = parser_expr(input);
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert!(remaining.is_empty());
        println!("expr: {expr:#?}");

        let input = "a{b,c}";
        let result = parser_expr(input);
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert!(remaining.is_empty());
        println!("expr: {expr:#?}");

        let input = "ab";
        let result = parser_expr(input);
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert!(remaining.is_empty());
        println!("expr: {expr:#?}");

        let input = "{{ab,z},{c,d}}";
        let result = parser_expr(input);
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert!(remaining.is_empty());
        println!("expr: {expr:#?}");
    }
}
