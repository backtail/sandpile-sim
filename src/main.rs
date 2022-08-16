pub mod output;
pub mod sandpile;

use clap::Parser;
use output::{create_png, raw_data_to_rgba};
use sandpile::Sandpile;
use std::path::PathBuf;
use std::process;

// Structure representing the command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // number of grains
    #[clap(short, long, value_parser, default_value_t = 10_000)]
    num_grains: usize,

    // image side length
    #[clap(short, long, value_parser, default_value_t = 75)]
    len_sides: usize,

    // probability parameter
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    probability: f32,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Get side length
    let side_length = args.len_sides;

    // Get number of grains to start with
    let num_grains = args.num_grains;

    // get probability paramter
    let probability = args.probability;

    // find middle of the grid
    let middle_point = (side_length - 1) / 2;

    // create empty sandpile
    let mut s = Sandpile::new(side_length, side_length);

    // create sandtower in the middle of grid
    s.set_value_at(num_grains, (middle_point, middle_point));

    // set probability
    match s.set_probailitiy(probability) {
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("Probability value must be between 0.0 and 1.0!");
            eprintln!("Exiting program!");
            process::exit(0)
        }

        Ok(()) => (),
    }

    // topple until finished
    while !s.is_completely_toppled {
        s.topple_sandpile();
    }

    // OS unspecific path
    let mut path = PathBuf::new();
    path.push(r".");
    path.push("render");

    // create custom file name
    let file_name = format!(
        "img_{}_grains_{}x{}px_probability_{}.png",
        num_grains, side_length, side_length, probability
    );

    // extend path by custom file name
    path.push(file_name);

    // convert data to RGBA values from empty vector
    let mut rgba_data: Vec<u8> = vec![];
    raw_data_to_rgba(s.cells.data(), &mut rgba_data);

    // write data to png file
    create_png(
        path.to_str().unwrap(),
        (side_length + 2) as u32,
        (side_length + 2) as u32,
        rgba_data.as_slice(),
    );
}
