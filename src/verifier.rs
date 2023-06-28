pub fn get_mime_index(path: &str) -> Result<usize, i32> {
    return match path.chars().skip(1).position(|x| x == '.') {
        Some(inx) => Ok(inx),
        None => Err(-1),
    };
}
