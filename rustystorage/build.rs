use std::path::PathBuf;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // panic!("{}", env::var("OUT_DIR").unwrap());
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(&descriptor_path.join("file_descriptor.bin"))
        .compile(&["proto/file.proto"], &["proto"])?;

    Ok(())
}