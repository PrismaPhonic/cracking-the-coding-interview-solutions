# Palindrome Permutation 

## The Problem

Given a string, write a function to check if it is a permutation of a palindrome.
A palindrome is a word or phrase that is the same forwards and backwards. A permutation
is a rearrangement of letters. The palindrome does not need to be limited to just dictionary
words.

## Example
**Input:** Tact Coa

**Output:** True (permutations: "taco cat", "atco cta", etc.)

## Javascript Solution

```javascript
function palindromePermutation(s) {
  const counts = {};

  for (const c of s) {
    counts[c] = (counts[c] || 0) + 1;
  }
  
  const odds = Object.values(counts).filter(v => v % 2 !== 0).length;  

  return s.length % 2 === 0 ? odds === 0 : odds === 1;
}
```

This solution involves setting up a basic frequency counter. We skip over any
empty characters as they will mess up our checking. Then we simply filter out
our values to get a count of our odd count values. For the chars to be arranged
into a possible palindrome all char counts have to be even for an even length
string (minus spaces), and for an odd length string we need to have exactly one
odd count char.
