pub struct Pair(i32, i32);

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        // value is `int-int`
        let splt: Vec<&str> = value.split('-').collect();
        let a: i32 = splt[0].parse().unwrap();
        let b: i32 = splt[1].parse().unwrap();

        return Pair(a, b);
    }
}

pub fn is_fully_containing(a: &Pair, b: &Pair) -> bool {
    return (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1);
}

pub fn is_partially_overlap(a: &Pair, b: &Pair) -> bool {
    return (a.0 <= b.0 && b.0 <= a.1) || (b.0 <= a.0 && a.0 <= b.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_contain_a() {
        let a = Pair(3, 7);
        let b = Pair(2, 8);

        let res = is_fully_containing(&a, &b);

        assert_eq!(res, true);
    }
    #[test]
    fn it_should_contain_b() {
        let a = Pair(2, 8);
        let b = Pair(3, 7);

        let res = is_fully_containing(&a, &b);

        assert_eq!(res, true);
    }

    #[test]
    fn it_should_not_contain_b() {
        let a = Pair(2, 8);
        let b = Pair(3, 10);

        let res = is_fully_containing(&a, &b);

        assert_eq!(res, false);
    }

    #[test]
    fn it_should_not_contain_b_2() {
        let a = Pair(2, 8);
        let b = Pair(9, 10);

        let res = is_fully_containing(&a, &b);

        assert_eq!(res, false);
    }
    #[test]
    fn it_should_partially_overlap() {
        let a = Pair(2, 8);
        let b = Pair(8, 10);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, true);
    }
    #[test]
    fn it_should_partially_overlap_2() {
        let a = Pair(2, 4);
        let b = Pair(6, 8);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, false);
    }

    #[test]
    fn it_should_partially_overlap_3() {
        let a = Pair(2, 3);
        let b = Pair(4, 5);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, false);
    }

    #[test]
    fn it_should_partially_overlap_4() {
        let a = Pair(5, 7);
        let b = Pair(7, 9);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, true);
    }

    #[test]
    fn it_should_partially_overlap_5() {
        let a = Pair(2, 8);
        let b = Pair(3, 7);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, true);
    }

    #[test]
    fn it_should_partially_overlap_6() {
        let a = Pair(6, 6);
        let b = Pair(4, 6);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, true);
    }

    #[test]
    fn it_should_partially_overlap_7() {
        let a = Pair(2, 6);
        let b = Pair(4, 8);

        let res = is_partially_overlap(&a, &b);

        assert_eq!(res, true);
    }
}
