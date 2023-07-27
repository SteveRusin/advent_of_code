use std::{ fs, path::Path };

fn main() {
    let input = "./input.txt";
    let file = fs::read_to_string(Path::new(input)).unwrap();

    //part_1(file);
    part_2(file);
}

fn part_2(file: String) {
    let res: Vec<&str> = file
        .split("\n")
        .filter(|str| !str.is_empty())
        .collect();

    let grouped: u32 = group_vec_by(&res, 3)
        .iter()
        .map(|group| find_intersection_in_group(group))
        .map(convert_to_priority)
        .sum();

    println!("{:?}", grouped);
}

fn group_vec_by<'a>(vec: &'a Vec<&'a str>, size: usize) -> Vec<Vec<&'a str>> {
    let mut j = 0;
    let mut result: Vec<Vec<&str>> = vec![];

    for (i, el) in vec.iter().enumerate() {
        if i >= size && i % size == 0 {
            j += 1;
        }

        let group = result.get_mut(j);

        match group {
            Some(gr) => gr.push(el),
            None => result.push(vec![el]),
        }
    }

    return result;
}

fn part_1(file: String) {
    let sum: u32 = file
        .split("\n")
        .filter(|str| !str.is_empty())
        .map(|row| {
            let (left, right) = split_str(row);
            return find_intersection(left, right);
        })
        .map(convert_to_priority)
        .sum();

    println!("{}", sum);
}

fn split_str(str: &str) -> (&str, &str) {
    return str.split_at(str.len() / 2);
}

fn find_intersection<'a>(str_a: &'a str, str_b: &'a str) -> char {
    let chars_b = Box::new(str_b.chars());
    let res = str_a
        .chars()
        .find(|x| {
            return chars_b
                .clone()
                .into_iter()
                .any(|el_b| {
                    return el_b == *x;
                });
        })
        .unwrap();

    return res;
}

fn convert_to_priority(ch: char) -> u32 {
    let res = ch as u32;
    if res >= 97 && res <= 122 {
        return res - 96;
    } else {
        return res - 38;
    }
}

fn find_intersection_with_indx<'a>(str_a: &'a str, str_b: &'a str) -> Option<(usize, char)> {
    let chars_b = Box::new(str_b.chars());
    let res = str_a
        .chars()
        .enumerate()
        .find(|(_, x)| {
            return chars_b
                .clone()
                .into_iter()
                .any(|el_b| {
                    return el_b == *x;
                });
        });

    return res;
}

fn find_char(str_1: &str, char: &char, indx: usize) -> Option<(usize, char)> {
    if indx >= str_1.len() {
        return None;
    }

    let res = str_1
        .chars()
        .enumerate()
        .filter(|&(ind, _)| ind >= indx)
        .find(|(_, ch)| ch == char);

    return res;
}

fn find_just_char(str_1: &str, char: &char) -> Option<char> {
    let res = str_1.chars().find(|ch| ch == char);

    return res;
}

//fn walk<'a, 'b>(ch: &'a char, rest: &'b [&str]) -> Option<&'a char> {
fn walk(ch: &char, rest: &[&str]) -> Option<char> {
    if rest.len() == 0 {
        return Some(ch.clone());
    }

    let find_res = find_just_char(rest[0], ch);

    if let Some(ch) = find_res {
        return walk(&ch, &rest[1..]);
    }

    return None;
}

fn find_intersection_in_group(group_of_str: &Vec<&str>) -> char {
    let first = group_of_str[0];

    let res: Option<Option<char>> = first
        .chars()
        .map(|ch| {
            return walk(&ch, &group_of_str[1..]);
        })
        .find(|x| x.is_some());

    return res.flatten().unwrap();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_should_split_in_half() {
        let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (first_part, second_part) = split_str(input);

        assert_eq!(first_part, "vJrwpWtwJgWr");
        assert_eq!(second_part, "hcsFMMfFFhFp");
    }

    #[test]
    fn it_should_find_intersection() {
        let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (first_part, second_part) = split_str(input);

        let res = find_intersection(first_part, second_part);
        assert_eq!(res.to_string(), "p");
    }

    #[test]
    fn it_should_convert_priority() {
        let result_map = HashMap::from([
            ('a', 1),
            ('p', 16),
            ('s', 19),
            ('t', 20),
            ('v', 22),
            ('z', 26),
            ('A', 27),
            ('L', 38),
            ('P', 42),
            ('Z', 52),
        ]);

        result_map.into_iter().for_each(|(key, val)| {
            let res = convert_to_priority(key);

            assert_eq!(res, val);
        });
    }

    #[test]
    fn it_should_identify_common_item_in_group_1() {
        let inpt_1 = "abcd";
        let inpt_2 = "dght";
        let input_3 = "otgd";

        let res = find_intersection_in_group(&vec![inpt_1, inpt_2, input_3]);

        assert_eq!(res.to_string(), "d")
    }
    #[test]
    fn it_should_identify_common_item_in_group_2() {
        let inpt_1 = "abcd";
        let inpt_2 = "cdgh";
        let input_3 = "grd";

        let res = find_intersection_in_group(&vec![inpt_1, inpt_2, input_3]);

        assert_eq!(res.to_string(), "d")
    }

    #[test]
    fn it_should_find_char_from_index_1() {
        let input = "abcde";
        let res = find_char(input, &'c', 0);

        assert_eq!(res, Some((2, 'c')));
    }

    #[test]
    fn it_should_find_char_from_index_2() {
        let input = "abcdec";
        let res = find_char(input, &'c', 3);

        assert_eq!(res, Some((5, 'c')));
    }

    #[test]
    fn it_should_find_char_with_index_1() {
        let input = "abcdec";
        let input_2 = "cfs";
        let res = find_intersection_with_indx(input, input_2);

        assert_eq!(res, Some((2, 'c')));
    }

    #[test]
    fn it_should_group_by() {
        let vec = vec!["1", "2", "3", "4", "5", "6"];

        let res = group_vec_by(&vec, 3);

        assert_eq!(res, vec![vec!["1", "2", "3"], vec!["4", "5", "6"]]);
    }
}
