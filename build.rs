use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("SKIP_PROTOC").is_ok() {
        println!("cargo:warning=Skipping protoc compilation");
        return Ok(());
    }

    // Collect all *.proto paths from the `proto/` folder
    let mut proto_files = Vec::new();
    for entry in fs::read_dir("proto")? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("proto") {
            proto_files.push(path);
        }
    }

    // Convert `PathBuf` to strings where needed
    let proto_files_str = proto_files
        .iter()
        .map(|p| p.to_str().unwrap())
        .collect::<Vec<_>>();

    // Now configure tonic_build to compile everything into `src/protobuf`
    tonic_build::configure()
        .out_dir("src/protobuf")
        // The first slice is the list of .proto files,
        // The second slice is the "include" paths.
        .compile_protos(&proto_files_str, &["proto"])?;

    Ok(())
}
