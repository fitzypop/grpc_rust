use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile(&["./proto/todo.proto"], &["."])
        .unwrap_or_else(|e| panic!("protobuf compilation error: {}", e));
    Ok(())
}
