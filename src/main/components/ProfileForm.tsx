import { FunctionKey } from '@/shared/bindings/Key';
import { FUNCTION_LIST } from '@/shared/constants/function_aliases';
import { ProfileDto } from '@/main/domain/dtos/profile';
import { ProfileSchema } from '@/main/domain/schemas/ProfileSchema';
import { generateID } from '@/shared/helpers/generateID';
import { useGlobalActions } from '@/main/hooks/useGlobalActions';

import { valibotResolver } from '@hookform/resolvers/valibot';
import { FC, useCallback, useMemo } from 'react';
import { Controller, FormProvider, useForm } from 'react-hook-form';
import { KeyboardInput } from '../../shared/components/KeyboardInput';
import { Button } from '../../shared/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '../../shared/components/ui/dialog';
import { Input } from '../../shared/components/ui/input';
import {
  useProfileFormActions,
  useProfileFormState,
} from '../stores/ProfileFormStore';

interface ProfileFormProps {
  profiles: ProfileDto[];
}

export const ProfileForm: FC<ProfileFormProps> = ({ profiles }) => {
  const { setOpen, setValue } = useProfileFormActions();
  const { open, value, operation } = useProfileFormState();
  const { addProfile, updateProfile } = useGlobalActions();
  const usedKeys = useMemo(
    () => profiles.map((p) => p.functionKey).filter((val) => !!val),
    [profiles]
  );

  const availableKeys = useMemo(
    () => FUNCTION_LIST.filter((key) => !usedKeys.includes(key)),
    [usedKeys]
  );

  const form = useForm({
    resolver: valibotResolver(ProfileSchema),
    values: value?.toForm(),
    defaultValues: {
      name: '',
      functionKey: null,
    },
    mode: 'onChange',
  });

  const handleClose = useCallback(() => {
    setOpen(false);
    setValue(null);
    form.reset();
  }, [form, setOpen, setValue]);

  const handleChange = useCallback(
    (isOpen: boolean) => {
      if (!isOpen) handleClose();
    },
    [handleClose]
  );

  const handleSubmit = form.handleSubmit((data) => {
    if (operation === 'create') {
      addProfile(
        ProfileDto.fromData({
          id: generateID(),
          name: data.name.trim(),
          functionKey: data.functionKey,
          macros: [],
        })
      );
    } else if (value?.id) {
      updateProfile(value?.id, {
        name: data.name.trim(),
        functionKey: data.functionKey,
      });
    }

    handleClose();
  });

  return (
    <FormProvider {...form}>
      <Dialog open={open} onOpenChange={handleChange}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {operation === 'create' ? 'Nuevo Perfil' : 'Editar Perfil'}
            </DialogTitle>
          </DialogHeader>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <Controller
                name="name"
                control={form.control}
                render={({ field: { value, onChange, onBlur } }) => (
                  <Input
                    placeholder="Nombre del perfil"
                    value={value}
                    onChange={onChange}
                    onBlur={onBlur}
                  />
                )}
              />
              {form.formState.errors.name && (
                <p className="text-sm text-destructive-foreground mt-1">
                  {form.formState.errors.name.message}
                </p>
              )}
            </div>

            <div>
              <Controller
                name="functionKey"
                control={form.control}
                render={({ field: { onChange, value, ...field } }) => (
                  <KeyboardInput
                    {...field}
                    allowedKeys={availableKeys}
                    value={value ?? ''}
                    onKeyChange={(key) =>
                      onChange((key as FunctionKey) || null)
                    }
                    onKeyError={(error) =>
                      form.setError('functionKey', {
                        message: error,
                        type: 'validate',
                      })
                    }
                  />
                )}
              />
              {form.formState.errors.functionKey && (
                <p className="text-sm text-destructive-foreground mt-1">
                  {form.formState.errors.functionKey.message}
                </p>
              )}
            </div>

            <div className="flex gap-2">
              <Button type="submit" disabled={!form.formState.isValid}>
                Confirmar
              </Button>
              <Button variant="outline" type="button" onClick={handleClose}>
                Cancelar
              </Button>
            </div>
          </form>
        </DialogContent>
      </Dialog>
    </FormProvider>
  );
};
