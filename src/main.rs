//! # Spire

use spire::{Editor, Error};

fn main() -> Result<(), Error> {
    let mut args = std::env::args();

    match (args.nth(1), args.len()) {
        (Some(arg), 0) if arg == "--version" => println!("spire {}", "0.0.1"), // TODO: env variable
        (Some(arg), 0) if arg.starts_with('-') => return Err(Error::UnrecognizedOption(arg)),
        (file_name, 0) => Editor::new().run(&file_name)?,
        (_, num_args) => return Err(Error::TooManyArguments(num_args + 1)),
    }

    Ok(())
}
