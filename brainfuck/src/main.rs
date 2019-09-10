use std::io::Read;

use brainfuck::Interpreter;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(format!("Usage: {} hoge.bf", &args[0]));
    }
    let file_name = &args[1];
    let program = read_all(file_name).map_err(|err| err.to_string())?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program, &mut std::io::stdin(), &mut std::io::stdout())?;
    Ok(())
}

fn read_all(file_name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(file_name)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    Ok(buf)
}
