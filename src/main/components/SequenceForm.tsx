import { SequenceStepDto } from '@/main/domain/dtos/sequence';

import { SequenceStepSchema } from '@/main/domain/schemas/SequenceStepSchema';
import { generateID } from '@/shared/helpers/generateID';
import { useGlobalActions } from '@/main/hooks/useGlobalActions';

import { valibotResolver } from '@hookform/resolvers/valibot';
import { useCallback } from 'react';
import { Controller, FormProvider, useForm } from 'react-hook-form';
import { Choose, Otherwise, When } from '../../shared/components/Conditional';
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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../shared/components/ui/select';
import {
  useSequenceFormState,
  useSequenceFormActions,
} from '../stores/SequenceFormStore';

export function SequenceForm() {
  const { open, value, operation, currentProfileId, sequence, currentMacroId } =
    useSequenceFormState();
  const { setOpen, setValue } = useSequenceFormActions();
  const { updateMacroSequence } = useGlobalActions();

  const form = useForm({
    resolver: valibotResolver(SequenceStepSchema),
    values: value?.toForm(),
    defaultValues: {
      delay: 10,
      key: undefined,
      type: 'delay',
    },
    mode: 'onChange',
  });

  const stepType = form.watch('type');

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
    if (!currentProfileId || !currentMacroId)
      throw new Error('No profile selected');

    if (operation === 'create') {
      const step = {
        id: generateID(),
        type: data.type,
        key: data.key,
        delay: data.delay,
      };
      const newSequence = [...sequence, step];
      updateMacroSequence(currentProfileId, currentMacroId, newSequence);
    } else if (value?.id) {
      const step = SequenceStepDto.create({
        id: value.id,
        ...data,
      });
      const index = sequence.findIndex((s) => s.id === value.id);
      const newSequence = [...sequence];
      newSequence.splice(index, 1, step);
      updateMacroSequence(currentProfileId, currentMacroId, newSequence);
    }

    handleClose();
  });

  return (
    <FormProvider {...form}>
      <Dialog open={open} onOpenChange={handleChange}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {operation === 'create' ? 'Nuevo paso' : 'Editar paso'}
            </DialogTitle>
          </DialogHeader>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <Controller
                control={form.control}
                name="type"
                render={({ field: { onChange, value, ...field } }) => (
                  <Select
                    {...field}
                    value={value}
                    onValueChange={(value) => onChange(value)}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="keydown">Key Down</SelectItem>
                      <SelectItem value="keyup">Key Up</SelectItem>
                      <SelectItem value="delay">Delay</SelectItem>
                    </SelectContent>
                  </Select>
                )}
              />
              {form.formState.errors.type && (
                <p className="text-sm text-destructive-foreground mt-1">
                  {form.formState.errors.type.message}
                </p>
              )}
            </div>

            <div>
              <Choose>
                <When
                  condition={() =>
                    stepType === 'keydown' || stepType === 'keyup'
                  }
                >
                  <Controller
                    control={form.control}
                    name="key"
                    render={({ field: { onChange, value, ...field } }) => (
                      <KeyboardInput
                        {...field}
                        placeholder="Presiona las teclas..."
                        value={value || ''}
                        onKeyChange={(key) => onChange(key)}
                        onKeyError={(error) =>
                          form.setError('key', {
                            message: error,
                            type: 'validate',
                          })
                        }
                      />
                    )}
                  />
                  {form.formState.errors.key && (
                    <p className="text-sm text-destructive-foreground mt-1">
                      {form.formState.errors.key.message}
                    </p>
                  )}
                </When>

                <Otherwise>
                  <Controller
                    control={form.control}
                    name="delay"
                    render={({ field: { onChange, value, ...field } }) => (
                      <Input
                        type="number"
                        placeholder="Milisegundos"
                        {...field}
                        value={value || ''}
                        onChange={(e) =>
                          onChange(parseInt(e.target.value) || 0)
                        }
                      />
                    )}
                  />
                  {form.formState.errors.delay && (
                    <p className="text-sm text-destructive-foreground mt-1">
                      {form.formState.errors.delay.message}
                    </p>
                  )}
                </Otherwise>
              </Choose>
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
