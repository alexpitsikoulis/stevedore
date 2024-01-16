fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/stevedore.proto")?;
    tonic_build::compile_protos("proto/certificate_authority.proto")?;
    Ok(())
}
