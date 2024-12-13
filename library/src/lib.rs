use std::fs;
use std::error::Error;

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn get_values(line: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut vec: Vec<u32> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while j < line.len() {
        j += 1;

        if line.chars().nth(j) == Some(' ') {
            vec.push(line[i..j].parse()?);

            while line.chars().nth(j) == Some(' ') {
                j += 1;
            }

            i = j;
        }
    }

    vec.push(line[i..].parse()?);

    return Ok(vec);
}