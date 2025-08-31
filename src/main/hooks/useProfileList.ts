import { useCallback, useMemo } from 'react';
import { useGlobalActions } from './useGlobalActions'; 
import { useGlobalState } from '../stores/GlobalStore';

export const useProfileList = () => {
  const { state } = useGlobalState();
  const { selectProfile, toggleProfile, deleteProfile } = useGlobalActions();

  const { profiles, selectedProfileId } = state;

  const profilesWithKeys = useMemo(
    () => profiles.filter((p) => p.functionKey),
    [profiles]
  );

  const canDeleteProfile = useMemo(
    () => profiles.length > 1,
    [profiles.length]
  );

  const handleSelectProfile = useCallback(
    (id: number) => selectProfile(id),
    [selectProfile]
  );

  const handleToggleProfile = useCallback(
    (id: number) => toggleProfile(id),
    [toggleProfile]
  );

  const handleDeleteProfile = useCallback(
    (id: number) => {
      if (canDeleteProfile) deleteProfile(id);
    },
    [deleteProfile, canDeleteProfile]
  );

  return {
    profiles,
    selectedProfileId,
    profilesWithKeys: profilesWithKeys.length,
    actions: {
      selectProfile: handleSelectProfile,
      toggleProfile: handleToggleProfile,
      deleteProfile: handleDeleteProfile,
    },
  };
};
