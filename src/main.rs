pub mod output;
pub mod sandpile;

use output::{create_png, raw_data_to_rgba};
use sandpile::Sandpile;

fn main() {
    // choose a grain amount
    let num_grains: usize = 1_000_000;

    // guess a size for that amount of grains
    let side_length: usize = 740;

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

    // create custom file name
    let path = format!(
        ".\\render\\img_{}_grains_{}x{}px.png",
        num_grains, side_length, side_length
    );

    // convert data to RGBA values from empty vector
    let mut rgba_data: Vec<u8> = vec![];
    raw_data_to_rgba(s.cells.data(), &mut rgba_data);

    // write data to png file
    create_png(
        path,
        (side_length + 2) as u32,
        (side_length + 2) as u32,
        rgba_data.as_slice(),
    );
}
