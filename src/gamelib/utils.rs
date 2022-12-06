use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::ops::Rem;
use std::path::Path;
use image::Luma;
use qrcode::QrCode;
use uuid::Uuid;

/// Filter word list to only contain purely alphabetic words with word length of 6
///
/// # Arguments
///
/// * `input_path`: path of the input file
/// * `output_path`: path of the output file
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use utils;
/// utils::filter_word_lists("unfiltered.txt", "filtered.txt")
/// ```
pub fn filter_word_lists<P>(input_path: P, output_path: P, word_length: usize) -> u128
    where P: AsRef<Path> {
    let input_file = File::open(input_path).unwrap();
    let output_file = File::create(output_path).unwrap();

    let input_lines = io::BufReader::new(input_file).lines();
    let mut output_lines = io::LineWriter::new(output_file);

    let mut len = 0;
    for line in input_lines.flatten() {
        if line.chars().all(char::is_alphabetic) && line.chars().count() == word_length {
            output_lines.write_all((line + "\n").as_bytes()).unwrap();
            len += 1;
        }
    }
    len as u128
}

/// Generate identifying words from uuid and word list path
///
/// # Arguments
///
/// * `uuid`: input uuid
/// * `path`: input word list path
///
/// returns: String
pub fn uuid_to_words<P>(uuid: Uuid, path: P) -> String
    where P: AsRef<Path> {
    let mut uuid = uuid.as_u128();
    let file = File::open(path).unwrap();
    let words: Vec<String> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();
    let words_len = words.len() as u128;

    let mut word_list = VecDeque::new();
    while uuid != 0 {
        let rem = uuid.rem(words_len);
        word_list.push_front(words[rem as usize].clone());
        uuid /= words_len
    }
    Vec::from(word_list).join(" ")
}

/// Generate QR code from uuid and save it as PNG
///
/// # Arguments
///
/// * `uuid`: input uuid
/// * `image_path`: output path to save PNG image
///
/// returns: ()
pub fn uuid_to_qrcode<P>(uuid: Uuid, image_path: P)
    where P: AsRef<Path> {
    let uuid = uuid.as_bytes();
    let code = QrCode::new(uuid).unwrap();
    code.render::<Luma<u8>>().build().save(image_path).unwrap()
}
