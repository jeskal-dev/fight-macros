import { MacroDto } from "@/main/domain/dtos/macro";
import { ProfileDto } from "@/main/domain/dtos/profile";
import { nullable } from "@/shared/helpers/nullable";
import { Nullable, Type } from "@/shared/types/utils"; 

export const findProfile = (
  profiles: ProfileDto[],
  profileId: number
): Nullable<ProfileDto> => nullable(profiles.find((p) => p.id === profileId));

export const findMacro = (
  profile: Type<ProfileDto>,
  macroId: number
): Nullable<MacroDto> => nullable(profile.macros.find((m) => m.id === macroId));

export const updateProfileInState = (
  profiles: Type<ProfileDto>[],
  profileId: number,
  updater: (profile: Type<ProfileDto>) => Type<ProfileDto>
): ProfileDto[] => profiles.map((p) => new ProfileDto(p.id === profileId ? updater(p) : p));

export const updateMacroInProfile = (
  profile: Type<ProfileDto>,
  macroId: number,
  updater: (macro: Type<MacroDto>) => Type<MacroDto>
): ProfileDto =>
  new ProfileDto({
    ...profile,
    macros: profile.macros.map((m) => new MacroDto((m.id === macroId ? updater(m) : m))),
  });
