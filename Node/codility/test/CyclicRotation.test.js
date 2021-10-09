const cr = require("../src/CyclicRotation");

describe("CyclicRotation", () => {
  it("あまり", () => {
    expect(cr.remainder(1, 3)).toBe(1);
    expect(cr.remainder(10, 5)).toBe(0);
    expect(cr.remainder(11, 5)).toBe(1);
    expect(cr.remainder(2, 5)).toBe(2);
  });

  it("配列を右へ回転", () => {
    expect(cr.rotate([1, 2, 3])).toEqual([3, 1, 2]);
  });

  it("配列を右へ回数分回転", () => {
    expect(cr.solution([], 0)).toEqual([]);
    expect(cr.solution([1, 2, 3], 0)).toEqual([1, 2, 3]);
    expect(cr.solution([1, 2, 3], 1)).toEqual([3, 1, 2]);
    expect(cr.solution([1, 2, 3], 4)).toEqual([3, 1, 2]);
    expect(cr.solution([1, 2, 3], 3)).toEqual([1, 2, 3]);
    expect(cr.solution([1, 2, 3], 2)).toEqual([2, 3, 1]);
  });
});
