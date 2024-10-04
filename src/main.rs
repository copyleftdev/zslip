use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} --path=\"path/to/file\" --data=\"file content\" output.zip", args[0]);
        return 1;
    }


    let file_path = args[1].replace("--path=", "");
    let file_content = args[2].replace("--data=", "");
    let output_zip = &args[3];

    match create_zip_file(output_zip, &file_path, &file_content) {
        Ok(_) => println!("Zip file successfully written to {output_zip}"),
        Err(e) => println!("Error: {e:?}"),
    }

    0
}

fn create_zip_file(zip_filename: &str, file_path: &str, file_content: &str) -> zip::result::ZipResult<()> {
    let zip_path = Path::new(zip_filename);
    let file = File::create(zip_path).expect("Failed to create the zip file");

    let mut zip = ZipWriter::new(file);


    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);


    zip.start_file(file_path, options)?;
    zip.write_all(file_content.as_bytes())?;

 
    zip.finish()?;
    Ok(())
}
