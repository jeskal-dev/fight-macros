import { Nullable } from "@/shared/types/utils";

export function nullable<T>(value?: T | undefined | null): Nullable<T> {
  return value ?? null;
}

export function isNullable<T>(value: Nullable<T>): value is T {
  return value !== null;
}
