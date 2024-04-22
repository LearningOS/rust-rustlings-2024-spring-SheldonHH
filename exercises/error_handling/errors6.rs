use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError), // Assuming CreationError is defined elsewhere
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

// Assuming PositiveNonzeroInteger and CreationError are defined elsewhere
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(i64);

impl PositiveNonzeroInteger {
    // Assuming the new function is defined elsewhere and returns a Result
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value))
        } else {
            Err(CreationError)
        }
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x = s.parse::<i64>().map_err(ParsePosNonzeroError::from)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Example use of `CreationError` assuming it's an empty struct just for illustration
#[derive(Debug, PartialEq)]
struct CreationError;

fn main() {
    // This is just an example call to `parse_pos_nonzero` and might not be needed in your final code
    match parse_pos_nonzero("42") {
        Ok(num) => println!("Parsed number: {:?}", num),
        Err(e) => println!("Failed to parse number: {:?}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
