use std::ops::Range;
use std::slice;

#[aoc(day2, part2)]
pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    let mut numbers = [0; 8];
    let start_pointer = numbers.as_mut_ptr();
    let mut pointer = start_pointer;
    let mut num: u8 = 0;
    unsafe {
        for char in input.bytes() {
            if char == b' ' {
                *pointer = num;
                pointer = pointer.add(1);
                num = 0;
            } else if char == b'\n' {
                *pointer = num;
                res += is_valid_line(
                    &numbers[..(pointer.offset_from(start_pointer) + 1) as usize],
                    true
                ) as u32;
                pointer = start_pointer;
                num = 0;
            } else {
                num = num * 10 + char - 48u8
            }
        }
        *pointer = num;
        res += is_valid_line(
            &numbers[..(pointer.offset_from(start_pointer) + 1) as usize],
            true
        ) as u32;
    }
    res
}

fn is_inc(numbers: &[u8]) -> (bool, bool) {
    let amount_of_inc = [numbers[0] < numbers[1], numbers[1] < numbers[2], numbers[2] < numbers[3]].iter().filter(|x| **x).count();
    let amount_of_dec = [numbers[0] > numbers[1], numbers[1] > numbers[2], numbers[2] > numbers[3]].iter().filter(|x| **x).count();
    return (amount_of_inc > 1 || amount_of_dec > 1, amount_of_inc > 1);
}
fn is_valid_line(numbers: &[u8], can_skip: bool) -> bool {
    let (valid, increasing) = is_inc(numbers);
    if !valid {
        return false;
    }
    if increasing {
        check_all_numbers::<true>(numbers, can_skip)
    } else {
        check_all_numbers::<false>(numbers, can_skip)
    }
}

fn check_all_numbers<const INCREASING: bool>(numbers: &[u8], mut can_skip: bool) -> bool {
    let mut prev = numbers[0];
    let iter = &mut numbers.iter().enumerate().skip(1);
    loop {
        let (i, cur) = if let Some(x) = iter.next() { x } else { break };
        let cur = *cur;
        if !is_valid::<INCREASING>(prev, cur) {
            if can_skip {
                if i + 1 == numbers.len() {
                    return true
                };
                can_skip = false;
                if is_valid::<INCREASING>(prev, numbers[i + 1]) || ((i == 1 || is_valid::<INCREASING>(numbers[i - 2], cur))
                    && is_valid::<INCREASING>(cur, numbers[i + 1])) {
                    prev = numbers[i + 1];
                    iter.next();
                    continue;
                }
            }
            return false;
        }
        prev = cur;
    }
    true
}

#[inline(always)]
fn is_valid<const INCREASING: bool>(prev: u8, cur: u8) -> bool {
    if INCREASING { prev < cur && cur - prev < 4 } else { cur < prev && prev - cur < 4 }
}
