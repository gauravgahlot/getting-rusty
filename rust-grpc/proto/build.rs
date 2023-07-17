use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .compile(
            &[
                "src/protos/hello/hello.proto",
            ],
            &["src/protos"]
        )
        .unwrap();

    Ok(())
}
