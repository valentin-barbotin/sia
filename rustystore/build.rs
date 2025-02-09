use std::path::PathBuf;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // panic!("{}", env::var("OUT_DIR").unwrap());
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(&descriptor_path.join("storefile_descriptor.bin"))
        .compile_protos(&["proto/storefile.proto"], &["."])?;

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(&descriptor_path.join("tag_descriptor.bin"))
        .compile_protos(&["proto/tag.proto"], &["."])?;

    Ok(())
}