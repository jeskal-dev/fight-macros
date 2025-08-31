import { FunctionKey } from '@/shared/bindings/Key';
import { Nullable, Type } from '@/shared/types/utils';
import { MacroDto } from './macro';
import { Profile } from '@/shared/bindings/Profile';
import { IDataConvertible, IFormConvertible } from '@/shared/types/convertible';
import { ProfileSchema } from '../schemas/ProfileSchema';
import { parse } from 'valibot';

export class ProfileDto
  implements IDataConvertible<Profile>, IFormConvertible<ProfileSchema>
{
  id: number;
  name: Nullable<string>;
  functionKey: Nullable<FunctionKey>;
  active?: boolean;
  macros: MacroDto[];

  constructor(value: Type<ProfileDto>) {
    this.id = value.id;
    this.name = value.name;
    this.functionKey = value.functionKey;
    this.active = value.active ?? false;
    this.macros = value.macros.map((m) => new MacroDto(m));
  }

  static fromData(profile: Profile): ProfileDto {
    return new ProfileDto({
      id: profile.id,
      name: profile.name || null,
      functionKey: profile.functionKey,
      macros: profile.macros.map(MacroDto.fromData),
    });
  }

  static formArray(profiles: Profile[]): ProfileDto[] {
    return profiles.map(ProfileDto.fromData);
  }

  toData(): Profile {
    if (this.name === null) {
      throw new Error('Profile name cannot be null');
    }
    return {
      id: this.id,
      name: this.name,
      functionKey: this.functionKey,
      macros: this.macros.map((m) => m.toData()),
    };
  }

  toForm(): ProfileSchema {
    return parse(ProfileSchema, this);
  }
}
