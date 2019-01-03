# Rotate Matrix

## Table of Contents
1. [Rust Solution](/rs-solution/)
2. [Javascript Solution](/js-solution/)

## Problem

**Rotate Matrix**: Given an image represented by an NxN matrix, where each pixel
in the image is 4 bytes, write a method to rotate the image by 90 degrees. Do
this in place.

## Solution

The problem specifically asks to solve this in O(1) space complexity by mutating
in place. To accomplish this we start by rotating the corners, then moving one
in and so on until we've finished the outer ring of the matrix.  Once we do that
we move to the next ring of the matrix and repeat until we have no more rings
left to rotate.
