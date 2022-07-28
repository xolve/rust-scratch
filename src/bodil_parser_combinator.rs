// From https://bodil.lol/parser-combinators

#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

type Input<'a> = &'a str;
type Output = Element;
type Error<'a> = &'a str;
type ParseResult<'a, R> = Result<(&'a str, R), &'a str>;

// type Parser = dyn Fn(Input) -> Result<(Input, Output), Error>;
trait Parser<'a, R> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, R>;

    fn map<F, S>(self, map_fn: F) -> BoxedParser<'a, S>
    where
        Self: Sized + 'a,
        R: 'a,
        S: 'a,
        F: Fn(R) -> S + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, predicate: F) -> BoxedParser<'a, R>
    where
        Self: Sized + 'a,
        R: 'a,
        F: Fn(&R) -> bool + 'a,
    {
        BoxedParser::new(pred(self, predicate))
    }

    fn and_then<F, S, NextP>(self, f: F) -> BoxedParser<'a, S>
    where
        Self: Sized + 'a,
        R: 'a,
        S: 'a,
        NextP: Parser<'a, S> + 'a,
        F: Fn(R) -> NextP + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
}

impl<'a, F, R> Parser<'a, R> for F
where
    F: Fn(&'a str) -> ParseResult<'a, R>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, R> {
        self(input)
    }
}

fn match_literal<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    // fn match_literal<'a>(expected: &str) -> impl Fn(&str) -> Result<(&str, ()), &str>  + '_ {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

fn identifier(input: &str) -> ParseResult<String> {
    // fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();

    let first_char = input.chars().take(1).next();

    if first_char.map(|c| c.is_alphanumeric()) == Some(true) {
        matched.push(first_char.unwrap());

        input
            .chars()
            .skip(1)
            .take_while(|c| c.is_alphanumeric() || *c == '-')
            .for_each(|c| matched.push(c));

        let next_index = matched.len();
        Ok((&input[next_index..], matched))
    } else {
        Err(input)
    }
}

// fn map<P, F, R, S>(parser: P, map_fn: F) -> impl Fn(&str) -> ParseResult<S>
// where P: Fn(&str) -> ParseResult<R>,
//       F: Fn(R) -> S {
//     move |input|
//         { parser(input).map(|(rest, result)| (rest, map_fn(result))) }
// }
// After adding Parser<'a, R> as trait above can be written as following

fn map<'a, P, F, R, S>(parser: P, map_fn: F) -> impl Parser<'a, S>
where
    P: Parser<'a, R>,
    F: Fn(R) -> S,
{
    move |input| {
        parser
            .parse(input)
            .map(|(rest, result)| (rest, map_fn(result)))
    }
}

fn pair<'a, P1, R1, P2, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(rest, result2)| (rest, (result1, result2)))
        })
    }
}

fn left<'a, P1, R1, P2, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(result1, _)| result1)
}

fn right<'a, P1, R1, P2, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_, result2)| result2)
}

fn one_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
{
    move |input| {
        let mut results = Vec::new();

        parser
            .parse(input)
            .and_then(|(rest_of_input, first_result)| {
                results.push(first_result);

                let mut next_input = rest_of_input;
                while let Ok((rest_of_input, next_item)) = parser.parse(next_input) {
                    next_input = rest_of_input;
                    results.push(next_item);
                }

                Ok((next_input, results))
            })
    }
}

fn zero_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
{
    move |input| {
        let mut results = Vec::new();

        let mut next_input = input;
        while let Ok((rest_of_input, next_item)) = parser.parse(next_input) {
            next_input = rest_of_input;
            results.push(next_item);
        }

        Ok((next_input, results))
    }
}

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

fn pred<'a, P, R, F>(parser: P, predicate: F) -> impl Parser<'a, R>
where
    P: Parser<'a, R>,
    F: Fn(&R) -> bool,
{
    move |input| {
        parser.parse(input).and_then(|(rest, result)| {
            if predicate(&result) {
                Ok((rest, result))
            } else {
                Err(input)
            }
        })
    }
}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

fn quoted_string<'a>() -> BoxedParser<'a, String> {
    right(
        match_literal("\""),
        left(
            zero_or_more(any_char.pred(|c| *c != '"')),
            match_literal("\""),
        ),
    )
    .map(|cs| cs.iter().collect())
}

fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    pair(identifier, right(match_literal("="), quoted_string()))
}

fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    zero_or_more(right(space1(), attribute_pair()))
}

struct BoxedParser<'a, R> {
    parser: Box<dyn Parser<'a, R> + 'a>,
}

impl<'a, R> BoxedParser<'a, R> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, R> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, R> Parser<'a, R> for BoxedParser<'a, R> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, R> {
        self.parser.parse(input)
    }
}

fn single_element<'a>() -> impl Parser<'a, Element> {
    right(
        match_literal("<"),
        left(pair(identifier, attributes()), match_literal("/>")),
    )
    .map(|(name, attributes)| Element {
        name,
        attributes,
        children: Default::default(),
    })
}

fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String, String)>)> {
    right(match_literal("<"), pair(identifier, attributes()))
}

fn open_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), match_literal(">")).map(|(name, attributes)| Element {
        name,
        attributes,
        children: vec![],
    })
}

fn open_element_<'a>() -> impl Parser<'a, Element> {
    right(
        match_literal("<"),
        left(pair(identifier, attributes()), match_literal(">")),
    )
    .map(|(name, attributes)| Element {
        name,
        attributes,
        children: Default::default(),
    })
}

fn either<'a, P1, P2, R>(parser1: P1, parser2: P2) -> impl Parser<'a, R>
where
    P1: Parser<'a, R>,
    P2: Parser<'a, R>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

