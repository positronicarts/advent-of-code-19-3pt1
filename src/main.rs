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
        let direction: char = chars.next().unwrap();
        let num: i32 = chars.collect::<String>().parse().unwrap();

        match direction {
            'U' => Ok(Instruction::Up(num)),
            'D' => Ok(Instruction::Down(num)),
            'L' => Ok(Instruction::Left(num)),
            'R' => Ok(Instruction::Right(num)),
            _ => Err(()),
        }
    }
}

fn handle_instructions(
    grid: &mut std::collections::HashMap<(i64, i64), u16>,
    instructions: Vec<Instruction>,
    bitval: u16,
) {
    let (mut x, mut y): (i64, i64) = (0, 0);

    for instruction in &instructions {
        let (dist, movement): (&i32, Box<dyn Fn(i64, i64) -> (i64, i64)>) = match instruction {
            Instruction::Up(dist) => (dist, Box::new(|x, y| (x, y + 1))),
            Instruction::Down(dist) => (dist, Box::new(|x, y| (x, y - 1))),
            Instruction::Left(dist) => (dist, Box::new(|x, y| (x - 1, y))),
            Instruction::Right(dist) => (dist, Box::new(|x, y| (x + 1, y))),
        };

        for _ii in 0..*dist {
            let new_coords = movement(x, y);
            x = new_coords.0;
            y = new_coords.1;
            let val = grid.get(&(x, y)).map_or_else(|| bitval, |val| val | bitval);
            grid.insert((x, y), val);
        }
    }
}

fn extract_instructions<'a>(lines: &mut std::str::Lines<'a>) -> Vec<Instruction> {
    lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn main() {
    let content = std::fs::read_to_string("inputs.txt").unwrap();
    let mut lines = content.lines();
    let (i1, i2) = (extract_instructions(&mut lines), extract_instructions(&mut lines));
    
    let mut grid: std::collections::HashMap<(i64, i64), u16> = std::collections::HashMap::new();
    handle_instructions(&mut grid, i1, 1);
    handle_instructions(&mut grid, i2, 2);

    println!(
        "{}",
        grid.iter()
            .filter(|(_, val)| **val == 3)
            .filter(|((x, y), _)| *x != 0 || *y != 0)
            .map(|((x, y), _)| x.abs() + y.abs())
            .min()
            .unwrap()
    );
}
