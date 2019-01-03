# Is Unique

## Table of Contents
1. [Rust Solution](/rs-solution/)
2. [Javascript Solution](/js-solution/)

## The Problem

**Is Unique**: Implement an algorithm to determine if a string has all unique
characters.

## Solution

My solution for this involves creating a hashset.  We then iterate through each
character in the string and if it's not in the hashset we add it in - otherwise
we return false (found a duplicate, so the string is not 100% unique
characters).  If we finish looping through without finding a duplicate, return
true.
