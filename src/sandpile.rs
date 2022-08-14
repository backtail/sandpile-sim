use clap::ErrorKind;
use colored::Colorize;
use toodee::TooDee;

pub struct Sandpile {
    x: usize,
    y: usize,
    pub cells: TooDee<usize>,
    probability_to_topple: f32,
    pub is_completely_toppled: bool,
}

impl Sandpile {
    pub fn new(x: usize, y: usize) -> Self {
        // first and last rows are "invisible" to make computations easier
        let x_offseted = x + 2;
        let y_offseted = y + 2;

        Sandpile {
            x: x_offseted,
            y: y_offseted,
            cells: TooDee::init(x_offseted, y_offseted, 0),
            probability_to_topple: 1.0,
            is_completely_toppled: false,
        }
    }

    pub fn print_sandpile(&self) {
        // every cell but the "invisible" ones
        for i in 1..(self.x - 1) {
            for j in 1..(self.y - 1) {
                let value = self.cells[(i, j)];
                let output_string;
                match value {
                    0 => {
                        output_string = "██".black();
                    }
                    1 => {
                        output_string = "██".blue();
                    }
                    2 => {
                        output_string = "██".green();
                    }
                    3 => {
                        output_string = "██".red();
                    }
                    _ => {
                        output_string = "██".purple();
                    }
                }
                print!("{:2}", output_string);
            }
            println!();
            // println!();
        }
        println!();
    }

    pub fn set_value_at(&mut self, value: usize, coordinate: (usize, usize)) {
        // offset coordinates by 1 since first row and colums are "invisible"
        let adjusted_coordinates = (coordinate.0 + 1, coordinate.1 + 1);
        self.cells[adjusted_coordinates] = value;
    }

    pub fn set_probailitiy(&mut self, value: f32) -> Result<(), ErrorKind> {
        match value {
            x if x <= 0.0 => {
                self.probability_to_topple = 0.001;
                Err(ErrorKind::ValueValidation)
            }

            x if x > 0.0 && x <= 1.0 => {
                self.probability_to_topple = value;
                Ok(())
            }

            _ => {
                self.probability_to_topple = 1.0;
                Err(ErrorKind::ValueValidation)
            }
        }
    }

    pub fn topple_sandpile(&mut self) {
        let mut been_toppled = false;
        for i in 1..(self.x - 1) {
            for j in 1..(self.y - 1) {
                // most efficitient algorithm for big piles
                if self.cells[(i, j)] >= 8 {
                    let multiples = self.cells[(i, j)] / 4;

                    // reduce pile that's too big
                    self.cells[(i, j)] -= 4 * multiples;
                    been_toppled = true;
                    // move grains to neighbouring cells
                    self.cells[(i - 1, j)] += multiples;
                    self.cells[(i, j - 1)] += multiples;
                    self.cells[(i + 1, j)] += multiples;
                    self.cells[(i, j + 1)] += multiples;
                }

                // less division and multiplication for small piles
                if self.cells[(i, j)] > 3 && self.cells[(i, j)] < 8 {
                    // reduce pile that's too big
                    self.cells[(i, j)] -= 4;
                    been_toppled = true;
                    // move grains to neighbouring cells
                    self.cells[(i - 1, j)] += 1;
                    self.cells[(i, j - 1)] += 1;
                    self.cells[(i + 1, j)] += 1;
                    self.cells[(i, j + 1)] += 1;
                }
            }
        }
        if !been_toppled {
            self.is_completely_toppled = true;
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.x {
            for j in 0..self.y {
                self.cells[(i, j)] = 0;
            }
        }
        self.is_completely_toppled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::Sandpile;
    use clap::ErrorKind;

    #[test]
    fn probability() {
        let mut s = Sandpile::new(0, 0);

        // make sure probability is 1.0 when a sandpile is created
        assert_eq!(s.probability_to_topple, 1.0);

        // check all possible states
        let res = s.set_probailitiy(-1.0);
        assert_eq!(res, Err(ErrorKind::ValueValidation));
        assert_eq!(s.probability_to_topple, 0.001);

        let res = s.set_probailitiy(0.0);
        assert_eq!(res, Err(ErrorKind::ValueValidation));
        assert_eq!(s.probability_to_topple, 0.001);

        let res = s.set_probailitiy(2.0);
        assert_eq!(res, Err(ErrorKind::ValueValidation));
        assert_eq!(s.probability_to_topple, 1.0);

        let res = s.set_probailitiy(0.5);
        assert_eq!(res, Ok(()));
        assert_eq!(s.probability_to_topple, 0.5);
    }
}
