pub struct Solution;

fn parse_expr(s: &str) -> Option<(bool, &str)> {
    parse_const(s)
        .or_else(|| parse_not(s))
        .or_else(|| parse_and(s))
        .or_else(|| parse_or(s))
}

fn parse_char(s: &str, want: char) -> Option<((), &str)> {
    s.chars()
        .next()
        .and_then(|c| if c == want { Some(((), &s[1..])) } else { None })
}

fn parse_const(s: &str) -> Option<(bool, &str)> {
    parse_char(s, 'f')
        .map(|((), s1)| (false, s1))
        .or_else(|| parse_char(s, 't').map(|((), s1)| (true, s1)))
}

fn parse_parentheses<T>(s: &str, parser: impl Fn(&str) -> Option<(T, &str)>) -> Option<(T, &str)> {
    parse_char(s, '(')
        .and_then(|(_, s)| parser(s))
        .and_then(|(ret, s)| parse_char(s, ')').map(|(_, s)| (ret, s)))
}

fn parse_not(s: &str) -> Option<(bool, &str)> {
    parse_char(s, '!')
        .and_then(|(_, s)| parse_parentheses(s, parse_expr))
        .map(|(b, s)| (!b, s))
}

fn parse_expr_list(s: &str) -> Option<(Vec<bool>, &str)> {
    let mut res = vec![];
    let mut s = s;

    // parse 1 expression
    let (b, s1) = parse_expr(s)?;
    res.push(b);
    s = s1;
    // parse 0 or more `,expression`
    while let Some((b, s2)) = parse_char(s, ',').and_then(|(_, s1)| parse_expr(s1)) {
        res.push(b);
        s = s2;
    }

    Some((res, s))
}

fn parse_and(s: &str) -> Option<(bool, &str)> {
    parse_char(s, '&').and_then(|(_, s)| {
        parse_parentheses(s, parse_expr_list).map(|(list, s)| (list.into_iter().all(|b| b), s))
    })
}

fn parse_or(s: &str) -> Option<(bool, &str)> {
    parse_char(s, '|').and_then(|(_, s)| {
        parse_parentheses(s, parse_expr_list).map(|(list, s)| (list.into_iter().any(|b| b), s))
    })
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        parse_expr(&expression).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let expression = "&(|(f))".to_string();
        assert!(!Solution::parse_bool_expr(expression))
    }

    #[test]
    fn sample2() {
        let expression = "|(f,f,f,t)".to_string();
        assert!(Solution::parse_bool_expr(expression))
    }

    #[test]
    fn sample3() {
        let expression = "!(&(f,t))".to_string();
        assert!(Solution::parse_bool_expr(expression))
    }
}
