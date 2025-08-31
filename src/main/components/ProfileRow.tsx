import { ProfileDto } from '@/main/domain/dtos/profile';
import { cn } from '@/shared/lib/utils';

import { Edit, Keyboard, MoreVertical, Power, Trash2 } from 'lucide-react';
import { Show } from '../../shared/components/Conditional';
import { Avatar, AvatarFallback } from '../../shared/components/ui/avatar';
import { Badge } from '../../shared/components/ui/badge';
import { Button } from '../../shared/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../../shared/components/ui/dropdown-menu';
import {
  useProfileFormActions,
  useProfileFormState,
} from '../stores/ProfileFormStore';

interface ProfileRowProps {
  profile: ProfileDto;
  isSelected: boolean;
  isActive: boolean;
  onSelect: () => void;
  onToggle: () => void;
  onDelete: () => void;
}

export function ProfileRow({
  profile,
  isSelected,
  isActive,
  onSelect,
  onToggle,
  onDelete,
}: Readonly<ProfileRowProps>) {
  const { setOpen, setValue } = useProfileFormActions();
  const { open } = useProfileFormState();

  return (
    <div
      className={cn(
        'group relative flex items-center gap-2 p-2 rounded-md cursor-pointer transition-colors',
        isSelected ? 'bg-accent/50' : 'hover:bg-accent/30'
      )}
      onClick={onSelect}
    >
      <Avatar
        className={cn(
          'h-8 w-8 transition-all flex-shrink-0',
          isActive && 'ring-2 ring-success'
        )}
      >
        <AvatarFallback className="text-xs bg-primary/20">
          {profile.name?.[0].toUpperCase()}
        </AvatarFallback>
      </Avatar>

      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between">
          <p className="font-medium truncate">{profile.name}</p>
        </div>
        <div className="flex flex-row items-center">
          <p className="text-xs text-muted-foreground">
            {profile.macros.length} macros
          </p>
          <Show when={profile.functionKey}>
            <Badge variant="secondary" className="text-xs ml-2">
              <Keyboard className="h-3 w-3 mr-1" />
              {profile.functionKey}
            </Badge>
          </Show>
        </div>
      </div>
      <Show when={!open}>
        <div className="opacity-0 group-hover:opacity-100 transition-opacity flex items-center gap-1">
          <Button
            size="sm"
            variant="ghost"
            className="h-6 w-6 p-0"
            onClick={onToggle}
          >
            <Power
              className={cn(
                'h-3 w-3',
                isActive ? 'text-success' : 'text-muted-foreground'
              )}
            />
          </Button>

          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button size="sm" variant="ghost" className="h-6 w-6 p-0">
                <MoreVertical className="h-3 w-3" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem
                onClick={() => {
                  setOpen(true);
                  setValue(profile);
                }}
              >
                <Edit className="h-3 w-3 mr-2" />
                Editar
              </DropdownMenuItem>
              <DropdownMenuItem onClick={onDelete} className="text-destructive">
                <Trash2 className="h-3 w-3 mr-2" />
                Eliminar
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </Show>
    </div>
  );
}
