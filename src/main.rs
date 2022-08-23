pub mod output;
pub mod sandpile;

use std::panic::catch_unwind;
use std::sync::Mutex;

use clap::Parser;
use output::raw_data_to_rgba;
// use output::{create_gif, create_png};
use sandpile::Sandpile;
// use std::path::PathBuf;
// use std::process;

use speedy2d::color::Color;
use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

// Structure representing the command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // number of grains
    #[clap(short, long, value_parser, default_value_t = 1)]
    num_grains: usize,

    // image side length
    #[clap(short, long, value_parser, default_value_t = 100)]
    len_sides: usize,

    // probability parameter
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    probability: f32,
}

struct MyWindowHandler {
    sandpile: Sandpile,
    _num_grains: usize,
    canvas_size: (usize, usize),
    window_size: (u32, u32),
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self._num_grains += 1;

        // find middle of the grid
        let middle_point = (self.canvas_size.0 - 1) / 2;

        // create empty sandpile
        let mut s = Mutex::new(Sandpile::new(self.canvas_size.0, self.canvas_size.1));

        // create sandtower in the middle of grid
        s.get_mut()
            .unwrap()
            .set_value_at(self._num_grains, (middle_point, middle_point));

        let helper = Mutex::new(helper);
        let graphics = Mutex::new(graphics);

        // convert data to RGBA values from empty vector
        let rgba_data: Mutex<Vec<u8>> = Mutex::new(vec![]);

        if catch_unwind(|| {
            let mut s = s.into_inner().unwrap().clone();
            let rgba_data = &mut rgba_data.into_inner().unwrap();
            let helper = helper.into_inner().unwrap();
            let graphics = graphics.into_inner().unwrap();

            // topple torus sandpile each frame
            while !s.is_completely_toppled {
                s.topple_torus(0, middle_point, middle_point);
            }

            // println!("{:?}", s);

            raw_data_to_rgba(s.cells.data(), rgba_data);

            // disguise frame as image
            let image = graphics
                .create_image_from_raw_pixels(
                    ImageDataType::RGBA,
                    ImageSmoothingMode::NearestNeighbor,
                    (self.canvas_size.0 as u32, self.canvas_size.1 as u32),
                    rgba_data.as_slice(),
                )
                .unwrap();

            graphics.clear_screen(Color::BLACK);

            graphics.draw_rectangle_image(
                Rectangle::from_tuples(
                    (0.0, 0.0),
                    (self.window_size.0 as f32, self.window_size.1 as f32),
                ),
                &image,
            );

            if self._num_grains < 500 {
                helper.request_redraw();
            }
        })
        .is_err()
        {
            println!("Oh uh!");
        }

        // println!(
        //     "It took {} topples to topple {} grains!",
        //     s.get_mut().unwrap().num_topples,
        //     self._num_grains
        // );

        // if !self.sandpile.is_completely_toppled {
        //     self.sandpile.topple_torus_naive();
        //     helper.request_redraw();
        // } else {
        //     println!("Sandpile is stable!");
        //     println!(
        //         "It took {} steps with {} topples! ",
        //         self.sandpile.num_steps, self.sandpile.num_topples
        //     );
        // }
    }
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

    let window_size = (900, 900);

    // create window
    let window = Window::new_centered("Donut Sandpile", window_size).unwrap();

    // run window
    window.run_loop(MyWindowHandler {
        sandpile: s,
        _num_grains: num_grains,
        canvas_size: (side_length, side_length),
        window_size,
    });

    // // topple until finished
    // while !s.is_completely_toppled {
    //     s.topple_torus_naive();
    // }

    // println!(
    //     "Naive had {} steps with {} topples!",
    //     s.num_steps, s.num_topples
    // );

    // // OS unspecific pathprintln!
    // let mut path = PathBuf::new();
    // path.push(r".");
    // path.push("render");

    // // create custom file name
    // let file_name = format!(
    //     "img_{}_grains_{}x{}px_naive.png",
    //     num_grains, side_length, side_length
    // );

    // // extend path by custom file name
    // path.push(file_name);

    // // convert data to RGBA values from empty vector
    // let mut rgba_data: Vec<u8> = vec![];
    // raw_data_to_rgba(s.cells.data(), &mut rgba_data);

    // // write data to png file
    // create_png(
    //     path.to_str().unwrap(),
    //     (side_length + 2) as u32,
    //     (side_length + 2) as u32,
    //     rgba_data.as_slice(),
    // );
}