fn element<'a>() -> impl Parser<'a, Element> {
    whitespace_wrap(either(single_element(), parent_element()))
}

fn close_element<'a>(expected_name: String) -> impl Parser<'a, String> {
    right(
        match_literal("</"),
        left(
            identifier, // try with just match literal
            match_literal(">"),
        ),
    )
    .pred(move |id| *id == expected_name)
}

fn and_then<'a, P, F, R, S, NextP>(parser: P, f: F) -> impl Parser<'a, S>
where
    P: Parser<'a, R>,
    NextP: Parser<'a, S>,
    F: Fn(R) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

fn parent_element<'a>() -> impl Parser<'a, Element> {
    open_element().and_then(|el| {
        left(zero_or_more(element()), close_element(el.name.clone())).map(move |children| {
            let mut el = el.clone();
            el.children = children;
            el
        })
    })
}

fn whitespace_wrap<'a, P, R>(parser: P) -> impl Parser<'a, R>
where
    P: Parser<'a, R>,
{
    right(space0(), left(parser, space0()))
}

#[test]
fn single_element_parser() {
    assert_eq!(
        Ok((
            "",
            Element {
                name: "div".to_string(),
                attributes: vec![
                    ("class".to_string(), "still-ok".to_string()),
                    ("display".to_string(), "none super".to_string())
                ],
                children: Default::default()
            }
        )),
        single_element().parse("<div class=\"still-ok\" display=\"none super\"/>")
    )
}

#[test]
fn attributes_parser() {
    assert_eq!(
        Ok((
            "",
            vec![
                ("alice".to_string(), "lives in wonderland".to_string()),
                ("neo".to_string(), "follow the white rabbit".to_string())
            ]
        )),
        attributes().parse(" alice=\"lives in wonderland\" neo=\"follow the white rabbit\"")
    )
}

#[test]
fn attribute_pair_parser() {
    assert_eq!(
        Ok(("", ("alice".to_string(), "lives in wonderland".to_string()))),
        attribute_pair().parse("alice=\"lives in wonderland\"")
    )
}

#[test]
fn quoted_string_parser() {
    assert_eq!(
        Ok(("", "Hello Joe!".to_string())),
        quoted_string().parse("\"Hello Joe!\"")
    );
}

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
    assert_eq!(Ok(("ooomg", 'o')), parser.parse("oooomg"));
}

#[test]
fn one_or_more_combinator() {
    let parser = one_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), (),])), parser.parse("hahaha"));
    assert_eq!(Ok(("na", vec![(), (),])), parser.parse("hahana"));
    assert_eq!(Err("nanana"), parser.parse("nanana"));
    assert_eq!(Err("nahaha"), parser.parse("nahaha"));
}

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), (),])), parser.parse("hahaha"));
    assert_eq!(Ok(("na", vec![(), (),])), parser.parse("hahana"));
    assert_eq!(Ok(("nanana", vec![])), parser.parse("nanana"));
    assert_eq!(Ok(("nahaha", vec![])), parser.parse("nahaha"));
}

#[test]
fn pair_combinator() {
    let start_tag = match_literal("<");
    let tag_opener = pair(start_tag, identifier);
    assert_eq!(
        Ok(("/>", ((), "starting-tag".to_string()))),
        tag_opener.parse("<starting-tag/>")
    );
    assert_eq!(Err("oops"), tag_opener.parse("oops"));
}

#[test]
fn right_combinator() {
    let tag_opener = right(match_literal("<"), identifier);
    assert_eq!(
        Ok(("/>", "starting-tag".to_string())),
        tag_opener.parse("<starting-tag/>")
    );
}

#[test]
fn literal_parser() {
    let parse_literal = match_literal("hello");
    assert_eq!(Ok(("", ())), parse_literal.parse("hello"));
    assert_eq!(Ok((" world", ())), parse_literal.parse("hello world"));
    assert_eq!(Err("world"), parse_literal.parse("world"));
}

#[test]
fn identifier_praser() {
    assert_eq!(
        Ok(("", "m1x3d-num3r1c-id-12".to_string())),
        identifier("m1x3d-num3r1c-id-12")
    );
    assert_eq!(
        Ok(("_<>", "m1x3d-num3r1c-id-12".to_string())),
        identifier("m1x3d-num3r1c-id-12_<>")
    );
    assert_eq!(Err("_it_wont_parse"), identifier("_it_wont_parse"));
}

#[test]
fn sample_xml_parser() {
    let sample_xml = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;

    let parsed_result = Element {
        name: "top".to_string(),
        attributes: vec![("label".to_string(), "Top".to_string())],
        children: vec![
            Element {
                name: "semi-bottom".to_string(),
                attributes: vec![("label".to_string(), "Bottom".to_string())],
                children: Default::default(),
            },
            Element {
                name: "middle".to_string(),
                attributes: Default::default(),
                children: vec![Element {
                    name: "bottom".to_string(),
                    attributes: vec![("label".to_string(), "Another bottom".to_string())],
                    children: Default::default(),
                }],
            },
        ],
    };

    assert_eq!(Ok(("", parsed_result)), element().parse(sample_xml));
}

#[test]
fn mismatched_closing_tag() {
    let sampled_xml = r#"
        <top>
            <bottom/>
            </middle>"#;

    let parsed_result: ParseResult<Element> = Err("</middle>");

    assert_eq!(parsed_result, element().parse(sampled_xml));
}

fn main() {
    let parser = match_literal("hello");
    let res = parser.parse("hello world!");
    println!("{:?}", res);

    let res = parser.parse("bye bye");
    println!("{:?}", res);
}
