import { MacroSchema } from '@/main/domain/schemas/MacroSchema';
import { generateID } from '@/shared/helpers/generateID';
import { useGlobalActions } from '@/main/hooks/useGlobalActions';

import { valibotResolver } from '@hookform/resolvers/valibot';
import { useCallback } from 'react';
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
  useMacroFormState,
  useMacroFormActions,
} from '../stores/MacroFormStore';

export function MacroForm() {
  const { open, value, operation, currentProfileId } = useMacroFormState();
  const { setOpen, setValue } = useMacroFormActions();
  const { addMacro, updateMacro } = useGlobalActions();

  const form = useForm({
    resolver: valibotResolver(MacroSchema),
    values: value?.toForm(),
    defaultValues: {
      name: '',
      triggerKey: '',
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
    if (!currentProfileId) throw new Error('No profile selected');
    if (operation === 'create') {
      const macro = {
        id: generateID(),
        name: data.name.trim(),
        triggerKey: data.triggerKey.toString(),
        sequence: [],
      };
      addMacro(currentProfileId, macro);
    } else if (value?.id) {
      const macro = {
        name: data.name.trim(),
        triggerKey: data.triggerKey.toString(),
      };
      updateMacro(currentProfileId, value?.id, macro);
    }

    handleClose();
  });

  return (
    <FormProvider {...form}>
      <Dialog open={open} onOpenChange={handleChange}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {operation === 'create' ? 'Nueva Macro' : 'Editar Macro'}
            </DialogTitle>
          </DialogHeader>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <Controller
                name="name"
                control={form.control}
                render={({ field }) => (
                  <Input
                    placeholder="Nombre del macro"
                    {...field}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' && form.formState.isValid) {
                        e.preventDefault();
                        handleSubmit();
                      }
                    }}
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
                control={form.control}
                name="triggerKey"
                render={({ field: { onChange, value, ...field } }) => (
                  <KeyboardInput
                    {...field}
                    placeholder="Tecla de activacion (ej: Q, F4, Ctrl+S)"
                    value={value.toString()}
                    onKeyChange={(key) => onChange(key)}
                    onKeyError={(error) =>
                      form.setError('triggerKey', {
                        message: error,
                        type: 'validate',
                      })
                    }
                  />
                )}
              />
              {form.formState.errors.triggerKey && (
                <p className="text-sm text-destructive-foreground mt-1">
                  {form.formState.errors.triggerKey.message}
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
}
