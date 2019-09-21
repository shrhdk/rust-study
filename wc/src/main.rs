use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result, stdin};
use std::path::Path;

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

fn count_files_and_print<P: AsRef<Path>>(paths: &[P]) {
    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut total_bytes: usize = 0;

    for path in paths {
        let path = path.as_ref();
        match count_file(&path) {
            Ok((lines, words, bytes)) => {
                total_lines += lines;
                total_words += words;
                total_bytes += bytes;
                println!("{: >8}{: >8} {: >8} {}", lines, words, bytes, path.display());
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    println!(
        "{: >8}{: >8} {: >8} total",
        total_lines, total_words, total_bytes
    );
}

fn count_file<P: AsRef<Path>>(path: P) -> Result<(usize, usize, usize)> {
    let file = File::open(path.as_ref())?;
    count(file)
}

fn count<R: Read>(reader: R) -> Result<(usize, usize, usize)> {
    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut count_reader = CountReader::new(reader);
    let buf_reader = BufReader::new(&mut count_reader);
    for line in buf_reader.lines() {
        let mut prev_c = ' ';
        for c in line?.chars() {
            if !c.is_whitespace() & prev_c.is_whitespace() {
                words += 1;
            }
            prev_c = c
        }
        lines += 1;
    }
    Ok((lines, words, count_reader.bytes))
}

struct CountReader<R: Read> {
    inner: R,
    bytes: usize,
}

impl<R: Read> CountReader<R> {
    fn new(inner: R) -> Self {
        CountReader { inner, bytes: 0 }
    }
}

impl<R: Read> Read for CountReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.inner.read(buf);
        if let Ok(n) = result {
            self.bytes += n;
        }
        result
    }
}

#[test]
fn test_count() {
    assert_eq!(count("hello world".as_bytes()).unwrap(), (1, 2, 11));
    assert_eq!(count("a b c".as_bytes()).unwrap(), (1, 3, 5));
    assert_eq!(count(" a b c ".as_bytes()).unwrap(), (1, 3, 7));
    assert_eq!(count("a\nb\nc".as_bytes()).unwrap(), (3, 3, 5));
    assert_eq!(count("\na\nb\nc\n".as_bytes()).unwrap(), (4, 3, 7));
}
