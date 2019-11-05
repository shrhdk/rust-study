extern crate utf8reader;

use std::collections::HashMap;
use std::io::Result;
use std::io::{Error, ErrorKind, Read};
use std::str::FromStr;

use utf8reader::UTF8Reader;

#[derive(PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

pub fn parse<R: Read>(reader: R) -> Result<Json> {
    let mut json_reader = JsonReader::new(reader);
    json_reader.skip_white_spaces()?;
    let value = json_reader.read_value()?;
    json_reader.skip_white_spaces()?;
    if let Some(ch) = json_reader.read_char()? {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!("unexpected char '{}', want EOF", ch),
        ))
    } else {
        Ok(value)
    }
}

pub fn parse_str(s: &str) -> Result<Json> {
    parse(s.as_bytes())
}

struct JsonReader<R: Read> {
    inner: UTF8Reader<R>,
    peeked_ch: Option<char>,
}

impl<R: Read> JsonReader<R> {
    fn new(inner: R) -> JsonReader<R> {
        let inner = UTF8Reader::new(inner);
        JsonReader {
            inner,
            peeked_ch: None,
        }
    }

    fn read_value(&mut self) -> Result<Json> {
        match self.peek_char()? {
            Some(ch) => match ch {
                '"' => self.read_string(),
                '-' => self.read_number(),
                ch if ch.is_ascii_digit() => self.read_number(),
                '{' => self.read_object(),
                '[' => self.read_array(),
                't' => self.read_true(),
                'f' => self.read_false(),
                'n' => self.read_null(),
                _ => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unexpected char {}", ch),
                )),
            },
            None => Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF")),
        }
    }

    fn read_string(&mut self) -> Result<Json> {
        let mut s = String::new();
        self.skip("\"")?;
        while let Some(ch) = self.read_char()? {
            match ch {
                '"' => return Ok(Json::String(s)),
                '\\' => s.push(self.read_escaped_char()?),
                _ => s.push(ch),
            }
        }
        Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF"))
    }

    fn read_number(&mut self) -> Result<Json> {
        let mut s = String::new();

        // read negative sign or first char of mantissa
        match self.read_char()? {
            Some('-') => s.push('-'),
            Some(ch) if ch.is_ascii_digit() => s.push(ch),
            Some(ch) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unexpected char '{}', want '-' or digit", ch),
                ));
            }
            None => return Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF")),
        }

        // read mantissa
        loop {
            match self.peek_char()? {
                Some(ch) if ch.is_ascii_digit() => {
                    self.read_char()?;
                    s.push(ch)
                }
                Some('e') | Some('E') => {
                    self.read_char()?;
                    s.push('e');
                    break;
                }
                _ => match f64::from_str(&s) {
                    Ok(number) => return Ok(Json::Number(number)),
                    Err(_) => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("invalid format number {}", &s),
                        ));
                    }
                },
            }
        }

        // read exponent sign
        let exponent_sign = self.read_char()?;
        match exponent_sign {
            Some('-') | Some('+') => s.push(exponent_sign.unwrap()),
            Some(ch) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unexpected char '{}', want '-' or '+'", ch),
                ));
            }
            None => return Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF")),
        }

        // read exponent digits
        loop {
            match self.peek_char()? {
                Some(ch) if ch.is_ascii_digit() => {
                    self.read_char()?;
                    s.push(ch);
                }
                _ => {
                    break;
                }
            }
        }

        match f64::from_str(&s) {
            Ok(number) => Ok(Json::Number(number)),
            Err(_) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("invalid format number {}", &s),
            )),
        }
    }

    fn read_object(&mut self) -> Result<Json> {
        let mut obj = HashMap::<String, Json>::new();
        self.skip("{")?;
        loop {
            self.skip_white_spaces()?;
            let key = if let Json::String(key) = self.read_string()? {
                key
            } else {
                unreachable!()
            };
            self.skip_white_spaces()?;
            self.skip(":")?;
            self.skip_white_spaces()?;
            let value = self.read_value()?;
            self.skip_white_spaces()?;
            obj.insert(key, value);
            match self.read_char()? {
                Some(',') => continue,
                Some('}') => break,
                Some(ch) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("unexpected char '{}'", ch),
                    ));
                }
                None => return Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF")),
            }
        }
        Ok(Json::Object(Box::new(obj)))
    }

    fn read_array(&mut self) -> Result<Json> {
        let mut array = Vec::<Json>::new();
        self.skip("[")?;
        loop {
            self.skip_white_spaces()?;
            array.push(self.read_value()?);
            self.skip_white_spaces()?;
            match self.read_char()? {
                Some(',') => continue,
                Some(']') => break,
                Some(ch) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("unexpected char '{}'", ch),
                    ));
                }
                None => return Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF")),
            }
        }
        Ok(Json::Array(array))
    }

    fn read_true(&mut self) -> Result<Json> {
        self.skip("true")?;
        Ok(Json::Boolean(true))
    }

    fn read_false(&mut self) -> Result<Json> {
        self.skip("false")?;
        Ok(Json::Boolean(false))
    }

    fn read_null(&mut self) -> Result<Json> {
        self.skip("null")?;
        Ok(Json::Null)
    }

    fn read_escaped_char(&mut self) -> Result<char> {
        match self.read_char()? {
            Some('"') => Ok('"'),
            Some('\\') => Ok('\\'),
            Some('/') => Ok('/'),
            Some('b') => Ok('\x08'),
            Some('f') => Ok('\x0C'),
            Some('n') => Ok('\x0A'),
            Some('r') => Ok('\x0D'),
            Some('t') => Ok('\x09'),
            Some('u') => self.read_codepoint(),
            Some(ch) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("unexpected char {}", ch),
            )),
            None => Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF")),
        }
    }

    fn read_codepoint(&mut self) -> Result<char> {
        // TODO(shiro): Support surrogate pair
        let mut digits = String::new();
        for _ in 0..4 {
            if let Some(ch) = self.read_char()? {
                if !ch.is_digit(16) {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("unexpected char '{}', want digit", ch),
                    ));
                }
                digits.push(ch)
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF"));
            }
        }
        let codepoint = u32::from_str_radix(&digits, 16).unwrap();
        if let Some(ch) = std::char::from_u32(codepoint) {
            Ok(ch)
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Bad codepoint {}", codepoint),
            ))
        }
    }

    fn skip_white_spaces(&mut self) -> Result<()> {
        while let Some(ch) = self.peek_char()? {
            if ch == '\x20' || ch == '\x09' || ch == '\x0A' || ch == '\x0D' {
                self.read_char()?;
            } else {
                return Ok(());
            }
        }
        Ok(())
    }

    fn skip(&mut self, s: &str) -> Result<()> {
        for want_ch in s.chars() {
            let got_ch = self.read_char()?.unwrap();
            if got_ch != want_ch {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unexpected char '{}', want '{}'", got_ch, want_ch),
                ));
            }
        }
        Ok(())
    }

    fn read_char(&mut self) -> Result<Option<char>> {
        if self.peeked_ch.is_some() {
            let peeked_ch = self.peeked_ch;
            self.peeked_ch = None;
            Ok(peeked_ch)
        } else {
            self.inner.getc()
        }
    }

    fn peek_char(&mut self) -> Result<Option<char>> {
        if self.peeked_ch.is_none() {
            self.peeked_ch = self.inner.getc()?;
        }
        Ok(self.peeked_ch)
    }
}
