use brainfuck::Interpreter;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(format!("Usage: {} hoge.bf", &args[0]));
    }
    let program = std::fs::read(&args[1]).map_err(|err| err.to_string())?;
    Interpreter::new().interpret(&program, &mut std::io::stdin(), &mut std::io::stdout())?;
    Ok(())
}
