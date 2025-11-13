use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/proto/email_service.proto");
    tonic_build::compile_protos("src/proto/email_service.proto")?;
    Ok(())
}