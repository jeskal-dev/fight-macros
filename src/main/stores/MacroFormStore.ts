import { MacroDto } from '@/main/domain/dtos/macro';
import { Nullable } from '@/shared/types/utils';
import { create } from 'zustand';

export type MacroFormState = {
  open: boolean;
  setOpen: (open: boolean) => void;
  currentProfileId: Nullable<number>;
  setCurrentProfileId: (id: number) => void;
  value: Nullable<MacroDto>;
  setValue: (value: Nullable<MacroDto>) => void;
  operation: 'create' | 'update';
};

const useMacroFormStore = create<MacroFormState>((set) => ({
  open: false,
  setOpen: (open) => set({ open }),
  value: null,
  currentProfileId: null,
  setCurrentProfileId: (id) => set({ currentProfileId: id }),
  setValue: (value) => set({ value, operation: value ? 'update' : 'create' }),
  operation: 'create',
}));

export const useMacroFormState = () => {
  const open = useMacroFormStore((state) => state.open);
  const value = useMacroFormStore((state) => state.value);
  const operation = useMacroFormStore((state) => state.operation);
  const currentProfileId = useMacroFormStore((state) => state.currentProfileId);
  return { open, value, operation, currentProfileId };
};

export const useMacroFormActions = () => {
  const setOpen = useMacroFormStore((state) => state.setOpen);
  const setValue = useMacroFormStore((state) => state.setValue);
  const setCurrentProfileId = useMacroFormStore(
    (state) => state.setCurrentProfileId
  );
  return { setOpen, setValue, setCurrentProfileId };
};
