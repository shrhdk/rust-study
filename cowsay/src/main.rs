extern crate unicode_width;

use std::io::{Read, Result};

use unicode_width::UnicodeWidthStr;

fn main() -> Result<()> {
    let text = read_text()?;
    print_callout(&text);
    print_cow();
    Ok(())
}

fn read_text() -> Result<String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        let mut buf = String::new();
        std::io::stdin().lock().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        Ok(args[0].clone())
    }
}

fn print_callout(text: &str) {
    let width = UnicodeWidthStr::width(text);
    println!("  {:_<1$}", "", width);
    println!("< {} >", text);
    println!("  {:-<1$}", "", width);
}

fn print_cow() {
    println!(
        r"         \   ^__^
          \  (oo)\_______
             (__)\       )\/\
                 ||----w |
                 ||     ||"
    );
}
