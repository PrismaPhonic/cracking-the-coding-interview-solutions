# Palindrome Permutation 

## The Problem

Given a string, write a function to check if it is a permutation of a palindrome.
A palindrome is a word or phrase that is the same forwards and backwards. A permutation
is a rearrangement of letters. The palindrome does not need to be limited to just dictionary
words.

## Example
**Input:** Tact Coa

**Output:** True (permutations: "taco cat", "atco cta", etc.)

## Rust Solution

```rust
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
```

This solution involves setting up a basic frequency counter. We skip over any
empty characters as they will mess up our checking. Then we simply filter out
our values to get a count of our odd count values. For the chars to be arranged
into a possible palindrome all char counter have  to be even for an even length
string (minus spaces), and for an odd length string we need to have exactly one
odd count char.
