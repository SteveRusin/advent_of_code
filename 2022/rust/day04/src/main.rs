use std::{ fs, path::Path };

use day04::overlap::{ is_fully_containing, Pair, is_partially_overlap };

fn main() {
    /* let file_path = "./test-input.txt"; */
    let file_path = "./input.txt";
    let file = fs::read_to_string(Path::new(file_path)).expect("Error while opening file");

    let res: usize = file
        .split("\n")
        .filter(|v| !v.is_empty())
        .map(|v| {
            let tuple: Vec<&str> = v.split(",").collect();

            let a: Pair = tuple[0].into();
            let b: Pair = tuple[1].into();

            //  return is_fully_containing(&a, &b);
            return is_partially_overlap(&a, &b);
        })
        .filter(|x| *x)
        .count();
    println!("{:#?}", res)
}
