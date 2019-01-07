function palindromePermutation(s) {
  const counts = {};

  for (const c of s) {
    counts[c] = (counts[c] || 0) + 1;
  }
  
  const odds = Object.values(counts).filter(v => v % 2 !== 0).length;  

  return s.length % 2 === 0 ? odds === 0 : odds === 1;
}
