import { SequenceStepDto } from '@/main/domain/dtos/sequence';
import { Nullable } from '@/shared/types/utils';
import { create } from 'zustand';

export type SequenceFormState = {
  open: boolean;
  setOpen: (open: boolean) => void;
  value: Nullable<SequenceStepDto>;
  setValue: (value: Nullable<SequenceStepDto>) => void;
  currentProfileId: Nullable<number>;
  setCurrentProfileId: (id: number) => void;
  currentMacroId: Nullable<number>;
  setCurrentMacroId: (id: number) => void;
  sequence: SequenceStepDto[];
  setSequence: (sequence: SequenceStepDto[]) => void;
  operation: 'create' | 'update';
};

const useSequenceFormStore = create<SequenceFormState>((set) => ({
  open: false,
  setOpen: (open) => set({ open }),
  value: null,
  setValue: (value) => set({ value, operation: value ? 'update' : 'create' }),
  currentProfileId: null,
  setCurrentProfileId: (id) => set({ currentProfileId: id }),
  currentMacroId: null,
  setCurrentMacroId: (id) => set({ currentMacroId: id }),
  sequence: [],
  setSequence: (sequence) => set({ sequence }),
  operation: 'create',
}));

export const useSequenceFormState = () => {
  const open = useSequenceFormStore((state) => state.open);
  const value = useSequenceFormStore((state) => state.value);
  const operation = useSequenceFormStore((state) => state.operation);
  const currentProfileId = useSequenceFormStore(
    (state) => state.currentProfileId
  );
  const currentMacroId = useSequenceFormStore((state) => state.currentMacroId);
  const sequence = useSequenceFormStore((state) => state.sequence);
  return { open, value, operation, currentProfileId, currentMacroId, sequence };
};

export const useSequenceFormActions = () => {
  const setOpen = useSequenceFormStore((state) => state.setOpen);
  const setValue = useSequenceFormStore((state) => state.setValue);
  const setCurrentProfileId = useSequenceFormStore(
    (state) => state.setCurrentProfileId
  );
  const setCurrentMacroId = useSequenceFormStore(
    (state) => state.setCurrentMacroId
  );
  const setSequence = useSequenceFormStore((state) => state.setSequence);

  return {
    setOpen,
    setValue,
    setCurrentProfileId,
    setCurrentMacroId,
    setSequence,
  };
};
