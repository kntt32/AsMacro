/// Convert str to Integer
/// # Example
/// ```
/// use util::functions::stoi;
/// assert_eq!(1328, stoi("1328").unwrap());
/// assert_eq!(0xa639f3e, stoi("0xa639f3e").unwrap());
/// assert_eq!(0b101101110101010, stoi("0b101101110101010").unwrap());
/// assert_eq!(0o116672, stoi("0o116672").unwrap());
/// ```
pub fn stoi(s: &str) -> Option<usize> {
    const stoi_func: [fn(&str) -> Option<usize>; 5] =
        [stoi_minus, stoi_octal, stoi_decimal, stoi_hex, stoi_binary];

    for f in stoi_func {
        if let Some(n) = f(s) {
            return Some(n);
        }
    }

    None
}

fn remove_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if let Some(v) = s.split_at_checked(prefix.len()) {
        if v.0 == prefix {
            Some(v.1)
        } else {
            None
        }
    } else {
        None
    }
}

fn stoi_helper(s: &str, n: &[char]) -> Option<usize> {
    let mut num: usize = 0;

    for c in s.chars().map(|c| c.to_ascii_lowercase()) {
        let mut match_flag = false;
        if let Some(muln) = num.checked_mul(n.len()) {
            num = muln;
        } else {
            return None;
        }

        for i in 0..n.len() {
            if c == n[i] {
                num += i;
                match_flag = true;
                break;
            }
        }
        if !match_flag {
            return None;
        }
    }

    Some(num)
}

fn stoi_minus(s: &str) -> Option<usize> {
    stoi(remove_prefix(s, "-")?.trim())
}

/// Binary to Integer
pub fn stoi_binary(s: &str) -> Option<usize> {
    stoi_helper(remove_prefix(s, "0b")?, &['0', '1'])
}

/// Octal to Integer
pub fn stoi_octal(s: &str) -> Option<usize> {
    stoi_helper(
        remove_prefix(s, "0o")?,
        &['0', '1', '2', '3', '4', '5', '6', '7'],
    )
}

/// Decimal to Integer
pub fn stoi_decimal(s: &str) -> Option<usize> {
    stoi_helper(s, &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

/// Hex to Integer
pub fn stoi_hex(s: &str) -> Option<usize> {
    stoi_helper(
        remove_prefix(s, "0x")?,
        &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ],
    )
}

/// Matching string
/// # Example
/// ```
/// use util::functions::*;
/// let matching = [MatchStr::Char('['), MatchStr::Str("A"), MatchStr::Number, MatchStr::Char(']')];
/// assert_eq!(
///     Some(vec!["[", "A", "123", "]"]),
///     match_str("[ A 123]", &matching),
/// );
/// ```
pub fn match_str<'a>(mut s: &'a str, rule: &[MatchStr<'_>]) -> Option<Vec<&'a str>> {
    let mut results = Vec::new();

    fn match_helper<'b>(
        s: &'b str,
        next_rule: Option<&MatchStr<'_>>,
        matching_fn: impl Fn(&str) -> bool,
    ) -> Option<(&'b str, &'b str)> {
        let mut left = s.split_ascii_whitespace().next().or(Some("")).unwrap();

        if let Some(MatchStr::Char(c)) = next_rule {
            left = s.split(*c).next()?;
        }
        left = left.trim();

        if matching_fn(left) {
            Some((left, s.split_at(left.len()).1))
        } else {
            None
        }
    }

    for i in 0..rule.len() {
        s = s.trim();

        match rule[i] {
            MatchStr::Number => {
                let (left, right) = match_helper(s, rule.get(i + 1), |s| stoi(s).is_some())?;
                results.push(left);
                s = right;
            }
            MatchStr::Str(matching_s) => {
                let (left, right) = match_helper(s, rule.get(i + 1), |s| s == matching_s)?;
                results.push(left);
                s = right;
            }
            MatchStr::Char(matching_c) => {
                let s_split = s.split_at_checked(matching_c.len_utf8())?;
                if s_split.0.chars().next()? != matching_c {
                    return None;
                }
                results.push(s_split.0);
                s = s_split.1;
            }
            MatchStr::Custom(matching_fn) => {
                let (left, right) = match_helper(s, rule.get(i + 1), matching_fn)?;
                results.push(left);
                s = right;
            }
        }
    }
    Some(results)
}

/// Matching Rule for util::functions::match_str
pub enum MatchStr<'a> {
    Number,
    Str(&'a str),
    Char(char),
    Custom(fn(&str) -> bool),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn stoi_test() {
        assert_eq!(123, stoi("123").unwrap());
        assert_eq!(0xfe, stoi("0xfe").unwrap());
        assert_eq!(0b101, stoi("0b101").unwrap());
        assert_eq!(0o132, stoi("0o132").unwrap());
    }

    #[test]
    pub fn match_str_test() {
        fn is_reg64(s: &str) -> bool {
            s == "rbp" || s == "rdi"
        }
        assert_eq!(
            Some(vec!["[", "rbp", "+", "rdi", "*", "2", "]"]),
            match_str(
                &"[ rbp + rdi * 2 ]",
                &[
                    MatchStr::Char('['),
                    MatchStr::Str(&"rbp"),
                    MatchStr::Char('+'),
                    MatchStr::Str(&"rdi"),
                    MatchStr::Char('*'),
                    MatchStr::Number,
                    MatchStr::Char(']'),
                ],
            )
        );
    }
}