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
  has_decimal: boolean;
/**
*/
  stored_value?: number;
}
