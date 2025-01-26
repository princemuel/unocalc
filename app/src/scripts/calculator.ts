import { calculate } from "unocalc";
import { $ } from "./dom";
import { numberGuard } from "./utils";

const form = $("#keypad", HTMLFormElement);
const display = $("#result", HTMLOutputElement);

// let test_expression = "-1.5 * ( 3 + 4 ) * 2 - 5 / 2.5 + 6 / 3";

// test_expression = "-1.5*(3+4)*2-5/2.5+6/3";

const intNFormat = (value: number) =>
  (function () {
    const options: Intl.NumberFormatOptions =
      value.toString().length > 9
        ? { notation: "scientific" }
        : { notation: "standard" };
    return new Intl.NumberFormat(navigator.language, options).format(value);
  })();

let expression = "";
let result = 0.0;

const mathKeys = new Map([
  ["decimal", "decimal"],
  ["number", "number"],
  ["operator", "operator"],
  ["parens", "parens"],
]);

form.addEventListener("click", (e) => {
  const key = e.target;

  if (key instanceof HTMLButtonElement) {
    const keyType = key.dataset.type || "";
    const keyValue = (key.value ?? "").trim().toLowerCase();

    if (mathKeys.get(keyType) && isValid(expression + keyValue)) {
      // display.value =
      expression += keyValue;
    }

    if (keyType === "equals" && isValid(expression)) {
      result = calculate(expression);
    }

    if (keyType === "delete") expression = expression.slice(0, -1);
    if (keyType === "reset") expression = "";

    display.value = intNFormat(numberGuard(result));
  }
});

function isValid(expression: string) {
  // Remove all whitespace
  const removeWhitespace = (expr: string) => expr.replace(/\s+/gu, "");

  // Check for invalid characters
  const hasInvalidChars = (expr: string) => /[^0-9+\-*/().]/gu.test(expr);

  // Check for duplicate decimals in a number
  const hasDuplicateDecimals = (expr: string) => /\d+\.\d*\.\d+/gu.test(expr);

  // Check for balanced parentheses
  const areParenthesesBalanced = (expr: string) => {
    let balance = 0;
    for (const char of expr) {
      if (char === "(") balance++;
      if (char === ")") balance--;
      if (balance < 0) return false;
    }
    return balance === 0;
  };

  // const isSyntaxValid = (expr: string) => {
  //   const operators = "+-*/";
  //   const validChars = "0123456789" + operators + "()";
  //   let lastChar = "";
  //   let prevChar = "";

  //   for (const char of expr) {
  //     // Check for invalid characters
  //     if (!validChars.includes(char)) {
  //       console.log("invalid char");
  //       return false;
  //     }

  //     // Check for invalid operator usage
  //     if (operators.includes(char)) {
  //       if (
  //         operators.includes(lastChar) ||
  //         lastChar === "" ||
  //         lastChar === "("
  //       ) {
  //         console.log("invalid operator");

  //         return false;
  //       }
  //     }

  //     // Check for invalid parentheses usage
  //     if (char === ")") {
  //       if (
  //         lastChar === "" ||
  //         operators.includes(lastChar) ||
  //         lastChar === "("
  //       ) {
  //         console.log("invalid parentheses");
  //         return false;
  //       }
  //     }

  //     if (char === "(") {
  //       if (operators.includes(lastChar) || lastChar === "") {
  //         return false;
  //       }
  //     }

  //     // Handle edge case where there could be negative numbers
  //     if (
  //       char === "-" &&
  //       (lastChar === "" || operators.includes(lastChar) || lastChar === "(")
  //     ) {
  //       // Handle valid negative numbers or unary minus
  //       if (prevChar !== "" && !operators.includes(prevChar)) {
  //         return false;
  //       }
  //     }

  //     // Update lastChar and prevChar for next iteration
  //     prevChar = lastChar;
  //     lastChar = char;
  //   }

  //   // Ensure the expression does not end with an operator
  //   return !operators.includes(lastChar);
  // };

  // Main validation function
  const expr = removeWhitespace(expression);

  // const a = !hasInvalidChars(expr);
  // const b = !hasDuplicateDecimals(expr);
  // const c = areParenthesesBalanced(expr);
  // const d = isSyntaxValid(expr);
  // console.log(a, b, c, d);

  return (
    !hasInvalidChars(expr) &&
    !hasDuplicateDecimals(expr) &&
    areParenthesesBalanced(expr)
    // && isSyntaxValid(expr)
  );
}
