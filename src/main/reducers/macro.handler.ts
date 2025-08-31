import { ProfileDto } from '@/main/domain/dtos/profile';
import { SequenceStepDto } from '@/main/domain/dtos/sequence';
 
 
 
import { ActionEvent } from './macro.events';
import { MacroActionType } from './macro.actions';
import { DataState } from '../stores/GlobalStore';
import { updateProfileInState, updateMacroInProfile } from './macro.utils';

export const initialState: DataState = {
  profiles: [],
  selectedProfileId: 0,
  selectedMacroId: null,
};

export const macroReducer = (
  state: DataState,
  action: ActionEvent
): DataState => {
  switch (action.type) {
    case MacroActionType.SELECT_PROFILE:
      return {
        ...state,
        selectedProfileId: action.payload,
        selectedMacroId: null,
      };

    case MacroActionType.TOGGLE_PROFILE: {
      return {
        ...state,
        profiles: state.profiles.map(
          (p) =>
            new ProfileDto({
              ...p,
              active: p.id === action.payload,
            })
        ),
      };
    }

    case MacroActionType.ADD_PROFILE:
      return {
        ...state,
        profiles: [...state.profiles, action.payload],
      };

    case MacroActionType.UPDATE_PROFILE:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.id,
          (profile) => ({ ...profile, ...action.payload.updates })
        ),
      };

    case MacroActionType.DELETE_PROFILE: {
      const newProfiles = state.profiles.filter((p) => p.id !== action.payload);
      const newSelectedId = newProfiles.length > 0 ? newProfiles[0].id : 0;

      return {
        ...state,
        profiles: newProfiles,
        selectedProfileId:
          state.selectedProfileId === action.payload
            ? newSelectedId
            : state.selectedProfileId,
      };
    }
    case MacroActionType.SET_PROFILES:
      return {
        ...state,
        profiles: action.payload,
      };
    case MacroActionType.SELECT_MACRO:
      return { ...state, selectedMacroId: action.payload };

    case MacroActionType.ADD_MACRO:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.profileId,
          (profile) => ({
            ...profile,
            macros: [...profile.macros, action.payload.macro],
          })
        ),
      };

    case MacroActionType.UPDATE_MACRO:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.profileId,
          (profile) =>
            updateMacroInProfile(profile, action.payload.macroId, (macro) => ({
              ...macro,
              ...action.payload.updates,
            }))
        ),
      };

    case MacroActionType.DELETE_MACRO: {
      const { profileId, macroId } = action.payload;

      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          profileId,
          (profile) => ({
            ...profile,
            macros: profile.macros.filter((m) => m.id !== macroId),
          })
        ),
        selectedMacroId:
          state.selectedMacroId === macroId ? null : state.selectedMacroId,
      };
    }

    case MacroActionType.UPDATE_MACRO_SEQUENCE:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.profileId,
          (profile) =>
            updateMacroInProfile(profile, action.payload.macroId, (macro) => ({
              ...macro,
              sequence: action.payload.sequence,
            }))
        ),
      };

    case MacroActionType.ADD_STEP:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.profileId,
          (profile) =>
            updateMacroInProfile(profile, action.payload.macroId, (macro) => ({
              ...macro,
              sequence: [...macro.sequence, action.payload.step],
            }))
        ),
      };

    case MacroActionType.UPDATE_STEP:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.profileId,
          (profile) =>
            updateMacroInProfile(profile, action.payload.macroId, (macro) => ({
              ...macro,
              sequence: macro.sequence.map(
                (step) =>
                  new SequenceStepDto(
                    step.id === action.payload.stepId
                      ? { ...step, ...action.payload.updates }
                      : step
                  )
              ),
            }))
        ),
      };

    case MacroActionType.DELETE_STEP:
      return {
        ...state,
        profiles: updateProfileInState(
          state.profiles,
          action.payload.profileId,
          (profile) =>
            updateMacroInProfile(profile, action.payload.macroId, (macro) => ({
              ...macro,
              sequence: macro.sequence.filter(
                (step) => step.id !== action.payload.stepId
              ),
            }))
        ),
      };

    default:
      return state;
  }
};
