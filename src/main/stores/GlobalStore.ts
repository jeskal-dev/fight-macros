import { ProfileDto } from '@/main/domain/dtos/profile';
import {
  changeActiveProfile,
  loadConfig,
  saveConfig,
} from '@/main/services/config';
import { nullable } from '@/shared/helpers/nullable';
import { watch } from '@/shared/services/watch';
import { Nullable } from '@/shared/types/utils';
import { UnlistenFn } from '@tauri-apps/api/event';
import { Dispatch, useEffect, useRef } from 'react';
import { toast } from 'sonner';
import { create } from 'zustand';
import { subscribeWithSelector } from 'zustand/middleware';
import { MacroActionType } from '../reducers/macro.actions';
import { ActionEvent } from '../reducers/macro.events';
import { macroReducer } from '../reducers/macro.handler';

type StatusType = 'idle' | 'loading' | 'saving' | 'error';
export interface DataState {
  profiles: ProfileDto[];
  selectedProfileId: number;
  selectedMacroId: number | null;
}

export type GlobalState = {
  status: StatusType;
  error: Nullable<string>;
} & DataState;

export type GlobalActions = {
  setStatus: Dispatch<StatusType>;
  setError: Dispatch<Nullable<string>>;
  dispatch: Dispatch<ActionEvent>;

  watchActiveProfile: () => Promise<UnlistenFn | undefined>;
  loadProfiles: () => Promise<void>;
  saveChanges: () => Promise<void>;

  handleProfileKeyDownEffect: () => () => void;
};

export type GlobalStore = GlobalState & GlobalActions;

const useGlobalStore = create<GlobalStore>()(
  subscribeWithSelector((set, get) => ({
    status: 'idle',
    error: null,
    profiles: [],
    selectedProfileId: 0,
    selectedMacroId: null,

    // Acciones
    setStatus: (status) => set({ status }),
    setError: (error) => set({ error }),
    dispatch: (action) => {
      const currentState = get();
      const reducerInput = {
        profiles: currentState.profiles,
        selectedProfileId: currentState.selectedProfileId,
        selectedMacroId: currentState.selectedMacroId,
      };
      const reducerOutput = macroReducer(reducerInput, action);
      set({ ...currentState, ...reducerOutput });
    },
    async watchActiveProfile() {
      const { dispatch } = get();

      try {
        return await watch<number>('selected-profile-changed', (event) => {
          dispatch({
            type: MacroActionType.TOGGLE_PROFILE,
            payload: event.payload,
          });
        });
      } catch (err) {
        const error = 'Failed to set up profile watcher';
        console.error(error, err);
        set({ error });
      }
    },
    async loadProfiles() {
      const { dispatch } = get();
      set({ status: 'loading', error: null });
      try {
        const result = await loadConfig();
        if (result.isErr()) {
          throw new Error(result.error);
        }

        const config = result.value;
        const profiles = config.profiles
          .map((data) => ProfileDto.fromData(data))
          .map(
            (profile) =>
              new ProfileDto({
                ...profile,
                active: profile.id === config.selectedProfileId,
              })
          );
        dispatch({ type: MacroActionType.SET_PROFILES, payload: profiles });
        set({ status: 'idle' });
      } catch (err) {
        const error = 'Failed to load profiles';
        console.error(error, err);
        set({ error, status: 'error' });
      }
    },
    async saveChanges() {
      const { status, profiles } = get();
      if (status === 'loading') return;
      set({ status: 'saving', error: null });
      try {
        const activeProfile = profiles.find((p) => p.active);
        const profilesData = profiles.map((p) => p.toData());

        const saveResult = await saveConfig({
          profiles: profilesData,
          selectedProfileId: nullable(activeProfile?.id),
        });

        if (saveResult.isErr()) throw new Error(saveResult.error);
        set({ status: 'idle' });
      } catch (e) {
        const err = Object(e);
        const error = err.message || 'Unknown error saving changes';
        console.error(error, err);
        set({ error, status: 'error' });
      }
    },
    handleProfileKeyDownEffect() {
      const { dispatch, profiles } = get();
      const handleKeyDown = (event: KeyboardEvent) => {
        if (!(event.key.startsWith('F') && event.key.length > 1)) return;

        event.preventDefault();
        event.stopPropagation();

        const profile = profiles.find((p) => p.functionKey === event.key);
        if (profile) {
          changeActiveProfile(profile.id).then(() => {
            dispatch({
              type: MacroActionType.TOGGLE_PROFILE,
              payload: profile.id,
            });
          });
        }
      };

      document.addEventListener('keydown', handleKeyDown);

      return () => {
        document.removeEventListener('keydown', handleKeyDown);
      };
    },
  }))
);

export const useGlobalStoreInitializer = () => {
  const { loadProfiles, watchActiveProfile, handleProfileKeyDownEffect } =
    useGlobalStore();
  const unlistenRef = useRef<UnlistenFn | undefined>(undefined);
  useEffect(() => void loadProfiles(), [loadProfiles]);
  useEffect(() => {
    const unlisten = () => {
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = undefined;
      }
    };
    unlisten();
    const calling = async () => {
      const unlisten = await watchActiveProfile();
      unlistenRef.current = unlisten;
    };
    calling();

    return () => unlisten();
  }, [watchActiveProfile]);

  useEffect(() => {
    const unlisten = handleProfileKeyDownEffect();
    return () => unlisten();
  }, [handleProfileKeyDownEffect]);

  useEffect(() => {
    const unlisten = useGlobalStore.subscribe(
      (store) => store.profiles,
      (current, previous) => {
        const changes = {
          current: JSON.stringify(current),
          previous: JSON.stringify(previous),
        };

        if (changes.current === changes.previous) return;
        const { status, saveChanges } = useGlobalStore.getState();
        if (status === 'loading' || status === 'saving') return;

        saveChanges();
      },
      {
        equalityFn: (a, b) => JSON.stringify(a) === JSON.stringify(b),
        fireImmediately: false,
      }
    );
    return () => unlisten();
  }, []);
  useEffect(() => {
    const unlisten = useGlobalStore.subscribe(
      (store) => store.error,
      (error) => {
        if (error) {
          const { setError } = useGlobalStore.getState();
          toast.error(error);
          setError(null);
        }
      }
    );
    return () => unlisten();
  }, []);
};

export const useGlobalState = () => {
  const profiles = useGlobalStore((state) => state.profiles);
  const selectedMacroId = useGlobalStore((state) => state.selectedMacroId);
  const selectedProfileId = useGlobalStore((state) => state.selectedProfileId);
  const status = useGlobalStore((state) => state.status);
  const error = useGlobalStore((state) => state.error);

  const state = { profiles, selectedMacroId, selectedProfileId };
  return { state, status, error };
};

export const useGlobalStoreActions = () => {
  const dispatch = useGlobalStore((state) => state.dispatch);
  const setStatus = useGlobalStore((state) => state.setStatus);
  const setError = useGlobalStore((state) => state.setError);
  const refresh = useGlobalStore((state) => state.loadProfiles);
  return { dispatch, setStatus, setError, refresh };
};
