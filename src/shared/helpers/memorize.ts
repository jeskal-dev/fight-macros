import { FC, memo } from "react";

export function memorize<T>(fn: T): T {
  return memo(fn as FC) as unknown as T;
}
