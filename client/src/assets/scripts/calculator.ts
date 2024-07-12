import { Calculator, Operation } from "unocalc";
import { getElement } from "./dom";
import { number_guard } from "./utils";

const calculator = Calculator.new();

const operations = new Map([
  ["+", Operation.Add],
  ["-", Operation.Subtract],
  ["x", Operation.Multiply],
  ["/", Operation.Divide],
]) satisfies Map<string, Operation>;

const form = getElement("#keypad", HTMLFormElement);

form.addEventListener("click", (e) => {
  const key = e.target;

  if (key instanceof HTMLButtonElement) {
    if (key.dataset.keytype === "number") {
      const operator = (key.textContent ?? "").trim().toLowerCase();
      const digit = number_guard(operator);
      calculator.input_digit(digit);
    }

    if (key.dataset.keytype === "operator") {
      const operator = (key.textContent ?? "").trim().toLowerCase();
      const operation = operations.get(operator);
      if (operation) calculator.input_operation(operation);
    }

    if (key.dataset.keytype === "decimal") calculator.input_decimal();
    if (key.dataset.keytype === "delete") calculator.delete_last_digit();
    if (key.dataset.keytype === "reset") calculator.reset();
    if (key.dataset.keytype === "equals") calculator.calculate();

    console.log("stored_value", calculator.stored_value());
    console.log("current_operation", calculator.current_operation());
    console.log("current_value", calculator.current_value());
    console.log("decimal_place", calculator.decimal_place());
  }
});

// console.log(calculator);

// const keys = getElements("[data-id]", HTMLButtonElement, form);

// function get_operator(operator: string) {
//   switch (operator) {
//     case "+":

//       break;

//     default:
//       break;
//   }
// }
