use anyhow::{bail, Context, Result};
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    io::ErrorKind,
    path::Path,
};

use crate::embedded::EMBEDDED_FILES;

const CARGO_TOML: &[u8] = {
    let cargo_toml = include_bytes!("../dev/Cargo.toml");
    // Skip the first line (comment).
    let mut start_ind = 0;
    while cargo_toml[start_ind] != b'\n' {
        start_ind += 1;
    }
    cargo_toml.split_at(start_ind + 1).1
};

pub fn init() -> Result<()> {
    if Path::new("exercises").is_dir() && Path::new("Cargo.toml").is_file() {
        bail!(PROBABLY_IN_RUSTLINGS_DIR_ERR);
    }

    let rustlings_path = Path::new("rustlings");
    if let Err(e) = create_dir(rustlings_path) {
        if e.kind() == ErrorKind::AlreadyExists {
            bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
        }
        return Err(e.into());
    }

    set_current_dir("rustlings")
        .context("Failed to change the current directory to `rustlings`")?;

    EMBEDDED_FILES
        .init_exercises_dir()
        .context("Failed to initialize the `rustlings/exercises` directory")?;

    fs::write("Cargo.toml", CARGO_TOML)
        .context("Failed to create the file `rustlings/Cargo.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `rustlings/.gitignore`")?;

    create_dir(".vscode").context("Failed to create the directory `rustlings/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    println!("{POST_INIT_MSG}");

    Ok(())
}

pub const GITIGNORE: &[u8] = b"Cargo.lock
.rustlings-state.txt
target
";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const PROBABLY_IN_RUSTLINGS_DIR_ERR: &str =
    "A directory with the name `exercises` and a file with the name `Cargo.toml` already exist
in the current directory. It looks like Rustlings was already initialized here.
Run `rustlings` for instructions on getting started with the exercises.

If you didn't already initialize Rustlings, please initialize it in another directory.";

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `rustlings` already exists in the current directory.
You probably already initialized Rustlings.
Run `cd rustlings`
Then run `rustlings` again";

const POST_INIT_MSG: &str = "Done initialization!

Run `cd rustlings` to go into the generated directory.
Then run `rustlings` to get started.";
