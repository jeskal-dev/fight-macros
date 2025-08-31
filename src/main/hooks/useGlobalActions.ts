import { MacroDto } from '@/main/domain/dtos/macro';
import { ProfileDto } from '@/main/domain/dtos/profile';
import { SequenceStepDto } from '@/main/domain/dtos/sequence'; 
import { changeActiveProfile } from '@/main/services/config'; 
import { Type } from '@/shared/types/utils';
import { useCallback } from 'react';
import { MacroActionType } from '../reducers/macro.actions';
import { useGlobalStoreActions } from '../stores/GlobalStore';

export const useGlobalActions = () => {
  const { dispatch } = useGlobalStoreActions();

  const selectProfile = useCallback(
    (id: number) => {
      dispatch({ type: MacroActionType.SELECT_PROFILE, payload: id });
    },
    [dispatch]
  );

  const toggleProfile = useCallback(
    (id: number) => {
      changeActiveProfile(id).then(() =>
        dispatch({ type: MacroActionType.TOGGLE_PROFILE, payload: id })
      );
    },
    [dispatch]
  );

  const addProfile = useCallback(
    (profile: ProfileDto) => {
      dispatch({ type: MacroActionType.ADD_PROFILE, payload: profile });
    },
    [dispatch]
  );

  const updateProfile = useCallback(
    (id: number, updates: Partial<ProfileDto>) => {
      dispatch({
        type: MacroActionType.UPDATE_PROFILE,
        payload: { id, updates },
      });
    },
    [dispatch]
  );

  const deleteProfile = useCallback(
    (id: number) => {
      dispatch({ type: MacroActionType.DELETE_PROFILE, payload: id });
    },
    [dispatch]
  );

  const selectMacro = useCallback(
    (macroId: number | null) => {
      dispatch({ type: MacroActionType.SELECT_MACRO, payload: macroId });
    },
    [dispatch]
  );

  const addMacro = useCallback(
    (profileId: number, macro: Type<MacroDto>) => {
      dispatch({
        type: MacroActionType.ADD_MACRO,
        payload: { profileId, macro: new MacroDto(macro) },
      });
    },
    [dispatch]
  );

  const updateMacro = useCallback(
    (profileId: number, macroId: number, updates: Partial<MacroDto>) => {
      dispatch({
        type: MacroActionType.UPDATE_MACRO,
        payload: { profileId, macroId, updates },
      });
    },
    [dispatch]
  );

  const deleteMacro = useCallback(
    (profileId: number, macroId: number) => {
      dispatch({
        type: MacroActionType.DELETE_MACRO,
        payload: { profileId, macroId },
      });
    },
    [dispatch]
  );

  const updateMacroSequence = useCallback(
    (profileId: number, macroId: number, sequence: Type<SequenceStepDto>[]) => {
      dispatch({
        type: MacroActionType.UPDATE_MACRO_SEQUENCE,
        payload: {
          profileId,
          macroId,
          sequence: sequence.map(SequenceStepDto.create),
        },
      });
    },
    [dispatch]
  );

  const addStep = useCallback(
    (profileId: number, macroId: number, step: SequenceStepDto) => {
      dispatch({
        type: MacroActionType.ADD_STEP,
        payload: { profileId, macroId, step },
      });
    },
    [dispatch]
  );

  const updateStep = useCallback(
    (
      profileId: number,
      macroId: number,
      stepId: number,
      updates: Partial<SequenceStepDto>
    ) => {
      dispatch({
        type: MacroActionType.UPDATE_STEP,
        payload: { profileId, macroId, stepId, updates },
      });
    },
    [dispatch]
  );

  const deleteStep = useCallback(
    (profileId: number, macroId: number, stepId: number) => {
      dispatch({
        type: MacroActionType.DELETE_STEP,
        payload: { profileId, macroId, stepId },
      });
    },
    [dispatch]
  );

  return {
    selectProfile,
    toggleProfile,
    addProfile,
    updateProfile,
    deleteProfile,
    selectMacro,
    addMacro,
    updateMacro,
    deleteMacro,
    updateMacroSequence,
    addStep,
    updateStep,
    deleteStep,
  };
};
