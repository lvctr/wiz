use crate::parser::wiz::lexical_structure::identifier;
use crate::syntax::name_space::{NameSpaceElementSyntax, NameSpaceSyntax};
use crate::syntax::token::TokenSyntax;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{AsChar, Compare, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::RangeFrom;

pub fn name_space<I>(s: I) -> IResult<I, NameSpaceSyntax>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(many0(name_space_element), |elements| NameSpaceSyntax {
        leading_trivia: Default::default(),
        elements,
        trailing_trivia: Default::default(),
    })(s)
}

pub fn name_space_element<I>(s: I) -> IResult<I, NameSpaceElementSyntax>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((identifier, tag("::"))), |(i, sep): (_, I)| {
        NameSpaceElementSyntax {
            name: TokenSyntax::from(i),
            separator: TokenSyntax::from(sep),
        }
    })(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::name_space::{name_space, name_space_element};
    use crate::syntax::name_space::{NameSpaceElementSyntax, NameSpaceSyntax};

    #[test]
    fn test_name_space_element() {
        assert_eq!(
            name_space_element("name::"),
            Ok(("", NameSpaceElementSyntax::from("name")))
        );
    }

    #[test]
    fn test_name_space() {
        assert_eq!(name_space(""), Ok(("", NameSpaceSyntax::default())));
        assert_eq!(
            name_space("a::b::"),
            Ok(("", NameSpaceSyntax::from(vec!["a", "b"])))
        );
    }
}
