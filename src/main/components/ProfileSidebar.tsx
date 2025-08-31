import { useProfileList } from '@/main/hooks/useProfileList';
import { Plus, RefreshCcw } from 'lucide-react';
import { ProfileForm } from './ProfileForm';
import { ProfileList } from './ProfileList';
import { RefreshButton } from './RefreshButton';
import { SectionHeader } from './SectionHeader';
import { Button } from '../../shared/components/ui/button';
import { useGlobalStoreActions } from '../stores/GlobalStore';
import { useProfileFormActions } from '../stores/ProfileFormStore';
import { KeyboardButton } from './KeyboardButton';

export function ProfileSidebar() {
  const { refresh } = useGlobalStoreActions();
  const { setOpen, setValue } = useProfileFormActions();
  const {
    profiles,
    selectedProfileId,
    profilesWithKeys,
    actions: listActions,
  } = useProfileList();

  return (
    <div className="h-full bg-background">
      <SectionHeader
        title="Perfiles"
        subtitle={
          <span className="text-xs">{profilesWithKeys} con teclas rápidas</span>
        }
        actions={
          <>
            <KeyboardButton />
            <RefreshButton
              size="sm"
              variant="ghost"
              className="size-6"
              onClick={refresh}
            >
              <RefreshCcw className="size-4" />
            </RefreshButton>
            <Button
              size="sm"
              variant="ghost"
              className="size-6"
              onClick={() => {
                setOpen(true);
                setValue(null);
              }}
            >
              <Plus className="h-3 w-3" />
            </Button>
          </>
        }
      />

      <ProfileList
        profiles={profiles}
        selectedProfileId={selectedProfileId}
        onSelectProfile={listActions.selectProfile}
        onToggleProfile={listActions.toggleProfile}
        onDeleteProfile={listActions.deleteProfile}
      />

      <ProfileForm profiles={profiles} />
    </div>
  );
}
