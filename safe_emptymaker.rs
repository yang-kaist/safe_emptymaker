use std::env;
use std::fs::{self, OpenOptions};
use std::path::Path;

fn main()
{
	env::args()
	.skip(1)
	.map(|path_str| path_str.trim().to_string())
	.filter(|path_str| !path_str.is_empty())
	.for_each(|path_str|
		{
			let path = Path::new(&path_str);
			if let Some(parent) = path.parent()
			{
				if !parent.as_os_str().is_empty() && !parent.exists()
				{
					if let Err(e) = fs::create_dir_all(parent)
					{
						eprintln!("Error creating directory '{}': {}",parent.display(),e);
						return;
					}
				}
			}

			if !path.exists()
			{
				if let Err(e) = OpenOptions::new()
				.write(true)
				.create_new(true)
				.open(path)
				{
					eprintln!("Error creating file '{}': {}", path.display(), e);
				}
			}
		}
	);
}
