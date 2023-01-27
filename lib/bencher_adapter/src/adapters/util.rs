use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::fold_many1, IResult,
};
use ordered_float::OrderedFloat;

pub fn time_as_nanos<T>(time: T, units: Units) -> OrderedFloat<f64>
where
    T: Into<Time>,
{
    (time.into().as_f64() * units.as_nanos()).into()
}

#[derive(Clone, Copy)]
pub enum Time {
    UInt64(u64),
    Float64(f64),
}

impl From<u64> for Time {
    fn from(int: u64) -> Self {
        Self::UInt64(int)
    }
}

impl From<f64> for Time {
    fn from(float: f64) -> Self {
        Self::Float64(float)
    }
}

impl Time {
    fn as_f64(&self) -> f64 {
        match self {
            Self::UInt64(int) => *int as f64,
            Self::Float64(float) => *float,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Units {
    Pico,
    Nano,
    Micro,
    Milli,
    Sec,
}

impl Units {
    #[allow(clippy::float_arithmetic)]
    pub fn as_nanos(&self) -> f64 {
        match self {
            Self::Pico => 1.0 / 1_000.0,
            Self::Nano => 1.0,
            Self::Micro => 1_000.0,
            Self::Milli => 1_000_000.0,
            Self::Sec => 1_000_000_000.0,
        }
    }
}

pub fn parse_units(input: &str) -> IResult<&str, Units> {
    alt((
        map(tag("ps"), |_| Units::Pico),
        map(tag("ns"), |_| Units::Nano),
        map(tag("\u{3bc}s"), |_| Units::Micro),
        map(tag("\u{b5}s"), |_| Units::Micro),
        map(tag("ms"), |_| Units::Milli),
        map(tag("s"), |_| Units::Sec),
    ))(input)
}

pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    let (remainder, int) = parse_int(input)?;
    Ok((remainder, into_number(int)?))
}

pub fn parse_f64(input: &str) -> IResult<&str, f64> {
    let (remainder, float) = parse_float(input)?;
    Ok((remainder, into_number(float)?))
}

pub fn parse_int(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1(
        alt((digit1, tag(","))),
        Vec::new,
        |mut int_chars, int_char| {
            if int_char == "," {
                int_chars
            } else {
                int_chars.push(int_char);
                int_chars
            }
        },
    )(input)
}

pub fn parse_float(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1(
        alt((digit1, tag("."), tag(","))),
        Vec::new,
        |mut float_chars, float_char| {
            if float_char == "," {
                float_chars
            } else {
                float_chars.push(float_char);
                float_chars
            }
        },
    )(input)
}

pub fn into_number<T>(input: Vec<&str>) -> Result<T, nom::Err<nom::error::Error<&str>>>
where
    T: std::str::FromStr,
{
    let mut number = String::new();
    for digit in input {
        number.push_str(digit);
    }

    T::from_str(&number)
        .map_err(|_e| nom::Err::Error(nom::error::make_error("\0", nom::error::ErrorKind::Tag)))
}