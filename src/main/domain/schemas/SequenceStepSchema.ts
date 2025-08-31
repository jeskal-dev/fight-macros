import {
  InferOutput,
  literal,
  minValue,
  nonEmpty,
  number,
  object,
  optional,
  pipe,
  string,
  union,
} from 'valibot';

const SequenceStepTypeSchema = union(
  [literal('delay'), literal('keydown'), literal('keyup')],
  "El tipo de paso debe ser 'delay', 'keydown' o 'keyup'"
);

export const SequenceStepSchema = object({
  type: SequenceStepTypeSchema,
  key: optional(
    pipe(
      string('La tecla debe ser una cadena de texto'),
      nonEmpty('La tecla no puede estar vacía')
    ),
    undefined
  ),
  delay: optional(
    pipe(
      number('El tiempo debe ser un número'),
      minValue(1, 'El tiempo no puede estar vacío')
    ),
    undefined
  ),
});

export type SequenceStepSchema = InferOutput<typeof SequenceStepSchema>;
