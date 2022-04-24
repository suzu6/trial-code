/**
 * 数式から逆ポーランド記法への変換と計算
 */

const tests = [
  {
    formula: "2 + 3",
    rpn: "2 3 +",
    result: 5,
  },
  {
    formula: "5 + 3 * 2",
    rpn: "5 3 2 * +",
    result: 11,
  },
  {
    formula: "6 / 2 * 3",
    rpn: "6 2 3 * /",
    rpn2: "6 2 / 3 *",
    result: 9,
  },
  {
    formula: "5 * 4 + 2 * 3",
    rpn: "5 4 * 2 3 * +",
    result: 26,
  },
];

const main = (tests) => {
  for (const [index, test] of Object.entries(tests)) {
    console.log(`test${index}`);
    // 数式から逆ポーランド記法へ変換
    const rpn = RPN.from(test.formula);
    console.log(" ", rpn === test.rpn, rpn);

    // 逆ポーランド記法から計算
    const result = RPN.calculate(test.rpn);
    console.log(" ", result === test.result, result);
  }
};

class RPN {
  static from(formula) {
    const tokens = this.tokenaizer(formula);
    return this.sorter(tokens).join(" ");
  }

  static tokenaizer = (str) => {
    return str.split(" ");
  };

  static sorter = (tokens) => {
    let stack = [];
    let result = [];
    tokens.map((token) => {
      [stack, result] = this.syntax(token, stack, result);
    });
    // 残った演算子を追加
    while (stack.length > 0) {
      result.push(stack.pop());
    }
    return result;
  };

  static syntax = (token, stack, result) => {
    if (Operater.isTimeOrDiv(token)) {
      stack.push(token);
    } else if (Operater.isAddOrSub(token)) {
      if (Operater.isTimeOrDiv(stack[stack.length - 1])) {
        result.push(stack.pop());
      }
      stack.push(token);
    } else {
      // 数値
      result.push(token);
    }
    return [stack, result];
  };

  /**
   * 逆ポーランド記法の文字列から計算結果を返す
   * @param {string} rpn 逆ポーランド記法の文字列
   * @returns {number}
   */
  static calculate(rpn) {
    const tokens = this.tokenaizer(rpn);
    let stack = [];
    let result = 0;
    for (const token of tokens) {
      [stack, result] = this.calc(token, stack, result);
    }
    return result;
  }

  static calc = (token, stack, result) => {
    if (!Operater.isOperater(token)) {
      // 数値
      stack.push(parseInt(token));
    } else {
      const [b, a] = [stack.pop(), stack.pop()];
      result = Operater.calculate(a, b, token);
      stack.push(parseInt(result));
    }
    return [stack, result];
  };
}

class Operater {
  static isAddOrSub(str) {
    return str === "+" || str === "-";
  }
  static isTimeOrDiv(str) {
    return str === "*" || str === "/";
  }

  static isOperater(str) {
    return this.isAddOrSub(str) || this.isTimeOrDiv(str);
  }

  static calculate(a, b, o) {
    switch (o) {
      case "*":
        return this.times(a, b);
      case "/":
        return this.div(a, b);
      case "+":
        return this.add(a, b);
      case "-":
        return this.sub(a, b);
      default:
        throw Error("無効なオペレーター");
    }
  }

  static add(a, b) {
    return a + b;
  }
  static sub(a, b) {
    return a - b;
  }
  static div(a, b) {
    return a / b;
  }
  static times(a, b) {
    return a * b;
  }
}

main(tests);
