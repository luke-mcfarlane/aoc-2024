use library::read_file;

fn main() {
    let content = match read_file("input.txt") {
        Ok(content) => content,
        Err(error) => panic!("Problem reading the file: {error:?}"),
    };

    const MUL: &str = "mul(";

    let mut i = 0;
    let mut values: Vec<u32> = Vec::new(); 

    while i < content.len() - 8 {
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
                        values.push(current[0] * current[1]);
                    },
                    Err(_) => { break; }
                }
                
            }

            break;
        }
    }

    println!("{:?}", values.into_iter().reduce(|acc, e| acc + e).unwrap());
}

// mul(x,y)