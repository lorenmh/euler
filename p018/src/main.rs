use std::{collections::HashMap, vec::Vec, cmp::max};

const TREE: &'static str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

fn build_tree() -> Vec<Vec<u32>> {
    let mut tree: Vec<Vec<u32>> = Vec::new();
    let mut count: u32 = 0;
    for level in TREE.split('\n') {
        let mut l: Vec<u32> = Vec::new();
        for val in level.split(' ') {
            count += 1;
            l.push(val.parse().unwrap());
        }
        tree.push(l);
    }
    println!("N = {}", count);
    return tree;
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct CacheKey(usize, usize);

fn dp(count: &mut u32, cache: &mut HashMap<CacheKey, u32>, tree: &Vec<Vec<u32>>, l: usize, i: usize) -> u32 {
    *count += 1;
    if l >= tree.len() {
        return 0;
    }
    if let Some(&result) = cache.get(&CacheKey(l, i)) {
        return result;
    }
    // use dfs so cache is warm
    let result = {
        let left = i;
        let right = i + 1;
        tree[l][i] + max(
            dp(count, cache, tree, l+1, left),
            dp(count, cache, tree, l+1, right),
        )
    };

    cache.insert(CacheKey(l, i), result);

    result
}

fn main() {
    let tree = build_tree();
    let mut count = 0;
    let mut cache: HashMap<CacheKey, u32> = HashMap::new();
    let result = dp(&mut count, &mut cache, &tree, 0, 0);
    println!("Complexity = {}", count);
    println!("Answer = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        let tree = build_tree();
        let l = tree.len() - 1;
        assert_eq!(tree[0][0], 75);
        assert_eq!(tree[1][0], 95);
        assert_eq!(tree[1][1], 64);
        assert_eq!(tree[l][tree[l].len() - 1], 23);
        assert_eq!(tree[l][tree[l].len() - 2], 4);
    }
}