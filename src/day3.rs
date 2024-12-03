use std::ops::Add;
use memchr::memmem;


#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { solve1(input) }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    unsafe { solve2(input) }
}

unsafe fn solve1(input: &str) -> u32 {
    let input = input.as_bytes();
    memmem::find_iter(input, b"mul(")
        .into_iter()
        .map(|pos| get_rest_of_mul(input, pos))
        .sum()
}

unsafe fn solve2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut res = 0;
    let mut on = true;
    let mut pos = 0;
    loop {
        if on && *input.get_unchecked(pos) == b'm' && *input.get_unchecked(pos + 1) == b'u' && *input.get_unchecked(pos + 2) == b'l' && *input.get_unchecked(pos + 3) == b'(' {
            // skip the rest of the characters: `ul(_,_)` (I dont bother to check how long the numbers are cuz its probably not worth the overhead)
            res += get_rest_of_mul(input, pos);
            pos += 7;
        } else if *input.get_unchecked(pos) == b'd' && *input.get_unchecked(pos + 1) == b'o' {
            if !on && *input.get_unchecked(pos + 2) == b'(' && *input.get_unchecked(pos + 3) == b')' {
                // skip the rest of the characters: `o()`
                pos += 3;
                on = true;
            } else if on && *input.get_unchecked(pos + 2) == b'n' && *input.get_unchecked(pos + 3) == b'\'' && *input.get_unchecked(pos + 4) == b't' && *input.get_unchecked(pos + 5) == b'(' && *input.get_unchecked(pos + 6) == b')'{
                // skip the rest of the characters: `on't()`
                pos += 6;
                on = false;
            }
        }
        // 8 is the minimum amount needed for a mul: mul(2,2)
        if pos > input.len() - 9 { break } else { pos += 1 };

    }
    res
}

unsafe fn get_num(input: &[u8], mut pos: usize) -> (u32, usize) {
    let mut num1 = *input.get_unchecked(pos) as u32 - 48;
    let char = *input.get_unchecked(pos + 1);
    if char > 47 && char < 58 {
        num1 = num1 * 10 + char as u32 - 48;
        let char = *input.get_unchecked(pos + 2);
        if char > 47 && char < 58 {
            return (num1 * 10 + char as u32 - 48, pos + 2)
        }
        return (num1, pos + 1)
    }
    (num1, pos)
}

unsafe fn get_rest_of_mul(input: &[u8], pos: usize) -> u32 {
    let mut pos = pos + 4;
    if *input.get_unchecked(pos) < 48 || *input.get_unchecked(pos) > 57 {
        return 0
    }
    let num1;
    (num1, pos) = get_num(input, pos);
    if *input.get_unchecked(pos + 1) != b',' { return 0 }

    let mut pos = pos + 2;
    if *input.get_unchecked(pos) < 48 || *input.get_unchecked(pos) > 57 {
        return 0
    }
    let num2;
    (num2, pos) = get_num(input, pos);
    (*input.get_unchecked(pos + 1) == b')') as u32 * num1 * num2
}