use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramLiteral {
    //FloatLiteral(f32),
    IntLiteral(u32),
    StringLiteral(String),
}

impl ProgramSerialize for ProgramLiteral {
    fn add_bytes(self, buf: &mut ProgramBytes) {
        buf.1.push(0x1);
        match self {
            Self::IntLiteral(x) => {
                buf.1.push(0x0);
                buf.1.extend_from_slice(x.to_be_bytes().as_slice());
            }
            Self::StringLiteral(x) => {
                buf.1.push(0x1);
                buf.1.extend_from_slice(x.as_bytes());
            }
        }
    }
}

impl ProgramDeserialize for ProgramLiteral {
    fn from_bytes(buf: &mut Iter<u8>, i: &mut usize) -> Self {
        *i += 1;
        match *buf.next().unwrap() {
            0x0 => match next_u32!(buf, i) {
                4 => ProgramLiteral::IntLiteral(next_u32!(buf, i)),
                _x => panic!("only u32 supported currently, given length was {_x}"),
            },
            0x1 => {
                let mut string_maker = String::new();
                let len = next_u32!(buf, i);
                for _ in 0..len {
                    *i += 1;
                    string_maker.push((*buf.next().unwrap()) as char);
                }
                ProgramLiteral::StringLiteral(string_maker)
            }
            _ => panic!("invalid literal, make sure this is the correct runtime"),
        }
    }
}

pub fn parse_literal<'a, E>(input: &'a str) -> IResult<&'a str, ProgramLiteral, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    alt((
        map(parse_num, ProgramLiteral::IntLiteral),
        map(parse_string, ProgramLiteral::StringLiteral),
    ))(input)
}

fn parse_num<'a, E>(i: &'a str) -> IResult<&'a str, u32, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map_res(digit1, |digit_str: &str| digit_str.parse::<u32>()),
        map(preceded(tag("-"), digit1), |_digit_str: &str| {
            panic!("no negative numbers yet");
            //-_digit_str.parse::<u8>().unwrap()
        }),
    ))(i)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
    EscapedWS,
}

fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    let build_str = fold_many0(
        parse_literal_fragment,
        String::new,
        |mut string, fragment| {
            match fragment {
                StringFragment::Literal(s) => string.push_str(s),
                StringFragment::EscapedChar(c) => string.push(c),
                StringFragment::EscapedWS => {}
            }
            string
        },
    );
    delimited(char('`'), build_str, char('`'))(input)
}

fn parse_literal_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_string_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))(input)
}

fn parse_string_literal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let not_quote_slash = is_not("`\\");

    verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    preceded(
        char('\\'),
        alt((
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('`', char('`')),
        )),
    )(input)
}

fn parse_escaped_whitespace<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    preceded(char('\\'), multispace1)(input)
}
