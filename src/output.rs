use png;
use std::fs::File;
use std::io::BufWriter;

pub fn create_png(path: String, width: u32, height: u32, data: &[u8]) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

pub fn raw_data_to_rgba(input_data: &[usize], output_data: &mut Vec<u8>) {
    for i in 0..input_data.len() {
        match input_data[i] as usize {
            0 => {
                // invisible
                output_data.push(0);
                output_data.push(0);
                output_data.push(0);
                output_data.push(0);
            }
            1 => {
                // blue
                output_data.push(0);
                output_data.push(0);
                output_data.push(255);
                output_data.push(255);
            }
            2 => {
                // green
                output_data.push(0);
                output_data.push(255);
                output_data.push(0);
                output_data.push(255)
            }
            3 => {
                // red
                output_data.push(255);
                output_data.push(0);
                output_data.push(0);
                output_data.push(255);
            }
            _ => {
                // invisible
                output_data.push(0);
                output_data.push(0);
                output_data.push(0);
                output_data.push(0);
            }
        }
    }
}
