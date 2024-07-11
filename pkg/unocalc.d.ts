/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Operation {
  Add = 0,
  Subtract = 1,
  Multiply = 2,
  Divide = 3,
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
* @returns {boolean}
*/
  has_decimal(): boolean;
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
/**
*/
  current_operation?: Operation;
/**
*/
  current_value: number;
/**
*/
  decimal_place: number;
/**
*/
  stored_value?: number;
}
