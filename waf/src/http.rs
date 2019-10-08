use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::net::TcpStream;

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
    stream: TcpStream,
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Connection {
    pub fn new(mut stream: TcpStream) -> Result<Self> {
        let mut br = BufReader::new(&mut stream);

        let mut line = "".to_string();
        br.read_line(&mut line)?;
        let (method, path, version) = parse_request_line(&line)?;

        Ok(Connection {
            stream,
            method,
            path,
            version,
        })
    }

    pub fn read_headers(&mut self) -> Result<Headers> {
        let mut headers = Headers::new();
        let mut line = "".to_string();
        let mut br = BufReader::new(&mut self.stream);
        while let Ok(_) = br.read_line(&mut line) {
            let tokens: Vec<&str> = line.split(':').collect();
            if tokens.len() != 2 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid header {}", line),
                ));
            }
            headers.insert(tokens[0].to_string(), tokens[1].to_string());
        }
        Ok(headers)
    }

    pub fn write_status(&mut self, code: u16, reason: &str) -> Result<()> {
        let status_line = format!("{} {} {}\r\n", self.version, code, reason);
        self.stream.write_all(status_line.as_bytes())
    }

    pub fn write_header(&mut self, key: &str, value: &str) -> Result<()> {
        self.print(key)?;
        self.print(": ")?;
        self.println(value)
    }

    pub fn write_headers(&mut self, headers: &Headers) -> Result<()> {
        for (k, v) in headers {
            self.write_header(k, v)?;
        }
        Ok(())
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data)
    }

    pub fn print(&mut self, line: &str) -> Result<()> {
        self.write_all(line.as_bytes())
    }

    pub fn println(&mut self, line: &str) -> Result<()> {
        self.print(line)?;
        self.print("\r\n")
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
