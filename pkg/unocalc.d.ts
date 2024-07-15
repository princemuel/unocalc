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
*/
  constructor();
/**
*/
  calculate(): void;
/**
* @param {string} value
*/
  set_digit(value: string): void;
/**
* @param {Operation} operation
*/
  set_operation(operation: Operation): void;
/**
*/
  backspace(): void;
/**
*/
  reset(): void;
/**
* @returns {string}
*/
  get_current_value(): string;
/**
* @returns {string}
*/
  get_input_buffer(): string;
}
