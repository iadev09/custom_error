mod error;
mod fs;

pub use self::error::{Error, Result};
use fs::ls;
fn main() -> Result<()> {
	for folder in [".", "empty_folder", "not_exist_folder"] {
		println!("files : {:#?}", ls(folder));
	}
	Ok(())
}
