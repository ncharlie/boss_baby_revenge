pub fn solve(input: &str) -> &str {
    let mut it = input.bytes();

    // check initial shot
    if it.next().unwrap() == b'R' {
        return "Bad boy";
    }
    
    let mut last_elem_is_r = false;
    let mut count = 1;

    for ch in it {
        if ch == b'S' {
            if last_elem_is_r {     // new group detected
                if count > 0 {      // validate last group
                    return "Bad boy";
                }
                count = 0;
                last_elem_is_r = false;
            }
            count = count + 1;      // add for shot taken
        } else {
            count = count - 1;      // subtract for retaliate shot
            last_elem_is_r = true;
        }
    }

    // check last group
    if count > 0 {
        "Bad boy"
    } else {
        "Good boy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("SRR"), "Good boy");
    }

    #[test]
    fn test_table_solve() {
        let cases = [
            ("S", "Bad boy"),
            ("R", "Bad boy"), 
            ("SR", "Good boy"),
            ("RS", "Bad boy"),
            ("SSSSSS", "Bad boy"),
            ("RRRRRR", "Bad boy"),
            ("SRSSRRR", "Good boy"),
            ("RSSRR", "Bad boy"),
            ("SSSRRRRS", "Bad boy"),
            ("SRSR", "Good boy"),
            ("SSSSRRRR", "Good boy"),
            ("SSRRRSSR", "Bad boy"),
        ];
        for &(input, expected) in &cases {
            let result = solve(input);
            assert_eq!(result, expected, "input: {}, wanted: {}, got: {}", input, expected, result);
        }
    }
}
