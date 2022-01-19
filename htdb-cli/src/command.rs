use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::error::convert_error;
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::Err as NomErr;
use nom::Parser;

#[derive(Debug)]
pub enum Command<'a> {
    Get {
        hash_key: &'a str,
        tree_key: &'a str,
    },
    Put {
        hash_key: &'a str,
        tree_key: &'a str,
        data: &'a str,
    },
    Contains {
        hash_key: &'a str,
        tree_key: &'a str,
    },
    Delete {
        hash_key: &'a str,
        tree_key: &'a str,
    },
    Range {
        hash_key: &'a str,
        tree_start: &'a str,
        tree_end: &'a str,
    },
    Succ {
        hash_key: &'a str,
        tree_key: &'a str,
    },
    Pred {
        hash_key: &'a str,
        tree_key: &'a str,
    },
    Count {},
    Show {},
    Save {},
    Load {},
    Exit {},
}

fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

fn parse_get<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("GET"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_key, _)| Command::Get { hash_key, tree_key },
    )
}

fn parse_put<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("PUT"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_key, _, data, _)| Command::Put {
            hash_key,
            tree_key,
            data,
        },
    )
}

fn parse_contains<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("CONTAINS"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_key, _)| Command::Contains { hash_key, tree_key },
    )
}

fn parse_delete<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("DELETE"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_key, _)| Command::Delete { hash_key, tree_key },
    )
}

fn parse_range<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("RANGE"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_start, _, tree_end, _)| Command::Range {
            hash_key,
            tree_start,
            tree_end,
        },
    )
}

fn parse_succ<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("SUCC"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_key, _)| Command::Succ { hash_key, tree_key },
    )
}

fn parse_pred<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            tag("PRED"),
            space1,
            take_till(is_whitespace),
            space1,
            take_till(is_whitespace),
            space0,
        )),
        |(_, _, hash_key, _, tree_key, _)| Command::Pred { hash_key, tree_key },
    )
}

fn parse_count<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(tuple((tag("COUNT"), space0)), |(_, _)| Command::Count {})
}

fn parse_show<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(tuple((tag("SHOW"), space0)), |(_, _)| Command::Show {})
}

fn parse_save<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(tuple((tag("SAVE"), space0)), |(_, _)| Command::Save {})
}

fn parse_load<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(tuple((tag("LOAD"), space0)), |(_, _)| Command::Load {})
}

fn parse_exit<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(tuple((tag("EXIT"), space0)), |(_, _)| Command::Exit {})
}

fn parser<'a, E>() -> impl Parser<&'a str, Command<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((
            space0,
            alt((
                parse_get(),
                parse_put(),
                parse_contains(),
                parse_delete(),
                parse_range(),
                parse_count(),
                parse_show(),
                parse_save(),
                parse_load(),
                parse_exit(),
            )),
            space0,
        )),
        |(_, command, _)| command,
    )
}

impl<'a> Command<'a> {
    pub fn parse(input: &str) -> Result<Command, String> {
        match parser().parse(input) {
            Ok((_tail, command)) => Ok(command),
            Err(NomErr::Error(error)) | Err(NomErr::Failure(error)) => {
                Err(convert_error(input, error))
            }
            Err(error) => Err(format!("{}", error)),
        }
    }
}
