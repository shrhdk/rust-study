use std::io::{Read, Write};

pub struct Interpreter {
    pos: usize,
    ptr: usize,
    data: Vec<u8>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret<R: Read, W: Write>(&mut self, program: &[u8], input: &mut R, output: &mut W) -> Result<(), String> {
        while self.pos < program.len() {
            match program[self.pos] {
                b'>' => {
                    self.ptr += 1;
                    if self.ptr == self.data.len() {
                        self.data.resize(self.data.len() * 2, 0);
                    }
                }
                b'<' => {
                    if self.ptr == 0 {
                        return Err("bad pointer".to_string());
                    }
                    self.ptr -= 1;
                }
                b'+' => self.data[self.ptr] = self.data[self.ptr].wrapping_add(1),
                b'-' => self.data[self.ptr] = self.data[self.ptr].wrapping_sub(1),
                b'.' => {
                    output.write_all(&[self.data[self.ptr]])
                        .map_err(|err| err.to_string())?;
                }
                b',' => {
                    input.read_exact(&mut self.data[self.ptr..=self.ptr])
                        .map_err(|err| err.to_string())?;
                }
                b'[' if self.data[self.ptr] == 0 => {
                    let mut n = 1;
                    while n > 0 {
                        self.pos += 1;
                        if self.pos == program.len() {
                            return Err("missing close brackets".to_string());
                        }
                        match program[self.pos] {
                            b'[' => n += 1,
                            b']' => n -= 1,
                            _ => { /* Ignore */ }
                        }
                    }
                }
                b']' if self.data[self.ptr] != 0 => {
                    let mut n = 1;
                    while n > 0 {
                        if self.pos == 0 {
                            return Err("missing open brackets".to_string());
                        }
                        self.pos -= 1;
                        match program[self.pos] {
                            b']' => n += 1,
                            b'[' => n -= 1,
                            _ => { /* Ignore */ }
                        }
                    }
                }
                _ => { /* NOP */ }
            }
            self.pos += 1;
        }
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        let pos: usize = 0;
        let ptr: usize = 0;
        let data: Vec<u8> = vec![0; 30000];
        Self { pos, ptr, data }
    }
}
