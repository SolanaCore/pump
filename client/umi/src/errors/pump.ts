/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import { Program, ProgramError } from '@metaplex-foundation/umi';

type ProgramErrorConstructor = new (
  program: Program,
  cause?: Error
) => ProgramError;
const codeToErrorMap: Map<number, ProgramErrorConstructor> = new Map();
const nameToErrorMap: Map<string, ProgramErrorConstructor> = new Map();

/** OverflowDetected: Overflow detected */
export class OverflowDetectedError extends ProgramError {
  override readonly name: string = 'OverflowDetected';

  readonly code: number = 0x1770; // 6000

  constructor(program: Program, cause?: Error) {
    super('Overflow detected', program, cause);
  }
}
codeToErrorMap.set(0x1770, OverflowDetectedError);
nameToErrorMap.set('OverflowDetected', OverflowDetectedError);

/** UnderflowDetected: Underflow detected */
export class UnderflowDetectedError extends ProgramError {
  override readonly name: string = 'UnderflowDetected';

  readonly code: number = 0x1771; // 6001

  constructor(program: Program, cause?: Error) {
    super('Underflow detected', program, cause);
  }
}
codeToErrorMap.set(0x1771, UnderflowDetectedError);
nameToErrorMap.set('UnderflowDetected', UnderflowDetectedError);

/** InvalidTokenAmount: the token amount can't be zero */
export class InvalidTokenAmountError extends ProgramError {
  override readonly name: string = 'InvalidTokenAmount';

  readonly code: number = 0x1772; // 6002

  constructor(program: Program, cause?: Error) {
    super("the token amount can't be zero", program, cause);
  }
}
codeToErrorMap.set(0x1772, InvalidTokenAmountError);
nameToErrorMap.set('InvalidTokenAmount', InvalidTokenAmountError);

/** InvalidSolAmount: the sol amount can't be zero */
export class InvalidSolAmountError extends ProgramError {
  override readonly name: string = 'InvalidSolAmount';

  readonly code: number = 0x1773; // 6003

  constructor(program: Program, cause?: Error) {
    super("the sol amount can't be zero", program, cause);
  }
}
codeToErrorMap.set(0x1773, InvalidSolAmountError);
nameToErrorMap.set('InvalidSolAmount', InvalidSolAmountError);

/** InvalidInputs: Invalis Inputs check the that either name, ticker, uri or description are not empty */
export class InvalidInputsError extends ProgramError {
  override readonly name: string = 'InvalidInputs';

  readonly code: number = 0x1774; // 6004

  constructor(program: Program, cause?: Error) {
    super(
      'Invalis Inputs check the that either name, ticker, uri or description are not empty',
      program,
      cause
    );
  }
}
codeToErrorMap.set(0x1774, InvalidInputsError);
nameToErrorMap.set('InvalidInputs', InvalidInputsError);

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 */
export function getPumpErrorFromCode(
  code: number,
  program: Program,
  cause?: Error
): ProgramError | null {
  const constructor = codeToErrorMap.get(code);
  return constructor ? new constructor(program, cause) : null;
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 */
export function getPumpErrorFromName(
  name: string,
  program: Program,
  cause?: Error
): ProgramError | null {
  const constructor = nameToErrorMap.get(name);
  return constructor ? new constructor(program, cause) : null;
}
