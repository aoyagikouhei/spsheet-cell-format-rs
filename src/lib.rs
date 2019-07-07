use nom::{
    IResult,
    branch::*,
    bytes::complete::*,
    character::complete::*,
    multi::*,
    sequence::delimited,
};

#[derive(Debug,PartialEq)]
pub enum FormatItem {
    Standard(String),
    Quoted(String),
    Single(char),
    Exclamation(char),
}

#[allow(dead_code)]
fn standard(input: &str) -> IResult<&str, FormatItem> {
    let (input, target) = alt((tag("General"), tag("G/標準"), tag("@")))(input)?;
    Ok((input, FormatItem::Standard(target.to_string())))
}

#[allow(dead_code)]
fn quoted(input: &str) -> IResult<&str, FormatItem> {
    let (input, target) = delimited(char('"'), is_not("\""), char('"'))(input)?;
    Ok((input, FormatItem::Quoted(target.to_string())))
}

#[allow(dead_code)]
fn exclamation(input: &str) -> IResult<&str, FormatItem> {
    let (input, _) = char('!')(input)?;
    let (input, target) = take(1usize)(input)?;
    Ok((input, FormatItem::Exclamation(target.as_bytes()[0] as char)))
}

#[allow(dead_code)]
fn single(input: &str) -> IResult<&str, FormatItem> {
    let (input, target) = alt((
        char('¥'),
        char('$'),
        char('+'),
        char('('),
        char(':'),
        char('^'),
        char('\''),
        char('{'),
        char('<'),
        char('='),
        char('-'),
        char('/'),
        char(')'),
        char('&'),
        char('~'),
        char('}'),
        char('>'),
        char(' '),
    ))(input)?;
    Ok((input, FormatItem::Single(target)))
}

#[allow(dead_code)]
fn formats(input: &str) -> IResult<&str, Vec<FormatItem>> {
    many1(alt((exclamation, standard, quoted, single)))(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let at = crate::FormatItem::Standard("@".to_string());
        let xyz = crate::FormatItem::Quoted("xyz".to_string());
        assert_eq!(crate::quoted("\"xyz\"abc"), Ok(("abc", xyz)));
        assert_eq!(crate::standard("@aaa"), Ok(("aaa", at)));
        let res = vec![
            crate::FormatItem::Single('+'),
            crate::FormatItem::Quoted("xyz".to_string()),
            crate::FormatItem::Exclamation('@'),
            crate::FormatItem::Standard("@".to_string()),
            crate::FormatItem::Quoted("abc".to_string()),
        ];
        assert_eq!(crate::formats("+\"xyz\"!@@\"abc\""), Ok(("", res)));
    }
}
