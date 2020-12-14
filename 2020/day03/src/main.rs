use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut slope = Slope::parse(&input);
    println!("The slope is {}x{}", slope.width, slope.length);

    slope.stride = 1; slope.speed = 1; let trees1 = slope.num_trees_in_path();
    slope.stride = 3; slope.speed = 1; let trees2 = slope.num_trees_in_path();
    slope.stride = 5; slope.speed = 1; let trees3 = slope.num_trees_in_path();
    slope.stride = 7; slope.speed = 1; let trees4 = slope.num_trees_in_path();
    slope.stride = 1; slope.speed = 2; let trees5 = slope.num_trees_in_path();

    println!("There are {} trees on this path", trees1);
    println!("There are {} trees on this path (Part 1)", trees2);
    println!("There are {} trees on this path", trees3);
    println!("There are {} trees on this path", trees4);
    println!("There are {} trees on this path", trees5);

    println!("Multiplication of encountered trees: {} (Part 2)", trees1*trees2*trees3*trees4*trees5);

    Ok(())
}

struct Slope {
    width: usize,
    length: usize,
    stride: usize,
    speed: usize,
    map: Vec<Vec<char>>,
}

impl Slope {
    fn new(width: usize, length: usize, stride: usize, speed: usize, map: Vec<Vec<char>>) -> Self { Self { width, length, stride, speed, map } }

    fn parse(data: &String) -> Self {
        let rows: Vec<&str> = data.trim().lines().collect();

        let width: usize = rows[0].len();
        let length: usize = rows.len();
        let stride: usize = 1;
        let speed: usize = 1;

        let mut map: Vec<Vec<char>> = vec![];

        for r in rows {
            map.push( r.chars().collect::<Vec<char>>() );
        }

        Slope::new(width, length, stride, speed, map)
    }

    fn coord(&self, step: usize) -> (usize, usize) {
        (step*self.speed, step*self.stride % self.width)
    }

    fn square(&self, row: usize, column: usize) -> char {
        self.map[row][column]
    }

    fn is_tree(&self, row: usize, column: usize) -> bool {
        self.square(row, column).eq_ignore_ascii_case(&'#')
    }

    fn _is_open(&self, row: usize, column: usize) -> bool {
        self.square(row, column).eq_ignore_ascii_case(&'.')
    }

    fn valid(&self, step: usize) -> bool {
        self.coord(step).0 < self.length
    }

    fn num_trees_in_path(&self) -> usize {
        let mut n: usize = 0;
        let mut trees: usize = 0;
        loop {
            if !self.valid(n) {break;}

            let (row, column) = self.coord(n);
            if self.is_tree(row, column) {trees += 1;}

            n += 1;
        }

        trees
    }
}
