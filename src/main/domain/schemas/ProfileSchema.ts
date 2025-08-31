import { InferOutput, literal, nonEmpty, nullish, object, pipe, string, union } from 'valibot';

export const FunctionKeySchema = union(
  [
    literal('F1', 'El valor debe ser F1'),
    literal('F2', 'El valor debe ser F2'),
    literal('F3', 'El valor debe ser F3'),
    literal('F4', 'El valor debe ser F4'),
    literal('F5', 'El valor debe ser F5'),
    literal('F6', 'El valor debe ser F6'),
    literal('F7', 'El valor debe ser F7'),
    literal('F8', 'El valor debe ser F8'),
    literal('F9', 'El valor debe ser F9'),
    literal('F10', 'El valor debe ser F10'),
    literal('F11', 'El valor debe ser F11'),
    literal('F12', 'El valor debe ser F12'),
  ],
  'La tecla de función debe ser F1-F12'
);

export const ProfileSchema = object({
  name: pipe(string('El nombre debe ser un texto'), nonEmpty('El nombre no puede estar vacío')),
  functionKey: nullish(FunctionKeySchema, null),
});

export type ProfileSchema = InferOutput<typeof ProfileSchema>;
