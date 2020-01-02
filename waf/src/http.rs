use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Result, Write};
use std::net::TcpStream;

use chrono::Utc;

pub type Headers = HashMap<String, String>;

pub const METHOD_GET: &str = "GET";
pub const METHOD_HEAD: &str = "HEAD";
pub const METHOD_POST: &str = "POST";
pub const METHOD_PUT: &str = "PUT";
pub const METHOD_DELETE: &str = "DELETE";
pub const METHOD_CONNECT: &str = "CONNECT";
pub const METHOD_OPTIONS: &str = "OPTIONS";
pub const METHOD_TRACE: &str = "TRACE";
pub const METHOD_PATCH: &str = "PATCH";

pub struct Connection {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Result<Self> {
        let mut reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream.try_clone()?);

        let mut line = "".to_string();
        reader.read_line(&mut line)?;
        let (method, path, version) = parse_request_line(&line)?;

        Ok(Connection {
            reader,
            writer,
            method,
            path,
            version,
        })
    }

    pub fn read_headers(&mut self) -> Result<HashMap<String, String>> {
        let mut headers = HashMap::new();
        while let Some((key, value)) = self.read_header()? {
            headers.insert(key, value);
        }
        Ok(headers)
    }

    pub fn read_header(&mut self) -> Result<Option<(String, String)>> {
        let mut line = "".to_string();
        self.reader.read_line(&mut line)?;
        if &line == "\r\n" || &line == "\n" {
            Ok(None)
        } else {
            Ok(Some(parse_header(&line)?))
        }
    }

    pub fn write_status(&mut self, code: u16, reason: &str) -> Result<()> {
        self.writer
            .write_fmt(format_args!("{} {} {}\r\n", self.version, code, reason))
    }

    pub fn write_headers(&mut self, headers: &mut HashMap<String, String>) -> Result<()> {
        for (key, value) in headers {
            self.write_header(key, value)?;
        }
        Ok(())
    }

    pub fn write_header(&mut self, key: &str, value: &str) -> Result<()> {
        self.writer
            .write_fmt(format_args!("{}: {}\r\n", key, value))
    }

    pub fn write_date_header(&mut self) -> Result<()> {
        let date_str = &Utc::now().format("%a, %d %m %Y %H:%M:%S GMT").to_string();
        self.write_header("Date", date_str)
    }

    pub fn finish_header(&mut self) -> Result<()> {
        self.writer.write_all("\r\n".as_bytes())
    }
}

impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl BufRead for Connection {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

fn parse_request_line(line: &str) -> Result<(String, String, String)> {
    let tokens: Vec<&str> = line.trim_end().split(' ').collect();
    if tokens.len() == 3 {
        Ok((
            tokens[0].to_string(),
            tokens[1].to_string(),
            tokens[2].to_string(),
        ))
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!("failed to parse {} as HTTP Request-Line", line),
        ))
    }
}

fn parse_header(line: &str) -> Result<(String, String)> {
    let key_value: Vec<&str> = line.splitn(2, ':').collect();
    if key_value.len() == 2 {
        let key = key_value[0].trim().to_string();
        let value = key_value[1].trim().to_string();
        Ok((key, value))
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid header {}", &line),
        ))
    }
}
