mod definition;
mod fragment;
mod instruction;
mod literal;
mod register;
mod section;

pub use nom::branch::alt;
pub use nom::bytes::complete::{is_not, tag, take_till};
pub use nom::character::complete::{digit1, multispace0, multispace1};
pub use nom::character::streaming::{alpha1, char};
pub use nom::combinator::{complete, eof, map, map_res, value, verify};
pub use nom::error::{FromExternalError, ParseError};
pub use nom::sequence::{delimited, preceded, terminated};
pub use nom::IResult;
pub use std::slice::Iter;

pub use definition::*;
pub use fragment::*;
pub use instruction::*;
pub use literal::*;
pub use nom::multi::fold_many0;
pub use register::*;
pub use section::*;

pub fn parse_program<'a, E>(input: &'a str) -> IResult<&'a str, Vec<ProgramFragment>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    fold_many0(parse_fragment::<E>, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(input)
}

pub trait ProgramSerialize {
    fn add_bytes(self, buf: &mut Vec<u8>);
}

pub trait ProgramDeserialize {
    fn from_bytes(buf: &mut Iter<u8>, i: &mut usize) -> Self;
}
