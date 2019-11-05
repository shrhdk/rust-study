extern crate utf8reader;

use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read};
use std::io::Result;

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
        Err(Error::new(ErrorKind::InvalidInput, format!("unexpected char '{}', want EOF", ch)))
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
        JsonReader { inner, peeked_ch: None }
    }

    fn read_value(&mut self) -> Result<Json> {
        match self.peek_char()? {
            Some(ch) => {
                match ch {
                    '"' => self.read_string(),
                    '-' | _ if ch.is_ascii_digit() => self.read_number(),
                    '{' => self.read_object(),
                    '[' => self.read_array(),
                    't' => self.read_true(),
                    'f' => self.read_false(),
                    'n' => self.read_null(),
                    _ => Err(Error::new(ErrorKind::InvalidInput, format!("unexpected char {}", ch)))
                }
            }
            None => Err(Error::new(ErrorKind::InvalidInput, "unexpected EOF"))
        }
    }

    fn read_string(&mut self) -> Result<Json> {
        // TODO(shiro): Support escaped char.
        self.skip("\"")?;
        let mut v = Vec::<char>::new();
        while let Some(ch) = self.read_char()? {
            if ch == '"' {
                break;
            } else {
                v.push(ch);
            }
        }
        Ok(Json::String(v.iter().collect()))
    }

    fn read_number(&mut self) -> Result<Json> {
        unimplemented!()
    }

    fn read_object(&mut self) -> Result<Json> {
        unimplemented!()
    }

    fn read_array(&mut self) -> Result<Json> {
        unimplemented!()
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

    fn read_escaped_char() -> Result<char> {
        unimplemented!()
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
                return Err(Error::new(ErrorKind::InvalidInput, format!("unexpected char '{}', want '{}'", got_ch, want_ch)));
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
