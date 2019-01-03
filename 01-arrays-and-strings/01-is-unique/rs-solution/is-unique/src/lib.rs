use std::collections::HashSet;

fn is_unique(s: String) -> bool {
    let mut occurances = HashSet::new();

    for char in s.chars() {
        if occurances.insert(char) == false {
            return false;
        }
    };

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_unique() {
        assert_eq!(is_unique("abcdefg".to_string()), true);
    }

    #[test]
    fn not_unique() {
        assert_eq!(is_unique("zefgjizl".to_string()), false);
    }
}
