use clap::ErrorKind;
use colored::Colorize;
use rand::{self, Rng};
use toodee::TooDee;

pub struct Sandpile {
    x: usize,
    y: usize,
    pub cells: TooDee<usize>,
    probability_to_topple: f32,
    pub is_completely_toppled: bool,
    pub num_steps: usize,
    pub num_topples: usize,
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
            num_steps: 0,
            num_topples: 0,
        }
    }

    pub fn print_sandpile(&self) {
        // every cell but the "invisible" ones
        for i in 0..(self.x - 0) {
            for j in 0..(self.y - 0) {
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

    pub fn topple(&mut self, value: usize, x: usize, y: usize) {
        let x_compare = x as isize;
        let y_compare = y as isize;
        let x_max = (self.x - 1) as isize;
        let y_max = (self.y - 1) as isize;

        // increase step count
        self.num_steps += 1;
        if x_compare + 1 <= x_max
            && y_compare + 1 <= y_max
            && x_compare - 1 >= 0
            && y_compare - 1 >= 0
        {
            // add topple amount
            self.cells[(x, y)] += value;

            if self.cells[(x, y)] >= 4 {
                // increase topples count
                self.num_topples += 1;

                let multiples = self.cells[(x, y)] / 4;
                self.cells[(x, y)] -= 4 * multiples;

                self.topple(multiples, x - 1, y);
                self.topple(multiples, x, y - 1);
                self.topple(multiples, x + 1, y);
                self.topple(multiples, x, y + 1);
            }

            self.is_completely_toppled = true;
        }
    }

    pub fn topple_torus(&mut self, value: usize, x: usize, y: usize) {
        let x_max = self.x - 1;
        let y_max = self.y - 1;

        // increase step count
        self.num_steps += 1;

        // add topple amount
        self.cells[(x, y)] += value;

        if self.cells[(x, y)] >= 4 {
            // increase topples count
            self.num_topples += 1;

            let multiples = self.cells[(x, y)] / 4;
            self.cells[(x, y)] -= 4 * multiples;

            // check bound and wrap around if at border
            if x == 0 {
                self.topple_torus(multiples, x_max, y);
            } else {
                self.topple_torus(multiples, x - 1, y);
            }

            if y == 0 {
                self.topple_torus(multiples, x, y_max);
            } else {
                self.topple_torus(multiples, x, y - 1);
            }

            if x == x_max {
                self.topple_torus(multiples, 0, y);
            } else {
                self.topple_torus(multiples, x + 1, y);
            }

            if y == y_max {
                self.topple_torus(multiples, x, 0)
            } else {
                self.topple_torus(multiples, x, y + 1);
            }
        }

        self.is_completely_toppled = true;
    }

    pub fn topple_sandpile(&mut self) {
        let mut been_toppled = false;

        // use old algorithm if probability is 1 since it is way more effecient
        if self.probability_to_topple == 1.0 {
            for i in 1..(self.x - 1) {
                for j in 1..(self.y - 1) {
                    // increase step count
                    self.num_steps += 1;

                    // most efficitient algorithm for big piles
                    if self.cells[(i, j)] >= 8 {
                        // increase topples count
                        self.num_topples += 1;

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
                        // increase topples count
                        self.num_topples += 1;

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
        } else {
            // create a random number tread for this topple stage
            let mut rng = rand::thread_rng();

            for i in 1..(self.x - 1) {
                for j in 1..(self.y - 1) {
                    if self.cells[(i, j)] > 3 {
                        // count how many grains have been moved
                        let mut moved_grain_counter: usize = 0;

                        // create random number between 0 and 1
                        // when the random number is higher than the probability threshold, dont't move sandgrain
                        let random: [f32; 4] = rng.gen();

                        // move grains to neighbouring cells if threshold is not met
                        for counter in 0..random.len() {
                            if random[counter] < self.probability_to_topple {
                                match counter {
                                    0 => {
                                        self.cells[(i - 1, j)] += 1;
                                    }
                                    1 => {
                                        self.cells[(i, j - 1)] += 1;
                                    }
                                    2 => {
                                        self.cells[(i + 1, j)] += 1;
                                    }
                                    3 => {
                                        self.cells[(i, j + 1)] += 1;
                                    }
                                    _ => (),
                                }
                                moved_grain_counter += 1;
                            }
                        }

                        // reduce grains that have been moved
                        self.cells[(i, j)] -= moved_grain_counter;

                        // even if no grains have been moved
                        // with really low probability values it could take a few passes until a grain will be moved
                        // setting this bool to true means piles with more than 3 grains have been found
                        been_toppled = true;
                    }
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
