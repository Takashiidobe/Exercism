use itertools::Itertools;

pub fn encode(source: &str) -> String {
    let ans = source.chars().dedup_with_count();
    let mut s = String::default();
    dbg!(&ans);
    for (count, c) in ans {
        dbg!(&s);
        if count == 1 {
            s.push(c);
        } else {
            s.push_str(&format!("{}{}", count, c));
        }
    }
    s
}

pub fn decode(source: &str) -> String {
    let mut s = String::default();

    let mut curr_count: u64 = 1;
    for c in source.bytes() {
        match c {
            b'0'..=b'9' => {
                let curr_num = c as u64 - b'0' as u64;
                if curr_count == 1 {
                    curr_count = curr_num;
                } else {
                    curr_count = curr_count * 10 + curr_num;
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b' ' => {
                for _ in 0..curr_count {
                    s.push(c as char);
                }
                curr_count = 1;
            }
            _ => unreachable!(),
        }
    }
    s
}
