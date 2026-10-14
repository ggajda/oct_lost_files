use std::{env, io};
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()?;
    }
    Ok(())
}
