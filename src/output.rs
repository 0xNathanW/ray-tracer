use std::fs::File;
use std::io::Write;
use crate::render::Image;
use crate::OutputFormat;

pub fn write_to_file(
    file_name: &str, 
    image: Image, 
    format: OutputFormat, 
    dimensions: (usize, usize),
) -> Result<(), std::io::Error> {

    let extension: &str = match format {
        OutputFormat::PNG => "png",
        OutputFormat::PPM => "ppm",
    };
    let flat_img = image.into_iter().flatten().collect::<Vec<u8>>();

    match format {
        OutputFormat::PNG => {
            image::save_buffer_with_format(
                &format!("{}.{}", file_name, extension),
                &flat_img.as_slice(),
                dimensions.0 as u32,
                dimensions.1 as u32,
                image::ColorType::Rgb8,
                image::ImageFormat::Png,
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        },

        OutputFormat::PPM => {
            let mut file = File::create(&format!("{}.{}", file_name, extension))?;
            file.write(format!("P3\n{} {}\n255\n", dimensions.0, dimensions.1).as_bytes())?;
            for pixel in flat_img.chunks(3) {
                file.write(format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]).as_bytes())?;
            }
            Ok(())
        }
    }
}
