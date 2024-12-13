use library::read_file;
use library::get_values;

fn main() {
    let content = match read_file("input.txt") {
        Ok(content) => content,
        Err(error) => panic!("Problem reading the file: {error:?}"),
    };

    let mut safe_num = 0;
    let mut safe_damp_num = 0;

    for line in content.lines() {
        let mut safe = [true, true];
        let mut safe_damp = [true, true];

        let values = match get_values(line) {
            Ok(split) => split,
            Err(error) => panic!("Problem getting values: {error:?}"),
        };

        for increasing in [0, 1] {
            let mut used_damp = false;
            let mut i = 1;

            while i < values.len() {
                if !check_safe(increasing == 1, values[i-1], values[i]) {
                    safe[increasing] = false;

                    if used_damp { safe_damp[increasing] = false; }

                    if i < values.len() - 1 && !check_safe(increasing == 1, values[i-1], values[i+1]) {
                        safe_damp[increasing] = false;
                    }

                    used_damp = true;
                    i += 1;
                }

                i += 1;
            }
        }

        if !safe[0] && !safe[1] { println!("{:?} {:?} {:?}", values, safe, safe_damp); }
        
        
        if safe[0] || safe[1] { safe_num += 1; }
        if safe_damp[0] || safe_damp[1] { safe_damp_num += 1;  }
    }

    println!("Safe reports: {}", safe_num);
    println!("Safe with dampening: {}", safe_damp_num);
}

fn check_safe(increasing: bool, a: u32, b: u32) -> bool {
    if increasing && a >= b { return false; }
    if !increasing && a <= b { return false; }
    if a.abs_diff(b) > 3 { return false; }

    return true;
}

// [1, 2, 6, 4, 5] [false, false] [false, false]