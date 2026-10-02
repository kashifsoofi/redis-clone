use super::resp::Resp;

use super::parse_error::ParseError;
use super::parse_error::ParseResult;

pub fn exract_line(buffer: &[u8], start_index: usize) -> ParseResult<(Vec<u8>, usize)> {
    let mut index: usize = start_index;

    if index >= buffer.len() {
        return Err(ParseError::OutOfBounds(index));
    }

    let mut separator_found: bool = false;

    let mut previous_element: u8 = buffer[index];
    index += 1;
    for &element in buffer[index..].iter() {
        index += 1;

        if previous_element == b'\r' && element == b'\n' {
            separator_found = true;
            break;
        }

        previous_element = element;
    }

    if !separator_found {
        return Err(ParseError::OutOfBounds(index));
    }

    Ok((Vec::from(&buffer[start_index..(index - 2)]), index))
}

pub fn extract_line_as_string(buffer: &[u8], start_index: usize) -> ParseResult<(String, usize)> {
    let (line, index) = exract_line(buffer, start_index)?;
    let line_as_string = String::from_utf8(line)?;
    Ok((line_as_string, index))
}

pub fn parse_simple_string(buffer: &[u8], start_index: usize) -> ParseResult<(Resp, usize)> {
    if buffer[start_index] == b'+' {
        let (line, index) = extract_line_as_string(buffer, start_index + 1)?;

        return Ok((Resp::SimpleString(line), index));
    }

    return Err(ParseError::UnsupportedType(buffer[start_index]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_line_ok() {
        // arrange
        let buffer = b"OK\r\n";
        let index = 0;

        // act
        let (output, index) = exract_line(buffer, index).unwrap();

        // assert
        assert_eq!(output, b"OK");
        assert_eq!(index, 4);
    }

    #[test]
    fn test_extract_line_echo() {
        // arrange
        let buffer = b"ECHO\r\n";
        let index = 0;

        // act
        let (output, index) = exract_line(buffer, index).unwrap();

        // assert
        assert_eq!(output, b"ECHO");
        assert_eq!(index, 6);
    }

    #[test]
    fn test_extract_line_empty() {
        // arrange
        let buffer = b"";
        let index = 0;

        // act
        match exract_line(buffer, index) {
            Err(ParseError::OutOfBounds(index)) => {
                assert_eq!(index, 0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_extract_line_no_separator() {
        // arrange
        let buffer = b"OK";
        let index = 0;

        // act
        match exract_line(buffer, index) {
            Err(ParseError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_extract_line_index_too_advnced() {
        // arrange
        let buffer = b"OK";
        let index = 1;

        // act
        match exract_line(buffer, index) {
            Err(ParseError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_extract_line_index_half_separator() {
        // arrange
        let buffer = b"OK\r";
        let index = 0;

        // act
        match exract_line(buffer, index) {
            Err(ParseError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_extract_line_index_incorrect_separator() {
        // arrange
        let buffer = b"OK\n";
        let index = 0;

        // act
        match exract_line(buffer, index) {
            Err(ParseError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn text_extract_line_as_string() {
        // arrange
        let buffer = b"OK\r\n";
        let index = 0;

        // act
        let (output, index) = extract_line_as_string(buffer, index).unwrap();

        // assert
        assert_eq!(output, "OK");
        assert_eq!(index, 4);
    }

    #[test]
    fn text_extract_line_as_string_invalid_utf8() {
        // arrange
        let buffer: Vec<u8> = vec![0xFF, 0xFE, b'\r', b'\n'];
        let index = 0;

        // act
        let error = extract_line_as_string(&buffer, index).unwrap_err();

        // assert
        assert_eq!(error, ParseError::FromUtf8);
    }

    #[test]
    fn test_parse_simple_string() {
        // arrange
        let buffer = b"+OK\r\n";
        let index = 0;

        // act
        let (output, index) = parse_simple_string(buffer, index).unwrap();

        // assert
        assert_eq!(output, Resp::simple_string("OK"));
        assert_eq!(index, 5);
    }
}
