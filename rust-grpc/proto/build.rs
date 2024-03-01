use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .out_dir("src/api") // the directory must exist before the build
        .compile(
            &["src/protos/hello/hello.proto"],
            &[
                "src/protos/",                
            ]
        )?;

    Ok(())
}
