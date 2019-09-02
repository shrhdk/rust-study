use brainfuck::*;

#[test]
fn test_interpret_hello_world() {
    let program = b">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>++++++++[<++++>-]<+.[-]++++++++++.";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(program, &mut &input[..], &mut output).expect("fail to interpret");

    assert_eq!(output, b"Hello World!\n");
}

#[test]
fn test_interpret_echo() {
    let program = b",>,>,<<.>.>.";
    let input = b"foo";
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(program, &mut &input[..], &mut output).expect("fail to interpret");

    assert_eq!(output, b"foo");
}

#[test]
fn test_multiple() {
    let program = b",>,<[>[>+>+<<-]>>[<<+>>-]<<<-]>>.";
    let input = &[3, 7];
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(program, &mut &input[..], &mut output).expect("fail to interpret");

    assert_eq!(output, &[21]);
}

#[test]
fn expand_data() {
    let mut program = vec![b'>'; 30000]; // initial data size is 30000
    program.push(b'+'); // increments address 30001
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program, &mut &input[..], &mut output);

    assert_eq!(result, Ok(()));
}

#[test]
fn test_ignore_addition_overflow() {
    let mut program = vec![b'+'; 256 + 1]; // 0 -> 1, 2, ..., 255, 0, 1
    program.push(b'.');
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program, &mut &input[..], &mut output).expect("fail to interpret");

    assert_eq!(output, &[1]);
}

#[test]
fn test_ignore_subtraction_overflow() {
    let program = b"-."; // 0 -> 255
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(program, &mut &input[..], &mut output).expect("fail to interpret");

    assert_eq!(output, &[255]);
}

#[test]
fn test_bad_ptr() {
    let program = b"<";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(program, &mut &input[..], &mut output);

    assert_eq!(result, Err("bad pointer".to_string()));
}

#[test]
fn test_missing_open_brackets() {
    let program = b"+]";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(program, &mut &input[..], &mut output);

    assert_eq!(result, Err("missing open brackets".to_string()));
}

#[test]
fn test_missing_close_brackets() {
    let program = b"[";
    let input = Vec::<u8>::new();
    let mut output = Vec::<u8>::new();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(program, &mut &input[..], &mut output);

    assert_eq!(result, Err("missing close brackets".to_string()));
}
