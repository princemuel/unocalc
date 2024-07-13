/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Operation {
  Add = 1,
  Subtract = 2,
  Multiply = 3,
  Divide = 4,
}
/**
*/
export class Calculator {
  free(): void;
/**
* @returns {Calculator}
*/
  static new(): Calculator;
/**
*/
  reset(): void;
/**
*/
  backspace(): void;
/**
* @param {string} value
*/
  input_digit(value: string): void;
/**
* @param {Operation} operation
*/
  set_operation(operation: Operation): void;
/**
*/
  calculate(): void;
/**
* @returns {number}
*/
  current_value(): number;
/**
* @returns {Operation | undefined}
*/
  current_operation(): Operation | undefined;
/**
* @returns {string}
*/
  input_buffer(): string;
/**
* @returns {(HistoryEntry)[]}
*/
  history(): (HistoryEntry)[];
}
/**
*/
export class HistoryEntry {
  free(): void;
}
