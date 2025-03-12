use std::collections::{HashMap, HashSet};

/*
<p>Let $d(n)$ be defined as the sum of proper divisors of $n$ (numbers less than $n$ which divide evenly into $n$).<br>
If $d(a) = b$ and $d(b) = a$, where $a \ne b$, then $a$ and $b$ are an amicable pair and each of $a$ and $b$ are called amicable numbers.</p>
<p>For example, the proper divisors of $220$ are $1, 2, 4, 5, 10, 11, 20, 22, 44, 55$ and $110$; therefore $d(220) = 284$. The proper divisors of $284$ are $1, 2, 4, 71$ and $142$; so $d(284) = 220$.</p>
<p>Evaluate the sum of all the amicable numbers under $10000$.</p>

 */

fn get_divisors(cache: &mut HashMap<u32, HashSet<u32>>, n: u32) -> HashSet<u32> {
    if n == 2 {
        return HashSet::from([2]);
    }
    if let Some(s) = cache.get(&n) {
        return s.clone();
    }

    let mut divisors: HashSet<u32> = HashSet::new();
    let start = (n as f32).sqrt().floor() as u32 + 1;
    for i in (2..start).rev() {
        if n % i == 0 {
            let component = n / i;
            divisors.extend([i, component]);
            divisors.extend(get_divisors(cache, i));
            divisors.extend(get_divisors(cache, component));
        }
    }
    cache.insert(n, divisors.clone());

    return divisors;
}

fn get_divisors_sum(cache: &mut HashMap<u32, HashSet<u32>>, n: u32) -> u32 {
    get_divisors(cache, n).iter().map(|&i| i).sum::<u32>() + 1
}


fn main() {
    let mut acc = 0u32;
    let mut cache: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut counted: HashSet<u32> = HashSet::new();
    for i in 1u32..10000 {
        if counted.contains(&i) {
            continue;
        }
        let s1 = get_divisors_sum(&mut cache, i);
        if i == s1 {
            continue;
        }
        let s2 = get_divisors_sum(&mut cache, s1);
        if s2 == i {
            println!("{} and {}", i, s1);
            counted.insert(i);
            counted.insert(s1);
            acc += i + s1;
        }
    }
    println!("{}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_divisors() {
        let mut cache = HashMap::new();
        assert_eq!(
            get_divisors(&mut cache, 20),
            HashSet::from([2,4,5,10]),
        );
        assert_eq!(
            get_divisors(&mut cache, 220),
            HashSet::from([2,4,5,10,11,20,22,44,55,110]),
        );
        assert_eq!(
            get_divisors(&mut cache, 284),
            HashSet::from([2,4,71,142]),
        );
    }

    #[test]
    fn test_get_divisors_sum() {
        let mut cache = HashMap::new();
        assert_eq!(
            get_divisors_sum(&mut cache, 20),
            22,
        );
        assert_eq!(
            get_divisors_sum(&mut cache, 220),
            284,
        );
        assert_eq!(
            get_divisors_sum(&mut cache, 284),
            220,
        );
        assert_eq!(
            get_divisors_sum(&mut cache, 1),
            1,
        );
        assert_eq!(
            get_divisors_sum(&mut cache, 2),
            3,
        );
    }
}