use std::{vec::Vec, cmp::max};

// Find the sum of the digits in the number 100!.

fn mul(a: &Vec<u16>, b: &Vec<u16>) -> Vec<u16> {
    let m = max(a.len(), b.len()) * 2 + 1;
    let mut acc: Vec<u16> = vec![0; m];
    for i in 0..a.len() {
        for j in 0..b.len() {
            acc[j+i] += a[i] * b[j];
        }
    }
    let mut carry = 0;
    for i in 0..m {
        acc[i] += carry;
        carry = acc[i] / 10;
        acc[i] %= 10;
    }
    let mut true_len = acc.len();
    while acc[true_len-1] == 0 {
        true_len -= 1
    }
    acc.truncate(true_len);
    return acc;
}

fn main() {
    let mut factors = [1u128; 10];
    for i in 0..10 {
        for j in 0u128..10 {
            if i == 0 && j == 0 { continue };
            factors[i] *= (i as u128 * 10) + j;
        }
    }
    println!("Factors: {:?}", factors);
    let mut factors_digits = Vec::new();
    for i in 0..10 {
        let mut k = factors[i];
        let mut digits = Vec::new();
        while k != 0 {
            digits.push((k % 10) as u16);
            k /= 10;
        }
        factors_digits.push(digits);
    }
    let mut acc = vec![1_u16];
    for factor in factors_digits {
        acc = mul(&factor, &acc);
    }
    acc.reverse();
    let result: u16 = acc.iter().sum();
    println!("Result: {}", acc.iter().map(|n| n.to_string()).collect::<String>());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        assert_eq!(
            mul(
                &mut vec![1,2,3,4],
                &mut vec![5,6,7,8],
            ),
            vec![5,6,5,3,7,8,7,3],
        );
        assert_eq!(
            mul(
                &mut vec![1,2,3,4],
                &mut vec![1],
            ),
            vec![1,2,3,4],
        );
    }
}