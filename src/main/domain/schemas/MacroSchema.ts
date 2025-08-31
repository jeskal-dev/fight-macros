import { KEY_LIST } from '@/shared/constants/keys_aliases';
import {
  InferOutput,
  literal,
  nonEmpty,
  object,
  pipe,
  string,
  union,
} from 'valibot';

export const TriggerKey = union(
  KEY_LIST.map(literal),
  'La tecla de activación debe ser una de las siguientes'
);

export const MacroSchema = object({
  name: pipe(
    string('El nombre debe ser un texto'),
    nonEmpty('El nombre no puede estar vacío')
  ),
  triggerKey: TriggerKey,
});

export type MacroSchema = InferOutput<typeof MacroSchema>;
