import { Type } from '@/shared/types/utils';
import { Macro } from '@/shared/bindings/Macro';
import { KeyCombination } from '@/shared/bindings/KeyCombination';
import { MODIFIER_ALIASES } from '@/shared/constants/modifiers_aliases';
import { IDataConvertible, IFormConvertible } from '@/shared/types/convertible';
import { SequenceStepDto } from './sequence';
import { MacroSchema } from '../schemas/MacroSchema';
import { parse } from 'valibot';

export class MacroDto
  implements IDataConvertible<Macro>, IFormConvertible<MacroSchema>
{
  id: number;
  name: string;
  triggerKey: string;
  sequence: SequenceStepDto[];

  constructor(value: Type<MacroDto>) {
    this.id = value.id;
    this.name = value.name;
    this.triggerKey = value.triggerKey;
    this.sequence = value.sequence.map((s) => new SequenceStepDto(s));
  }

  static fromData(macro: Macro): MacroDto {
    return new MacroDto({
      id: macro.id,
      name: macro.name,
      triggerKey: MacroDto.stringifyKeyCombination(macro.trigger),
      sequence: macro.sequence.map(SequenceStepDto.fromData),
    });
  }

  static formArray(macros: Macro[]): MacroDto[] {
    return macros.map(MacroDto.fromData);
  }

  toData(): Macro {
    return {
      id: this.id,
      name: this.name,
      trigger: MacroDto.parseTriggerKey(this.triggerKey),
      sequence: this.sequence.map((s) => s.toData()),
    };
  }

  private static parseTriggerKey(raw: string): KeyCombination {
    if (!raw.trim()) throw new Error('Empty trigger key');

    const parts = raw
      .split('+')
      .map((s) => s.trim())
      .filter(Boolean);

    if (parts.length === 0) throw new Error('Invalid trigger key');

    const key = parts.pop()!;
    const modifiers = parts
      .map((p) => {
        const m = MODIFIER_ALIASES[p];
        if (!m) throw new Error(`Unknown modifier: ${p}`);
        return m;
      })
      .map((m) => (m === 'Control' ? 'Ctrl' : m));

    return { modifiers, key };
  }

  private static stringifyKeyCombination({
    modifiers,
    key,
  }: KeyCombination): string {
    return [...modifiers, key].join(' + ');
  }

  toForm(): MacroSchema {
    return parse(MacroSchema, this);
  }
}
