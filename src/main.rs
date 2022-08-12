pub mod sandpile;
use sandpile::Sandpile;

fn main() {
    // choose a grain amount
    let num_grains: usize = 10_000;

    // guess a size for that amount of grains
    let side_length: usize = 75;

    // find middle of the grid
    let middle_point = (side_length - 1) / 2;

    // create empty sandpile
    let mut s = Sandpile::new(side_length, side_length);

    // create sandtower in the middle of grid
    s.set_value_at(num_grains, (middle_point, middle_point));

    // topple until finished
    while !s.is_completely_toppled {
        s.topple_sandpile();
    }

    // print to console
    s.print_sandpile();
}
