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
* @returns {number}
*/
  current_value(): number;
/**
* @returns {number | undefined}
*/
  stored_value(): number | undefined;
/**
* @returns {Operation | undefined}
*/
  current_operation(): Operation | undefined;
/**
* @returns {boolean}
*/
  has_decimal(): boolean;
/**
* @returns {number}
*/
  decimal_place(): number;
/**
* @returns {number | undefined}
*/
  calculate(): number | undefined;
/**
* @param {Operation} operation
*/
  input_operation(operation: Operation): void;
/**
* @param {number} digit
*/
  input_digit(digit: number): void;
/**
*/
  input_decimal(): void;
/**
*/
  reset(): void;
/**
*/
  delete_last_digit(): void;
}
