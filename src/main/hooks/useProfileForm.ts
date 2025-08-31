import { FUNCTION_LIST } from '@/shared/constants/function_aliases';
import { ProfileDto } from '@/main/domain/dtos/profile';
import { ProfileSchema } from '@/main/domain/schemas/ProfileSchema';
import { falsy } from '@/shared/helpers/falsy';
import { generateID } from '@/shared/helpers/generateID';
import { nullable } from '@/shared/helpers/nullable';
  
import { valibotResolver } from '@hookform/resolvers/valibot';
import { useCallback, useMemo } from 'react';
import { useForm } from 'react-hook-form';
import { useGlobalActions } from './useGlobalActions';
import { useProfileFormState, useProfileFormActions } from '../stores/ProfileFormStore';

export const useProfileForm = (profiles: ProfileDto[]) => {
  const { open, value, operation } = useProfileFormState();
  const { setOpen, setValue } = useProfileFormActions();
  const { addProfile } = useGlobalActions();
  const form = useForm({
    resolver: valibotResolver(ProfileSchema),
    values: value?.toForm(),
    defaultValues: {
      name: '',
      functionKey: null,
    },
    mode: 'onChange',
  });

  const usedKeys = useMemo(
    () => profiles.map((p) => p.functionKey).filter((val) => !!val),
    [profiles]
  );

  const availableKeys = useMemo(
    () => FUNCTION_LIST.filter((key) => !usedKeys.includes(key)),
    [usedKeys]
  );

  const handleOpen = useCallback(() => {
    setOpen(true);
  }, [setOpen]);

  const handleClose = useCallback(() => {
    setOpen(false);
    setValue(null);
    form.reset({
      name: '',
      functionKey: null,
    });
  }, [form, setOpen, setValue]);
  const handleSubmit = form.handleSubmit((data) => {
    const id = operation === 'update' && value?.id ? value.id : generateID();

    const newProfile = new ProfileDto({
      id,
      name: data.name.trim(),
      functionKey: nullable(data.functionKey),
      macros: operation === 'update' ? value?.macros || [] : [],
      active: falsy(value?.active),
    });

    addProfile(newProfile);
    handleClose();
  });

  return {
    open,
    form,
    usedKeys,
    availableKeys,
    actions: {
      open: handleOpen,
      close: handleClose,
      submit: handleSubmit,
    },
  };
};
