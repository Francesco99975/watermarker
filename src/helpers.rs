use std::fs;

pub fn get_dirs_images_paths(dirs: Vec<&String>) -> Vec<String> {
    let mut image_paths: Vec<String> = Vec::new();
    for dir in dirs {
        let paths = fs::read_dir(dir).expect("Could not get file paths");
        for path in paths {
            let clean_path = path.expect("Path retreival Error").path();

            if clean_path.is_file() {
                image_paths.push(
                    clean_path
                        .to_str()
                        .expect("File Path Parsing error")
                        .to_string(),
                );
            }
        }
    }

    image_paths
}
