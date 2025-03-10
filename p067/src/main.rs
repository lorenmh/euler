use std::{collections::HashMap, vec::Vec, cmp::max, fs::read_to_string};

fn build_tree() -> Vec<Vec<u32>> {
    let contents = read_to_string("./0067_triangle.txt").expect("Should have been able to read 0067_triangle.txt");
    let mut tree: Vec<Vec<u32>> = Vec::new();
    let mut count: u32 = 0;
    for level in contents.split('\n') {
        let mut l: Vec<u32> = Vec::new();
        for val in level.split(' ') {
            count += 1;
            if let Ok(v) = val.parse() {
                l.push(v);
            }
        }
        if l.len() > 0 {
            tree.push(l);
        }
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
        assert_eq!(tree[0][0], 59);
        assert_eq!(tree[1][0], 73);
        assert_eq!(tree[1][1], 41);
        assert_eq!(tree[l][tree[l].len() - 1], 35);
        assert_eq!(tree[l][tree[l].len() - 2], 63);
    }
}