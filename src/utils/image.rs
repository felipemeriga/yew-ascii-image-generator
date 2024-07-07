use ::image::{imageops::FilterType, DynamicImage, GenericImageView};
use gloo::file::{File, FileReadError};
use image::io::Reader as ImageReader;
use std::io;
use std::io::{Cursor, ErrorKind};
use thiserror::Error;
use web_sys::FileList;

#[derive(Error, Debug)]
pub enum ImageProcessingError {
    #[error("No files in the list")]
    NoFilesInList,

    #[error("Failed to guess format: {0}")]
    FormatGuessFailed(String),

    #[error("Failed to decode: {0}")]
    DecodeFailed(String),

    #[error("Image error occurred: {source}")]
    FileReadError {
        #[from]
        source: FileReadError,
    },

    #[error("{source}")]
    FileSizeError {
        #[from]
        source: io::Error,
    },
}

#[derive(Debug, Clone)]
pub struct ImageResult {
    pub data: Vec<u8>,
    pub ascii: String,
}

pub fn convert_to_ascii(img: DynamicImage, resolution: u32, print_info: bool) -> String {
    let character_set: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];
    let mut art = String::new();
    let mut last_y = 0;

    let small_img = img.resize(
        img.width() / resolution,
        img.height() / resolution,
        FilterType::Nearest,
    );

    if print_info {
        println!(
            "Original size: {:?}   Reduced: {:?}",
            img.dimensions(),
            small_img.dimensions()
        );
    }

    for pixel in small_img.pixels() {
        if last_y != pixel.1 {
            art.push('\n');
            last_y = pixel.1;
        }

        let pixel_data = pixel.2;
        let brightness: f64 =
            ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position =
            ((brightness / 255.0) * (character_set.len() - 1) as f64).round() as usize;
        art.push_str(character_set[character_position])
    }

    art
}

fn check_file_size(file: &File) -> io::Result<()> {
    let size_in_mb = file.size() / 1024 / 1024;
    if size_in_mb > 1 {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "File size is larger than 1 MB",
        ));
    }
    Ok(())
}

pub async fn image_process(file_list: FileList) -> Result<ImageResult, ImageProcessingError> {
    let files = gloo::file::FileList::from(file_list);
    let file = files.first().ok_or(ImageProcessingError::NoFilesInList)?;
    check_file_size(file)?;
    let data = gloo::file::futures::read_as_bytes(file).await?;

    let reader = ImageReader::new(Cursor::new(&data))
        .with_guessed_format()
        .map_err(|e| ImageProcessingError::FormatGuessFailed(e.to_string()))?;

    let image = reader
        .decode()
        .map_err(|e| ImageProcessingError::DecodeFailed(e.to_string()))?;
    let ascii = convert_to_ascii(image, 1, false);

    Ok(ImageResult { ascii, data })
}
