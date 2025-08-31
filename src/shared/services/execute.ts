import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { fromPromise } from "neverthrow";

export function execute<TResult, TData extends InvokeArgs = InvokeArgs>(
  command: string,
  data?: TData
) {
  return fromPromise<TResult, string>(invoke(command, data), (e) => String(e));
}
