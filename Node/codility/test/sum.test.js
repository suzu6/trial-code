const sum = require("../src/sum");

describe("sum", () => {
  it("足し算", () => {
    const result = sum(1, 2);
    expect(result).toBe(3);
  });
});
