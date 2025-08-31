export type Nullable<T> = T | null;

type Att<T> = {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
  [K in keyof T]: T[K] extends Function ? never : K;
}[keyof T];
export type TypeOf<T> = T extends Type<infer U> ? U : never;
export type Type<T> = { [P in keyof T]: T[P] } extends infer U
  ? Pick<U, Att<U>>
  : never;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Any = any;
