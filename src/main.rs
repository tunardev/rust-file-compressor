use std::io::{Read, Write};
use std::fs::{File};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

fn compress_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file contents
    let mut input = File::open(input_file)?;
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    // Compress the file contents  
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    
    // Finalize the compression
    let compressed_data = encoder.finish()?;

    // Write the compressed data to the output file
    let mut output = File::create(output_file)?;
    output.write_all(&compressed_data)?;

    Ok(())
}

fn decompress_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file contents
    let mut input = File::open(input_file)?;
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    // Decompress the file contents
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    // Write the decompressed data to the output file
    let mut output = File::create(output_file)?;
    output.write_all(&decompressed_data)?;

    Ok(())
}

fn main() {
    // Compress the file
    let input_file = "input.txt"; 
    let output_file = "output.txt.gz";

    // Call the compress_file function
    if let Err(e) = compress_file(input_file, output_file) {
        eprintln!("Error: {}", e);
    } else {
        println!("File {} compressed to {}", input_file, output_file);
    }

    // Decompress the file
    let input_file = "output.txt.gz";
    let output_file = "output.txt";

    // Call the decompress_file function
    if let Err(e) = decompress_file(input_file, output_file) {
        eprintln!("Error: {}", e);
    } else {
        println!("File {} decompressed to {}", input_file, output_file);
    }
}
