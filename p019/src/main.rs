/*
You are given the following information, but you may prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*/

/// Generates months from 1900 until 2000.
/// 
/// This could be done using simple math but I'm doing this to learn about Rust structures
fn gen_days_in_months() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for year in 1900u16..2001 {
        let days_per_month: [u8; 12] = [
            // Jan
            31,
            // Feb
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
            // Mar
            31,
            // Apr
            30,
            // May
            31,
            // Jun
            30,
            // Jul
            31,
            // Aug
            31,
            // Sep
            30,
            // Oct
            31,
            // Nov
            30,
            // Dec
            31,
        ];
        vec.extend(days_per_month)
    }
    return vec;
}

fn sundays() -> u16 {
    let days_in_months = gen_days_in_months();

    // dow is 0-6; Sunday = 0, Monday = 1, ..., Saturday = 6
    // Jan 1 1900 is a Monday, therefore we start at 1
    let mut day_of_week = 1u8;

    // compute for 1900 (we don't count it)
    for days_in_month in days_in_months[0..12].iter() {
        day_of_week = (day_of_week + days_in_month) % 7;
    }

    let mut acc = 0u16; // at most 1200 days in acc (12*100), hence u16

    for days_in_month in days_in_months[12..].iter() {
        day_of_week = (day_of_week + days_in_month) % 7;
        if day_of_week == 0 {
            acc += 1;
        }
    }

    return acc;
}

fn main() {
    println!("{}", sundays());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_year_info() {
        let months = gen_days_in_months();
        assert_eq!(
            months[0..12], // 1900
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        );
        assert_eq!(
            months[12..24], // 1901
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        );
        assert_eq!(
            months[100*12..], // 2000
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        );
    }
}
