pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State<'a> {
    input: &'a str,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Message {
    pos: usize,
}

type ParseResult<'a, O> = Result<(O, State<'a>), Message>;

trait Parser<'a, O> {
    fn parse(&self, state: State<'a>) -> ParseResult<'a, O>;

    #[allow(unused)]
    fn and(self, other: impl Parser<'a, O>) -> impl Parser<'a, O>
    where
        Self: Sized,
    {
        move |state| {
            let (_, state) = self.parse(state)?;
            let (result2, state) = other.parse(state)?;
            Ok((result2, state))
        }
    }

    fn map<O2, F>(self, f: F) -> impl Parser<'a, O2>
    where
        Self: Sized,
        F: Fn(O) -> O2,
    {
        move |state| {
            let (result, state) = self.parse(state)?;
            Ok((f(result), state))
        }
    }

    fn or(self, other: impl Parser<'a, O>) -> impl Parser<'a, O>
    where
        Self: Sized,
    {
        move |state| match self.parse(state) {
            Err(Message { pos }) if pos == state.pos => other.parse(state),
            res => res,
        }
    }
}

impl<'a, O, F> Parser<'a, O> for F
where
    F: Fn(State<'a>) -> ParseResult<'a, O>,
{
    fn parse(&self, state: State<'a>) -> ParseResult<'a, O> {
        self(state)
    }
}

fn run_parser<'a, O, P>(parser: P, input: &'a str) -> ParseResult<'a, O>
where
    P: Parser<'a, O>,
{
    let state = State { input, pos: 0 };
    parser.parse(state)
}

#[allow(unused)]
fn pure<'a, O: Clone>(value: O) -> impl Parser<'a, O> {
    move |state| Ok((value.clone(), state))
}

#[allow(unused)]
fn try_wrapper<'a, O, P>(parser: P) -> impl Parser<'a, O>
where
    P: Parser<'a, O>,
{
    move |state| match parser.parse(state) {
        Ok((result, state)) => Ok((result, state)),
        Err(_) => Err(Message { pos: state.pos }),
    }
}

fn satisfy<'a, F>(predicate: F) -> impl Parser<'a, char>
where
    F: Fn(char) -> bool,
{
    move |state: State<'a>| match state.input.chars().next() {
        Some(c) if predicate(c) => {
            let len = c.len_utf8();
            let new_pos = state.pos + len;
            let new_state = State {
                input: &state.input[len..],
                pos: new_pos,
            };
            Ok((c, new_state))
        }
        _ => Err(Message { pos: state.pos }),
    }
}

fn satisfy_str<'a, F>(look_ahead_len: usize, predicate: F) -> impl Parser<'a, ()>
where
    F: Fn(&'a str) -> bool,
{
    move |state: State<'a>| {
        if state.input.len() >= look_ahead_len && predicate(&state.input[..look_ahead_len]) {
            let new_pos = state.pos + look_ahead_len;
            let new_state = State {
                input: &state.input[look_ahead_len..],
                pos: new_pos,
            };
            Ok(((), new_state))
        } else {
            Err(Message { pos: state.pos })
        }
    }
}

fn any_char<'a>() -> impl Parser<'a, char> {
    satisfy(|_| true)
}

fn string<'a, 'b: 'a>(s: &'b str) -> impl Parser<'a, ()> {
    move |state: State<'a>| {
        if state.input.starts_with(s) {
            let len = s.len();
            let new_pos = state.pos + len;
            let new_state = State {
                input: &state.input[len..],
                pos: new_pos,
            };
            Ok(((), new_state))
        } else {
            Err(Message { pos: state.pos })
        }
    }
}

fn character<'a>(c: char) -> impl Parser<'a, ()> {
    satisfy(move |x| x == c).map(|_| ())
}

fn one_or_more<'a, P, O>(p: P) -> impl Parser<'a, Vec<O>>
where
    P: Parser<'a, O>,
{
    move |state| {
        let (first, state) = p.parse(state)?;
        let mut results = vec![first];
        let mut current_state = state;

        while let Ok((result, new_state)) = p.parse(current_state) {
            results.push(result);
            current_state = new_state;
        }

        Ok((results, current_state))
    }
}

