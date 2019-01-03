function rotateMatrix(matrix) {
  let len = matrix.length;
  for (let layer = 0; layer < len / 2; layer++) {
    let first = layer;
    let last = len - 1 - layer;
    for (let i = first; i < last; i++) {
      let offset = i - first;
      let temp = matrix[first][i];

      matrix[first][i] = matrix[last - offset][first];
      matrix[last - offset][first] = matrix[last][last - offset];
      matrix[last][last - offset] = matrix[i][last];
      matrix[i][last] = temp;
    }
  }
  return matrix;
}
