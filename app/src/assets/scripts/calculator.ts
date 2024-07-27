import { Calculator, Operation } from "unocalc";
import { getElement } from "./dom";
import { number_guard } from "./utils";

const calculator = new Calculator();

const operations = new Map([
  ["+", Operation.Add],
  ["-", Operation.Subtract],
  ["x", Operation.Multiply],
  ["/", Operation.Divide],
]) satisfies Map<string, Operation>;

const form = getElement("#keypad", HTMLFormElement);
const display = getElement("#result", HTMLOutputElement);

let expression = "";

const actions = {
  "=": function () {
    console.log(expression);
  },
  del: function () {
    expression = expression.slice(0, -1);
  },
  reset: function () {
    expression = "";
  },
  default: function (value = "") {
    if (is_valid(expression, value)) expression += value;
    console.log(expression);
  },
} as Record<string, (value?: string) => void>;

form.addEventListener("click", (e) => {
  const key = e.target;

  if (key instanceof HTMLButtonElement) {
    const key_type = key.dataset.type;
    const value = (key.value ?? "").trim().toLowerCase();
    console.log(value);

    (actions[value] || actions.default)(value);

    // if (
    //   key_type === "number" ||
    //   key_type === "decimal" ||
    //   key_type === "operator"
    // ) {
    //   actions.default(value);
    // } else if (key_type === "equals") {
    //   actions["="]();
    // } else if (key_type === "delete") {
    //   actions.del();
    // } else if (key_type === "reset") {
    //   actions.reset();
    // }

    // const result = number_guard(calculator.get_current_value());
    // output.value = new Intl.NumberFormat().format(result);
  }
});

// function is_valid(expression: string, value: string) {
//   const input_regex = /^[0-9+\-*/.]*$/;
//   // const expr_regex =
//   //   /^(\s*(?:[0-9]*\.?[0-9]+(?:\s*[-+*/]\s*[0-9]*\.?[0-9]+)*)|(?:\([^\(\)]+\))*)\s*$/;
//   //
//   const expr_regex =
//     /^((?:[0-9]*\.?[0-9]+(?:\s*[-+*/]\s*[0-9]*\.?[0-9]+)*)|(?:\([^\(\)]+\))*)\s*$/;

//   if (!input_regex.test(expression + value)) return false;
//   if (expr_regex.test(expression + value)) return false;

//   const last_number = expression.split(/[\+\-\*\/]/).pop();
//   if (value === "." && last_number?.includes(".")) return false;

//   return true;
// }
//
function is_valid(expression: string, value: string) {
  const input_regex = /^[0-9+\-*/.]*$/giu;
  // const expr_regex =
  //   /^(\d+(\.\d+)?([+\-*/]\d+(\.\d+)?)*|\(\d+(\.\d+)?([+\-*/]\d+(\.\d+)?)*\))$/;
  //
  const expr_regex =
    /^(\s*(?:[0-9]*\.?[0-9]+(?:\s*[-+*/]\s*[0-9]*\.?[0-9]+)*)|(?:\([^\(\)]+\))*)\s*$/giu;

  // Ensure the new value is valid and doesn't include illegal characters
  if (!input_regex.test(expression + value)) return false;

  // Check for valid expression formation
  if (!expr_regex.test(expression + value)) return false;

  // Check for multiple decimal points in the last number
  const last_number = expression.split(/[\+\-\*\/]/).pop() || "";
  if (value === "." && last_number.includes(".")) return false;

  return true;
}
