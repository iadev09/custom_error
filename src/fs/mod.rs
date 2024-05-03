mod error;

pub use error::{Error, Result};

pub fn ls(path: &str) -> Result<Vec<String>> {
	let files: Vec<String> = std::fs::read_dir(path)?
		.filter_map(|f| f.ok())
		.filter(|e| {
			e.file_type()
				.map(|ft| ft.is_file())
				.unwrap_or(false)
		})
		.filter_map(|e| e.file_name().into_string().ok())
		.collect();

	if files.is_empty() {
		return Err(Error::FolderIsEmpty);
	}

	Ok(files)
}
