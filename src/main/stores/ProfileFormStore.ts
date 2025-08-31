import { ProfileDto } from '@/main/domain/dtos/profile';
import { Nullable } from '@/shared/types/utils';
import { create } from 'zustand';

export type ProfileFormState = {
  open: boolean;
  setOpen: (open: boolean) => void;
  value: Nullable<ProfileDto>;
  setValue: (value: Nullable<ProfileDto>) => void;
  operation: 'create' | 'update';
};

  const useProfileFormStore = create<ProfileFormState>((set) => ({
  open: false,
  setOpen: (open) => set({ open }),
  value: null,
  setValue: (value) => set({ value, operation: value ? 'update' : 'create' }),
  operation: 'create',
}));

export const useProfileFormState = () => {
  const open = useProfileFormStore((state) => state.open);
  const value = useProfileFormStore((state) => state.value);
  const operation = useProfileFormStore((state) => state.operation);
  return { open, value, operation };
};

export const useProfileFormActions = () => {
  const setOpen = useProfileFormStore((state) => state.setOpen);
  const setValue = useProfileFormStore((state) => state.setValue);
  return { setOpen, setValue };
};
