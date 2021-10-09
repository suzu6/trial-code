// you can write to stdout for debugging purposes, e.g.
// console.log('this is a debug message');

function solution(N) {
  // write your code in JavaScript (Node.js 8.9.4)
  if (N < 5) return 0;
  const bin = toBinaryStrings(N);
  const result = calcMaxGap(bin);

  return result;
}

const toBinaryStrings = (N) => {
  return N.toString(2);
};

const calcMaxGap = (bin) => {
  let max_gap = 0;
  let pre1 = 0;
  for (const i in bin) {
    if (bin[i] === "1") {
      max_gap = Math.max(i - pre1 - 1, max_gap);
      pre1 = i;
    }
  }
  return max_gap;
};

module.exports = { solution, toBinaryStrings, calcMaxGap };
