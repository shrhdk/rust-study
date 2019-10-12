extern crate clap;

use std::{io, thread};
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::time::Duration;

use clap::{App, Arg};

fn main() -> io::Result<()> {
    let app = App::new("lifegame")
        .version("0.1.0")
        .author("Hideki Shiro <hideki@shiro.be>")
        .arg(
            Arg::with_name("path")
                .help("path of file contains initial state.")
                .required(true)
        )
        .arg(
            Arg::with_name("gens")
                .long("gens")
                .default_value("30")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("period")
                .long("period")
                .default_value("100")
                .takes_value(true)
        );

    let matches = app.get_matches();
    let path = matches.value_of("path").unwrap();
    let gens = matches.value_of("gens").unwrap().parse().unwrap();
    let period = matches.value_of("period").unwrap().parse().unwrap();
    let period = Duration::from_millis(period);

    let file = File::open(path)?;
    let mut life = Life::load(file)?;
    for _ in 0..gens {
        print!("{}[2J", 27 as char);
        println!("{}", life);
        life.update();
        thread::sleep(period);
    }
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

struct Life {
    world: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Life {
    pub fn load<R: Read>(r: R) -> io::Result<Self> {
        let br = BufReader::new(r);
        let mut world = Vec::new();
        for result in br.lines() {
            let line = result?;
            let mut row = Vec::new();
            for ch in line.chars() {
                let cell = match ch {
                    '.' => Cell::Dead,
                    '*' => Cell::Alive,
                    _ => return Err(io::Error::new(ErrorKind::InvalidInput, format!("failed to parse line {}", line))),
                };
                row.push(cell);
            }
            world.push(row);
        }
        let width = world[0].len();
        let height = world.len();
        Ok(Life { world, width, height })
    }

    pub fn update(&mut self) {
        let mut new_world = self.world.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let n = self.num_of_alive_neighbor(x, y);
                match self.get(x, y) {
                    Cell::Dead if n == 3 => new_world[y][x] = Cell::Alive,
                    Cell::Alive if n == 1 || (4 <= n && n <= 8) => new_world[y][x] = Cell::Dead,
                    _ => { /* Keep */ }
                }
            }
        }
        self.world = new_world;
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        self.world[y][x]
    }

    fn num_of_alive_neighbor(&self, x: usize, y: usize) -> usize {
        let mut n = 0;

        // top-left
        if 0 < x && 0 < y && self.get(x - 1, y - 1) == Cell::Alive {
            n += 1;
        }

        // top-center
        if 0 < y && self.get(x, y - 1) == Cell::Alive {
            n += 1;
        }

        // top-right
        if x + 1 < self.width && 0 < y && self.get(x + 1, y - 1) == Cell::Alive {
            n += 1;
        }

        // middle-left
        if 0 < x && self.get(x - 1, y) == Cell::Alive {
            n += 1;
        }

        // middle-right
        if x + 1 < self.width && self.get(x + 1, y) == Cell::Alive {
            n += 1;
        }

        // bottom-left
        if 0 < x && y + 1 < self.height && self.get(x - 1, y + 1) == Cell::Alive {
            n += 1;
        }

        // bottom-center
        if y + 1 < self.height && self.get(x, y + 1) == Cell::Alive {
            n += 1;
        }

        // bottom-right
        if x + 1 < self.width && y + 1 < self.height && self.get(x + 1, y + 1) == Cell::Alive {
            n += 1;
        }

        n
    }
}

impl Display for Life {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Cell::Alive => write!(f, "*")?,
                    Cell::Dead => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
