import { FC, ReactElement } from "react";
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function isElement<T extends FC<any>>(element: ReactElement, type: T): element is ReactElement<Parameters<T>[0]> {
  return element.type === type;
}
