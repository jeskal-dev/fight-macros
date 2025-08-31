import { MacroDto } from "@/main/domain/dtos/macro";
import { ProfileDto } from "@/main/domain/dtos/profile";
import { SequenceStepDto } from "@/main/domain/dtos/sequence"; 
import { MacroActionType } from "./macro.actions";
 export type ProfileAction =
  | { type: MacroActionType.SELECT_PROFILE; payload: number }
  | { type: MacroActionType.TOGGLE_PROFILE; payload: number }
  | { type: MacroActionType.ADD_PROFILE; payload: ProfileDto }
  | {
      type: MacroActionType.UPDATE_PROFILE;
      payload: { id: number; updates: Partial<ProfileDto> };
    }
  | { type: MacroActionType.DELETE_PROFILE; payload: number } | {
      type: MacroActionType.SET_PROFILES; payload: ProfileDto[];
  };

export type MacroAction =
  | { type: MacroActionType.SELECT_MACRO; payload: number | null }
  | {
      type: MacroActionType.ADD_MACRO;
      payload: { profileId: number; macro: MacroDto };
    }
  | {
      type: MacroActionType.UPDATE_MACRO;
      payload: {
        profileId: number;
        macroId: number;
        updates: Partial<MacroDto>;
      };
    }
  | {
      type: MacroActionType.DELETE_MACRO;
      payload: { profileId: number; macroId: number };
    }
  | {
      type: MacroActionType.UPDATE_MACRO_SEQUENCE;
      payload: {
        profileId: number;
        macroId: number;
        sequence: SequenceStepDto[];
      };
    };

export type StepAction =
  | {
      type: MacroActionType.ADD_STEP;
      payload: { profileId: number; macroId: number; step: SequenceStepDto };
    }
  | {
      type: MacroActionType.UPDATE_STEP;
      payload: {
        profileId: number;
        macroId: number;
        stepId: number;
        updates: Partial<SequenceStepDto>;
      };
    }
  | {
      type: MacroActionType.DELETE_STEP;
      payload: { profileId: number; macroId: number; stepId: number };
    };

export type ActionEvent = ProfileAction | MacroAction | StepAction;
