use std::env;
use std::fs::OpenOptions;
use std::path::Path;

fn main()
{
	env::args()
	.skip(1)
	.map(|path_str| path_str.trim().to_string())
	.filter(|path_str| !path_str.is_empty())
	.filter(|path_str| !Path::new(path_str).exists())
	.for_each(|path_str|
		{
			if let Err(e) = OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(Path::new(&path_str))
			{
				eprintln!("Error creating file '{}': {}", path_str, e);
			}
		}
	);
}