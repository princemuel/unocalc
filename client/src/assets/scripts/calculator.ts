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
const output = getElement("#result", HTMLOutputElement);

form.addEventListener("click", (e) => {
  const key = e.target;

  if (key instanceof HTMLButtonElement) {
    const key_type = key.dataset.type;

    if (key_type === "number") {
      const operator = (key.textContent ?? "").trim().toLowerCase();
      calculator.input_digit(operator);
    }

    if (key_type === "operator") {
      const operator = (key.textContent ?? "").trim().toLowerCase();
      const operation = operations.get(operator);
      if (operation) calculator.set_operation(operation);
    }

    if (key_type === "decimal") calculator.input_digit(".");
    if (key_type === "delete") calculator.backspace();
    if (key_type === "reset") calculator.reset();
    if (key_type === "equals") calculator.calculate();

    const result = number_guard(
      calculator.input_buffer() || calculator.current_value()
    );
    output.textContent = new Intl.NumberFormat().format(result);
  }
});
