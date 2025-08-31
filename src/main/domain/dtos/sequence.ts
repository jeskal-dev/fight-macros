import { SequenceStep } from '@/shared/bindings/SequenceStep';
import { IDataConvertible, IFormConvertible } from '@/shared/types/convertible';
import { Type } from '@/shared/types/utils';
import { SequenceStepSchema } from '../schemas/SequenceStepSchema';
import { parse } from 'valibot';

export class SequenceStepDto
  implements
    IDataConvertible<SequenceStep>,
    IFormConvertible<SequenceStepSchema>
{
  id: number;
  type: 'keydown' | 'keyup' | 'delay';
  key?: string;
  delay?: number;

  constructor(value: Type<SequenceStepDto>) {
    this.id = value.id;
    this.type = value.type;
    this.key = value.key;
    this.delay = value.delay;
  }

  static create(value: Type<SequenceStepDto>) {
    return new SequenceStepDto(value);
  }

  static fromData(step: SequenceStep): SequenceStepDto {
    if (step.type === 'delay') {
      return new SequenceStepDto({
        id: step.id,
        type: step.type,
        delay: step.ms,
      });
    } else if (step.type === 'keydown' || step.type === 'keyup') {
      return new SequenceStepDto({
        id: step.id,
        type: step.type,
        key: step.key,
      });
    }
    throw new Error(`Unknown step type: ${(step as SequenceStepDto).type}`);
  }

  toData(): SequenceStep {
    switch (this.type) {
      case 'keydown':
      case 'keyup':
        if (this.key === undefined) {
          throw new Error(`Key is required for type "${this.type}"`);
        }
        return { type: this.type, id: this.id, key: this.key };

      case 'delay':
        if (this.delay === undefined) {
          throw new Error(`Delay is required for type "${this.type}"`);
        }
        return { type: this.type, id: this.id, ms: this.delay };

      default:
        throw new Error(`Unknown step type: ${(this as SequenceStepDto).type}`);
    }
  }
  toForm(): SequenceStepSchema {
    return parse(SequenceStepSchema, this);
  }
}
