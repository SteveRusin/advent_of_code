use std::{ fs, path };

fn main() {
    // let path_to_file = path::Path::new("test-input.txt");
    let path_to_file = path::Path::new("input.txt");
    let content = fs::read_to_string(path_to_file).expect("Error while reading file");

    let mut res: Vec<_> = content
        .split("\n\n")
        .map(|cals| {
            let elvs_cals: Vec<i32> = cals
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    return s.parse::<i32>().unwrap();
                })
                .collect();

            let sum: i32 = elvs_cals.iter().sum();
            return sum;
        })
        .collect();

    res.sort();
    res.reverse();

    let _x: i32 = dbg!(res.iter().take(3).sum());
}
