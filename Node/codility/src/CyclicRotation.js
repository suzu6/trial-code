// you can write to stdout for debugging purposes, e.g.
// console.log('this is a debug message');

function solution(A, K) {
  // write your code in JavaScript (Node.js 8.9.4)
  const n = A.length;
  if (n === 0) return A;
  const r = remainder(K, n);
  for (const _ in Array(r).fill(0)) {
    A = rotate(A);
  }
  return A;
}

const remainder = (k, n) => {
  return k % n;
};

const rotate = (array) => {
  const last = array.pop();
  array.unshift(last);
  return array;
};

module.exports = { solution, remainder, rotate };