fn valid_tag(tag: &str) -> bool {
    !tag.is_empty() && tag.len() <= 9 && tag.chars().all(|c| c.is_uppercase())
}

fn open_close_tag<'a>(state: State<'a>) -> ParseResult<'a, ()> {
    fn open<'a>(state: State<'a>) -> ParseResult<'a, String> {
        let ((), state) = character('<').parse(state)?;
        let (tag_name, state) = one_or_more(satisfy(|c| c != '>')).parse(state)?;
        let ((), state) = character('>').parse(state)?;

        let tag_name: String = tag_name.iter().collect();
        if valid_tag(&tag_name) {
            Ok((tag_name, state))
        } else {
            Err(Message { pos: state.pos })
        }
    }

    fn close<'a>(state: State<'a>, tag_name: String) -> ParseResult<'a, ()> {
        let ((), state) = string("</").parse(state)?;
        let (tag_name2, state) = one_or_more(satisfy(|c| c != '>')).parse(state)?;
        let ((), state) = character('>').parse(state)?;

        let tag_name2: String = tag_name2.iter().collect();
        if valid_tag(&tag_name2) && tag_name == tag_name2 {
            Ok(((), state))
        } else {
            Err(Message { pos: state.pos })
        }
    }

    let (tag_name, state) = open(state)?;
    let ((), state) = tag_content(state)?;
    let ((), state) = close(state, tag_name)?;
    Ok(((), state))
}

fn tag_content<'a>(mut state: State<'a>) -> ParseResult<'a, ()> {
    let p = cdata.or(open_close_tag).or(any_char().map(|_| ()));

    while let Ok(((), new_state)) = p.parse(state) {
        state = new_state;
    }
    Ok(((), state))
}

fn cdata<'a>(state: State<'a>) -> ParseResult<'a, ()> {
    let ((), mut state) = string("<![CDATA[").parse(state)?;

    loop {
        if let Ok(((), new_state)) = satisfy_str(3, |s| s == "]]>").parse(state) {
            state = new_state;
            break;
        }
        let (_, new_state) = any_char().parse(state)?;
        state = new_state;
    }

    Ok(((), state))
}

impl Solution {
    pub fn is_valid(code: String) -> bool {
        let res = run_parser(open_close_tag, &code);
        // dbg!(&res);
        // if let Err(Message { pos }) = res {
        //     dbg!(&code[pos..]);
        // }

        matches!(res, Ok(((), state)) if state.input.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let input = "<![CDATA[<div>]]>";
        let state = State { input, pos: 0 };
        let result = string("<![CDATA[").parse(state);
        assert_eq!(
            result,
            Ok((
                (),
                State {
                    input: "<div>]]>",
                    pos: 9
                }
            ))
        );
    }

    #[test]
    fn test_fold() {
        let input = "<DIV><![CDATA[a]]></DIV>";
        let state = State { input, pos: 0 };
        let result = open_close_tag(state);
        if let Err(Message { pos }) = result {
            println!("Error at position: {}", pos);
            println!("Remaining input: {}", &input[pos..]);
        }
        assert_eq!(result, Ok(((), State { input: "", pos: 24 })));
    }

    #[test]
    fn sample1() {
        let code = "<DIV>This is the first line <![CDATA[<div>]]></DIV>";
        assert!(Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample2() {
        let code = "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>";
        assert!(Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample3() {
        let code = "<A>  <B> </A>   </B>";
        assert!(!Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample4() {
        let code = "<DIV>  div tag is not closed  <DIV>";
        assert!(!Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample5() {
        let code = "<DIV>  unmatched <  </DIV>";
        assert!(!Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample6() {
        let code = "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>";
        assert!(!Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample7() {
        let code =
            "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>";
        assert!(!Solution::is_valid(code.to_string()));
    }

    #[test]
    fn sample8() {
        let code = "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>";
        assert!(!Solution::is_valid(code.to_string()));
    }

    #[test]
    fn issue1() {
        let code = "<A></A><B></B>";
        assert!(!Solution::is_valid(code.to_string()));
    }
}
