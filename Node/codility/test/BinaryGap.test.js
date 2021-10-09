const BinaryGap = require("../src/BinaryGap");

describe("BinaryGap", () => {
  it("二進数に変換", () => {
    expect(BinaryGap.toBinaryStrings(5)).toBe("101");
    expect(BinaryGap.toBinaryStrings(51712)).toBe("1100101000000000");
    expect(BinaryGap.toBinaryStrings(0)).toBe("0");
  });

  it("連続するギャップの最大数を返す", () => {
    expect(BinaryGap.solution(0)).toBe(0);
    expect(BinaryGap.solution(51712)).toBe(2);
  });
});
