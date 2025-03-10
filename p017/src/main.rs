use std::collections::HashMap;

const PAIRS: &[(u32, &'static str)] = &[
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (30, "thirty"),
    (40, "forty"),
    (50, "fifty"),
    (60, "sixty"),
    (70, "seventy"),
    (80, "eighty"),
    (90, "ninety"),
];

fn slen(hm: &HashMap<u32, u32>, n: u32) -> u32 {
    let mut acc = 0;
    let mut j = n;
    if j >= 1000 {
        let k = j / 1000;
        acc += 8; // thousand
        match hm.get(&k) {
            None => panic!("miss {}", k),
            Some(&l) => acc += l, // "one"
        }
        j -= k * 1000;
    }
    if j >= 100 {
        let k = j / 100;
        acc += 7; // "hundred"
        if j % 100 != 0 {
            acc += 3; // "and"
        }
        match hm.get(&k) {
            None => panic!("miss {}", k),
            Some(&l) => acc += l, // "one"
        }
        j -= k * 100;
    }
    if j >= 20 {
        let k = j - (j % 10); // 44 -> 40
        match hm.get(&k) {
            None => panic!("miss {}", k),
            Some(&l) => acc += l,
        }
        j -= k;
    }
    if j >= 1 {
        match hm.get(&j) {
            None => panic!("miss {}", j),
            Some(&k) => acc += k,
        }
    }
    return acc;
}

fn build_map() -> HashMap<u32, u32> {
    let mut hm: HashMap<u32, u32> = HashMap::new();
    for (n, s) in PAIRS {
        hm.insert(*n, s.len() as u32);
    }
    return hm;
}

fn compute(n: u32) -> u32 {
    let mut acc: u32 = 0;
    let hm = build_map();

    for i in 1..n+1 {
        acc += slen(&hm, i);
    }

    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(compute(5), 19);
    }

    #[test]
    fn test_slen() {
        let hm = build_map();
        assert_eq!(slen(&hm, 342), 23);
        assert_eq!(slen(&hm, 115), 20);
        assert_eq!(slen(&hm, 1000), 11);
        assert_eq!(slen(&hm, 99), 10);
        assert_eq!(slen(&hm, 199), 23);
        assert_eq!(slen(&hm, 100), 10);
    }
}

fn main() {
    println!("{}", compute(1000));
}
