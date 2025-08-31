import { FunctionKey } from "@/shared/bindings/Key";

export const FUNCTION_ALIASES: Record<string, FunctionKey> = {
  F1: "F1",
  F2: "F2",
  F3: "F3",
  F4: "F4",
  F5: "F5",
  F6: "F6",
  F7: "F7",
  F8: "F8",
  F9: "F9",
  F10: "F10",
  F11: "F11",
  F12: "F12",
} as const;

export const FUNCTION_LIST = Object.values(FUNCTION_ALIASES);