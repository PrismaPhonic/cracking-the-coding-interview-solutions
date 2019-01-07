//# Palindrome Permutation:
//`palindrome_permuation` is a crate that solves problem 1.4 of Cracking the Coding Interview
//book.  Here's the problem:
//!
//Given a string, write a function to check if it is a permutation of a palindrome.
//A palindrome is a word or phrase that is the same forwards and backwards. A permutation
//is a rearrangement of letters. The palindrome does not need to be limited to just dictionary
//words.
//!
//# Example
//**Input:** Tact Coa
//!
//**Output:** True (permutations: "taco cat", "atco cta", etc.)

use std::collections::HashMap;

/// Checks if the input string `s` is a permutation of a palindrome
///
/// # Examples
///
/// ```rust
/// let s = "Tact Coa";
///
/// assert_eq!(palindrome_permutation::palindrome_permutation(s), true);
/// ```
pub fn palindrome_permutation(s: &str) -> bool {
    let mut freq = HashMap::new();
    let mut count = 0;
    for c in s.to_lowercase().chars() {
        if c != ' ' {
            count += 1;
            *freq.entry(c).or_insert(0) += 1;
        }
    }

    let odds = freq.values().filter(|v| *v % 2 != 0).count();

    if count % 2 == 0 { odds == 0 } else { odds == 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(palindrome_permutation("Tact Coa"), true);
    }

    #[test]
    fn test_1() {
        assert_eq!(palindrome_permutation("aabb"), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(palindrome_permutation("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaabc"), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(palindrome_permutation("abbcabb"), true);
    }
    
    #[test]
    fn test_4() {
        assert_eq!(palindrome_permutation("zyyzzzzz"), true);
    }

    #[test]
    fn test_5() {
        assert_eq!(palindrome_permutation("z"), true);
    }

    #[test]
    fn test_6() {
        assert_eq!(palindrome_permutation("zaa"), true);
    }
}
