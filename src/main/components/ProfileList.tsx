import { ProfileDto } from '@/main/domain/dtos/profile';
import { falsy } from '@/shared/helpers/falsy';
import { ProfileRow } from './ProfileRow';
import { SmartList } from '../../shared/components/SmartList';
import { ScrollArea } from '../../shared/components/ui/scroll-area';

interface ProfileListProps {
  profiles: ProfileDto[];
  selectedProfileId: number;
  onSelectProfile: (id: number) => void;
  onToggleProfile: (id: number) => void;
  onDeleteProfile: (id: number) => void;
}

export const ProfileList: React.FC<ProfileListProps> = ({
  profiles,
  selectedProfileId,
  onSelectProfile,
  onToggleProfile,
  onDeleteProfile,
}) => {
  return (
    <ScrollArea className="flex-1">
      <div className=" max-h-[calc(100vh-200px)] space-y-1">
        <SmartList data={profiles} keyFn={(p) => p.id}>
          {(profile) => (
            <ProfileRow
              profile={profile}
              isSelected={selectedProfileId === profile.id}
              isActive={falsy(profile.active)}
              onSelect={() => onSelectProfile(profile.id)}
              onToggle={() => onToggleProfile(profile.id)}
              onDelete={() => onDeleteProfile(profile.id)}
            />
          )}
        </SmartList>
      </div>
    </ScrollArea>
  );
};
