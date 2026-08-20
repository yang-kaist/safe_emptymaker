use std::env;
use std::fs;
use std::path::Path;

fn main() 
{
	env::args()
	.skip(1)
	.map(|path_str| path_str.trim().to_string())
	.filter(|path_str| !path_str.is_empty())
	.filter_map(|path_str|
		{
			Path::new(&path_str)
			.parent()
			.map(|p| p.to_path_buf())
		}
	)
	.filter(|dir_path| !dir_path.as_os_str().is_empty())
	.filter(|dir_path| !dir_path.exists())
	.for_each(|dir_path| 
		{
			if let Err(e) = fs::create_dir_all(&dir_path) 
			{
				eprintln!("Error creating directory '{}': {}", dir_path.display(), e);
			}
		}
	);
}