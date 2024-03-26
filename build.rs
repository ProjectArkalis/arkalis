fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = tonic_build::configure();
    let conf = conf.protoc_arg("--experimental_allow_proto3_optional");
    conf.compile(&["protos/arkalis.proto"], &[""])?;
    Ok(())
}
