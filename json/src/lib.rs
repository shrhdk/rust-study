extern crate utf8reader;

use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read};
use std::io::Result;
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
        Err(json_reader.unexpected_char_error("EOF", ch))
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
                _ => Err(self.unexpected_char_error("'\"', '-', '{', '[', 't', 'f' or 'n'", ch)),
            },
            None => Err(self.unexpected_eof_error()),
        }
    }

    fn read_string(&mut self) -> Result<Json> {
        let mut s = String::new();
        self.skip_str("\"")?;
        while let Some(ch) = self.read_char()? {
            match ch {
                '"' => return Ok(Json::String(s)),
                '\\' => s.push(self.read_escaped_char()?),
                _ => s.push(ch),
            }
        }
        Err(self.unexpected_eof_error())
    }

    fn read_number(&mut self) -> Result<Json> {
        let mut s = String::new();

        // read sign
        if let Some('-') = self.peek_char()? {
            self.skip_str("-")?;
            s.push('-');
        }

        // read integer part and fraction part
        match self.peek_char()? {
            Some('0') => {
                s.push(self.read_char()?.unwrap());
                if let Some('.') = self.peek_char()? {
                    s.push(self.read_char()?.unwrap());
                    self.read_digits(&mut s)?;
                }
            }
            Some(ch) if ch.is_ascii_digit() => {
                self.read_digits(&mut s)?;
                if let Some('.') = self.peek_char()? {
                    self.skip_str(".")?;
                    s.push('.');
                    self.read_digits(&mut s)?;
                }
            }
            Some(ch) => return Err(self.unexpected_char_error("digit", ch)),
            None => return Err(self.unexpected_eof_error()),
        }

        // read exponent part
        match self.peek_char()? {
            Some('e') | Some('E') => {
                self.read_char()?; // skip e or E
                s.push('e');
                match self.peek_char()? {
                    Some('-') | Some('+') => s.push(self.read_char()?.unwrap()),
                    Some(ch) => return Err(self.unexpected_char_error("'-' or '+'", ch)),
                    None => return Err(self.unexpected_eof_error()),
                }
                self.read_digits(&mut s)?;
            }
            _ => { /* No Exponent Part */ }
        }

        match f64::from_str(&s) {
            Ok(number) => Ok(Json::Number(number)),
            Err(_) => Err(self.error(&format!("invalid format number {}", &s))),
        }
    }

    fn read_object(&mut self) -> Result<Json> {
        let mut obj = HashMap::<String, Json>::new();
        self.skip_str("{")?;
        loop {
            self.skip_white_spaces()?;
            let key = if let Json::String(key) = self.read_string()? {
                key
            } else {
                unreachable!()
            };
            self.skip_white_spaces()?;
            self.skip_str(":")?;
            self.skip_white_spaces()?;
            let value = self.read_value()?;
            self.skip_white_spaces()?;
            obj.insert(key, value);
            match self.read_char()? {
                Some(',') => continue,
                Some('}') => break,
                Some(ch) => return Err(self.unexpected_char_error("',' or '}'", ch)),
                None => return Err(self.unexpected_eof_error()),
            }
        }
        Ok(Json::Object(Box::new(obj)))
    }

    fn read_array(&mut self) -> Result<Json> {
        let mut array = Vec::<Json>::new();
        self.skip_str("[")?;
        loop {
            self.skip_white_spaces()?;
            array.push(self.read_value()?);
            self.skip_white_spaces()?;
            match self.read_char()? {
                Some(',') => continue,
                Some(']') => break,
                Some(ch) => return Err(self.unexpected_char_error("',' or ']'", ch)),
                None => return Err(self.unexpected_eof_error()),
            }
        }
        Ok(Json::Array(array))
    }

    fn read_true(&mut self) -> Result<Json> {
        self.skip_str("true")?;
        Ok(Json::Boolean(true))
    }

    fn read_false(&mut self) -> Result<Json> {
        self.skip_str("false")?;
        Ok(Json::Boolean(false))
    }

    fn read_null(&mut self) -> Result<Json> {
        self.skip_str("null")?;
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
            Some('u') => self.read_code_point(),
            Some(ch) => Err(self.unexpected_char_error("'\"', '\\', '/', 'b', 'f', 'n', 'r', 't' or 'u'", ch)),
            None => Err(self.unexpected_eof_error()),
        }
    }

    fn read_code_point(&mut self) -> Result<char> {
        let high = self.read_code_point_digits()?;
        let code_point = if 0xD800 <= high && high <= 0xDBFF {
            self.skip_str("\\u")?;
            let lower = self.read_code_point_digits()?;
            if lower < 0xDC00 || 0xDFFF < lower {
                return Err(self.error(&format!("want lower surrogate, got U+{:04}", lower)));
            }
            0x10000 + (high - 0xD800) * 0x400 + (lower - 0xDC00)
        } else {
            high
        };

        if let Some(ch) = std::char::from_u32(code_point) {
            Ok(ch)
        } else {
            Err(self.error(&format!("bad code point U+{:04}", code_point)))
        }
    }

    fn read_code_point_digits(&mut self) -> Result<u32> {
        let mut digits = String::new();
        for _ in 0..4 {
            if let Some(ch) = self.read_char()? {
                if !ch.is_digit(16) {
                    return Err(self.unexpected_char_error("digit", ch));
                }
                digits.push(ch)
            } else {
                return Err(self.unexpected_eof_error());
            }
        }
        u32::from_str_radix(&digits, 16).map_err(|err| self.error(&err.to_string()))
    }

    fn read_digits(&mut self, s: &mut String) -> Result<()> {
        match self.read_char()? {
            Some(ch) if ch.is_ascii_digit() => s.push(ch),
            Some(ch) => return Err(self.unexpected_char_error("digit", ch)),
            None => return Err(self.unexpected_eof_error()),
        }
        loop {
            match self.peek_char()? {
                Some(ch) if ch.is_ascii_digit() => s.push(self.read_char()?.unwrap()),
                _ => return Ok(()),
            }
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

    fn skip_str(&mut self, s: &str) -> Result<()> {
        for want_ch in s.chars() {
            match self.read_char()? {
                Some(ch) if ch != want_ch => return Err(self.unexpected_char_error(&format!("'{}'", want_ch), ch)),
                None => return Err(self.unexpected_eof_error()),
                _ => { /* OK */ }
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

    fn error(&self, message: &str) -> Error {
        Error::new(ErrorKind::InvalidInput, message)
    }

    fn unexpected_char_error(&self, want: &str, got: char) -> Error {
        Error::new(
            ErrorKind::InvalidInput,
            format!("unexpected char '{}', want '{}'", got, want),
        )
    }

    fn unexpected_eof_error(&self) -> Error {
        Error::new(ErrorKind::InvalidInput, "unexpected EOF")
    }
}
