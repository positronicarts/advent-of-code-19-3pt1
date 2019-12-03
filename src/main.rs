#[derive(Debug, PartialEq)]
enum Instruction {
    Up(i32),
    Left(i32),
    Right(i32),
    Down(i32),
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut chars = s.chars();
        let direction : char = chars.next().unwrap();
        let num : i32 = chars.collect::<String>().parse().unwrap();

        match direction {
            'U' => Ok(Instruction::Up(num)),
            'D' => Ok(Instruction::Down(num)),
            'L' => Ok(Instruction::Left(num)),
            'R' => Ok(Instruction::Right(num)),
            _ => Err(())
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("inputs.txt").unwrap();
    let mut lines = content.lines();

    let wire1 : Vec<String> = lines.next().unwrap().split(",").map(|x| x.to_string()).collect();
    let wire2 : Vec<String> = lines.next().unwrap().split(",").map(|x| x.to_string()).collect();

    let instructions1 : Vec<Instruction> = wire1.into_iter().map(|s| s.parse().unwrap()).collect();
    let instructions2 : Vec<Instruction> = wire2.into_iter().map(|s| s.parse().unwrap()).collect();

    let mut grid : std::collections::HashMap<(i64, i64), u16> = std::collections::HashMap::new();

    let mut x : i64 = 0;
    let mut y : i64 = 0;

    for instruction in &instructions1 {
        match instruction {
            Instruction::Up(distance) => {
                for _ii in 0..*distance {
                    y += 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 1, |val| val | 1);
                    grid.insert((x, y), val);
                }
            }
            Instruction::Down(distance) => {
                for _ii in 0..*distance {
                    y -= 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 1, |val| val | 1);
                    grid.insert((x, y), val);
                }
            }     
            Instruction::Left(distance) => {
                for _ii in 0..*distance {
                    x -= 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 1, |val| val | 1);
                    grid.insert((x, y), val);
                }
            }           
            Instruction::Right(distance) => {
                for _ii in 0..*distance {
                    x += 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 1, |val| val | 1);
                    grid.insert((x, y), val);
                }
            }       
        }
    }

    let mut x : i64 = 0;
    let mut y : i64 = 0;

    for instruction in &instructions2 {
        match instruction {
            Instruction::Up(distance) => {
                for _ii in 0..*distance {
                    y += 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 2, |val| val | 2);
                    grid.insert((x, y), val);
                }
            }
            Instruction::Down(distance) => {
                for _ii in 0..*distance {
                    y -= 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 2, |val| val | 2);
                    grid.insert((x, y), val);
                }
            }     
            Instruction::Left(distance) => {
                for _ii in 0..*distance {
                    x -= 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 2, |val| val | 2);
                    grid.insert((x, y), val);
                }
            }           
            Instruction::Right(distance) => {
                for _ii in 0..*distance {
                    x += 1;
                    let val = grid.get(&(x, y)).map_or_else(|| 2, |val| val | 2);
                    grid.insert((x, y), val);
                }
            }       
        }
    }    

    let mut dist = None;

    for ((x, y), val) in grid {
        if ((val & 1) == 1) && ((val & 2) == 2) && ((x != 0) || (y != 0)) {
            match dist {
                None => {
                    dist = Some(x.abs() + y.abs());
                }
                Some(d) => { 
                    if (x.abs() + y.abs()) < d {
                        dist = Some(x.abs() + y.abs());
                    }
                }
            }
        }
    }

    println!("{}", dist.unwrap());
}
