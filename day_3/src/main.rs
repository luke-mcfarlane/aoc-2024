use library::read_file;

const MUL: &str = "mul(";
const DO: &str = "do()";
const DONT: &str = "don't()";

fn main() {
    let content = match read_file("input.txt") {
        Ok(content) => content,
        Err(error) => panic!("Problem reading the file: {error:?}"),
    };

    let mut i = 0;
    let mut values: Vec<u32> = Vec::new();
    let mut values_two: Vec<u32> = Vec::new();
    let mut disabled = false;

    while i < content.len() - 8 {
        if &content[i..i+4] == DO { disabled = false; i += 4; continue; }
        if &content[i..i+7] == DONT { disabled = true; i += 7; continue; }
        if &content[i..i+4] != MUL { i+= 1; continue; }

        i += 4;
        let mut j = i;
        let mut current: Vec<u32> = Vec::new();

        while j < content.len() {
            if content.chars().nth(j).expect("").is_digit(10) { j+= 1; continue; }

            if content.chars().nth(j) == Some(',') && j > i && current.len() == 0 {
                match content[i..j].parse() {
                    Ok(value) => {
                        current.push(value);
                        j += 1;
                        i = j;
                        continue;
                    },
                    Err(_) => { break; }
                }
            }

            if content.chars().nth(j) == Some(')') && j > i && current.len() == 1 {
                match content[i..j].parse() {
                    Ok(value) => {
                        current.push(value);

                        let result = current[0] * current[1];

                        values.push(result);
                        if !disabled { values_two.push(result); }

                        j += 1;
                        i = j;
                        break;
                    },
                    Err(_) => { break; }
                }
                
            }

            break;
        }
    }

    println!("values: {}", values.into_iter().sum::<u32>());
    println!("values part 2: {}", values_two.into_iter().sum::<u32>());
}