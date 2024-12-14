use library::read_file;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn main() {
    let content = match read_file("input.txt") {
        Ok(content) => content,
        Err(error) => panic!("Problem reading the file: {error:?}"),
    };

    let vectors: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        
    let length = vectors.len();

    let mut count = 0;

    for i in 0..length {
        let mut current = [0; 4];

        for j in 0..length {
            for k in 0..4 {
                let value = match k {
                    0 => vectors[i][j],
                    1 => vectors[j][i],
                    2 => vectors[i][length - j - 1],
                    3 => vectors[length - j - 1][i],
                    _ => 'Z'
                };

                if value == XMAS[current[k]] { current[k] += 1; } 
                else { current[k] = 0; }

                if current[k] == 4 { count += 1; current[k] = 0; }
            }
        }
    }

    for i in 0..2 * length - 1 {
        let mut current = [0; 4];

        for _j in 0..length - length.abs_diff(i + 1) {
            for k in 0..4 {
                let value = match k {
                    _ => 'Z'
                };

                if value == XMAS[current[k]] { current[k] += 1; } 
                else { current[k] = 0; }

                if current[k] == 4 { count += 1; current[k] = 0; }
            }
        }
    }

    println!("{}", count);
}
