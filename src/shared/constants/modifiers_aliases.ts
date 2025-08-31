import { ModifierKey } from "@/shared/bindings/ModifierKey";

export const MODIFIER_ALIASES: Record<string, ModifierKey> = {
  Ctrl: "Ctrl",
  Alt: "Alt",
  Shift: "Shift",
  Meta: "Meta",
  Control: "Ctrl",
} as const;

export const MODIFIER_LIST = Object.keys(MODIFIER_ALIASES);