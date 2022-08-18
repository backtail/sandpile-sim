pub mod output;
pub mod sandpile;

use clap::Parser;
use output::{create_png, raw_data_to_rgba};
// use output::create_gif;
use sandpile::Sandpile;
use std::path::PathBuf;
// use std::process;

// Structure representing the command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // number of grains
    #[clap(short, long, value_parser, default_value_t = 50_000)]
    num_grains: usize,

    // image side length
    #[clap(short, long, value_parser, default_value_t = 170)]
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

    // find middle of the grid
    let middle_point = (side_length - 1) / 2;

    // create empty sandpile
    let mut s = Sandpile::new(side_length, side_length);

    // create sandtower in the middle of grid
    s.set_value_at(num_grains, (middle_point, middle_point));

    // topple until finished
    while !s.is_completely_toppled {
        s.topple_torus(0, middle_point + 1, middle_point + 1);
    }

    println!(
        "Recursive had {} steps with {} topples!",
        s.num_steps, s.num_topples
    );

    // OS unspecific path
    let mut path = PathBuf::new();
    path.push(r".");
    path.push("render");

    // create custom file name
    let file_name = format!(
        "img_{}_grains_{}x{}px_recursive.png",
        num_grains, side_length, side_length
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

    // create empty sandpile
    let mut s = Sandpile::new(side_length, side_length);

    // create sandtower in the middle of grid
    s.set_value_at(num_grains, (middle_point, middle_point));

    // topple until finished
    while !s.is_completely_toppled {
        s.topple_sandpile();
    }

    println!(
        "Naive had {} steps with {} topples!",
        s.num_steps, s.num_topples
    );

    // OS unspecific pathprintln!
    let mut path = PathBuf::new();
    path.push(r".");
    path.push("render");

    // create custom file name
    let file_name = format!(
        "img_{}_grains_{}x{}px_naive.png",
        num_grains, side_length, side_length
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

    // // ------------------------------------------------------------------------------------------------------
    // // gif
    // // ------------------------------------------------------------------------------------------------------

    // // create empty sandpile
    // let mut s = Sandpile::new(side_length, side_length);

    // // define number of frames
    // let num_frames: usize = 30;

    // // buffer vector
    // let mut gif_buffer = Vec::new();

    // println!("Processing gif frames!");

    // // create many frames
    // for i in 0..num_frames {
    //     // reset sandpile
    //     s.reset();

    //     // create sandtower in the middle of grid
    //     s.set_value_at(num_grains, (middle_point, middle_point));

    //     // probabilitity from 0 to 1
    //     let p = (i + 1) as f32 / num_frames as f32;

    //     // set probability
    //     s.set_probailitiy(p).unwrap();

    //     // topple until finished
    //     while !s.is_completely_toppled {
    //         s.topple_sandpile();
    //     }

    //     // print progress in console
    //     println!("{:>3} of {:<3} with probability {}", i + 1, num_frames, p);

    //     let mut frame_data: Vec<u8> = vec![];

    //     raw_data_to_rgba(s.cells.data(), &mut frame_data);

    //     gif_buffer.push(frame_data);
    // }

    // // OS unspecific path
    // let mut path = PathBuf::new();
    // path.push(r".");
    // path.push("gif");

    // // create custom file name
    // let file_name = format!(
    //     "img_{}_grains_{}x{}px_{}_frames.gif",
    //     num_grains, side_length, side_length, num_frames
    // );

    // // extend path by custom file name
    // path.push(file_name);

    // create_gif(
    //     path.to_str().unwrap(),
    //     side_length as u16,
    //     side_length as u16,
    //     &mut gif_buffer,
    // );
}
