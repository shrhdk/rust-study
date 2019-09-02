use std::io::{Read, Write};

pub fn interpret<R: Read, W: Write>(program: &[u8], input: &mut R, output: &mut W) -> Result<(), String> {
    let mut pc: usize = 0;
    let mut ptr: usize = 0;
    let mut data: Vec<u8> = vec![0; 30000];
    while pc < program.len() {
        match program[pc] {
            b'>' => ptr += 1,
            b'<' => {
                if ptr == 0 {
                    return Err("bad pointer".to_string());
                }
                ptr -= 1;
            }
            b'+' => data[ptr] = data[ptr].wrapping_add(1),
            b'-' => data[ptr] = data[ptr].wrapping_sub(1),
            b'.' => {
                output.write_all(&[data[ptr]])
                    .map_err(|err| err.to_string())?;
            }
            b',' => {
                input.read_exact(&mut data[ptr..=ptr])
                    .map_err(|err| err.to_string())?;
            }
            b'[' if data[ptr] == 0 => {
                let mut n = 1;
                while n > 0 {
                    pc += 1;
                    if pc == program.len() {
                        return Err("missing close brackets".to_string());
                    }
                    match program[pc] {
                        b'[' => n += 1,
                        b']' => n -= 1,
                        _ => { /* Ignore */ }
                    }
                }
            }
            b']' if data[ptr] != 0 => {
                let mut n = 1;
                while n > 0 {
                    if pc == 0 {
                        return Err("missing open brackets".to_string());
                    }
                    pc -= 1;
                    match program[pc] {
                        b']' => n += 1,
                        b'[' => n -= 1,
                        _ => { /* Ignore */ }
                    }
                }
            }
            _ => { /* NOP */ }
        }
        pc += 1;
    }
    Ok(())
}

#[test]
fn test_bad_ptr() {
    let program = b"<";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();
    let result = interpret(program, &mut &input[..], &mut output);
    assert_eq!(result, Err("bad pointer".to_string()));
}

#[test]
fn test_missing_open_brackets() {
    let program = b"+]";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();
    let result = interpret(program, &mut &input[..], &mut output);
    assert_eq!(result, Err("missing open brackets".to_string()));
}

#[test]
fn test_missing_close_brackets() {
    let program = b"[";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();
    let result = interpret(program, &mut &input[..], &mut output);
    assert_eq!(result, Err("missing close brackets".to_string()));
}

#[test]
fn test_interpret_hello_world() {
    let program = b">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>++++++++[<++++>-]<+.[-]++++++++++.";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();
    interpret(program, &mut &input[..], &mut output).expect("fail to interpret");
    assert_eq!(output, b"Hello World!\n");
}

#[test]
fn test_interpret_echo() {
    let program = b",>,>,<<.>.>.";
    let input = b"foo";
    let mut output = Vec::<u8>::new();
    interpret(program, &mut &input[..], &mut output).expect("fail to interpret");
    assert_eq!(output, b"foo");
}

#[test]
fn test_multiple() {
    let program = b",>,<[>[>+>+<<-]>>[<<+>>-]<<<-]>>.";
    let input = &[3, 7];
    let mut output = Vec::<u8>::new();
    interpret(program, &mut &input[..], &mut output).expect("fail to interpret");
    assert_eq!(output, &[21]);
}
