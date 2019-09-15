use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Result};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        count_and_print(&mut stdin());
    } else {
        count_files_and_print(&args);
    }
}

fn count_and_print<R: Read>(reader: &mut R) {
    match count(reader) {
        Ok((lines, words, bytes)) => {
            println!("{: >8}{: >8} {: >8}", lines, words, bytes);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn count_files_and_print<S: AsRef<str>>(file_names: &[S]) {
    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut total_bytes: usize = 0;

    for file_name in file_names {
        let file_name = file_name.as_ref();
        match File::open(file_name) {
            Ok(file) => match count(&file) {
                Ok((lines, words, bytes)) => {
                    total_lines += lines;
                    total_words += words;
                    total_bytes += bytes;
                    println!("{: >8}{: >8} {: >8} {}", lines, words, bytes, file_name);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            Err(e) => {
                eprintln!("{}: {}", file_name, e);
            }
        }
    }

    println!(
        "{: >8}{: >8} {: >8} total",
        total_lines, total_words, total_bytes
    );
}

fn count<R: Read>(reader: R) -> Result<(usize, usize, usize)> {
    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut count_reader = CountReader::new(reader);
    let buf_reader = BufReader::new(&mut count_reader);
    for line in buf_reader.lines() {
        let line = line?;
        lines += 1;
        let mut last_is_whitespace = true;
        for c in line.chars() {
            if !c.is_whitespace() & last_is_whitespace {
                words += 1;
            }
            last_is_whitespace = c.is_whitespace()
        }
    }
    Ok((lines, words, count_reader.n))
}

struct CountReader<R: Read> {
    inner: R,
    n: usize,
}

impl<R: Read> CountReader<R> {
    fn new(inner: R) -> Self {
        CountReader { inner, n: 0 }
    }
}

impl<R: Read> Read for CountReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.inner.read(buf);
        if let Ok(n) = result {
            self.n += n;
        }
        result
    }
}

#[test]
fn test_count() {
    assert_eq!(
        count("hello world".as_bytes()).unwrap(),
        Length {
            lines: 1,
            words: 2,
            bytes: 11,
        }
    );
    assert_eq!(
        count("a b c".as_bytes()).unwrap(),
        Length {
            lines: 1,
            words: 3,
            bytes: 5,
        }
    );
    assert_eq!(
        count(" a b c ".as_bytes()).unwrap(),
        Length {
            lines: 1,
            words: 3,
            bytes: 7,
        }
    );
    assert_eq!(
        count("a\nb\nc".as_bytes()).unwrap(),
        Length {
            lines: 3,
            words: 3,
            bytes: 5,
        }
    );
    assert_eq!(
        count("\na\nb\nc\n".as_bytes()).unwrap(),
        Length {
            lines: 4,
            words: 3,
            bytes: 7,
        }
    );
}
