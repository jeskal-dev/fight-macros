import { MacroDto } from '@/main/domain/dtos/macro';
import { cn } from '@/shared/lib/utils';
import { Clock, KeyRound } from 'lucide-react';
import { MacroItemMenu } from './MacroItemMenu';
import { Badge } from '../../shared/components/ui/badge';
import { useMacroFormState } from '../stores/MacroFormStore';

interface MacroItemProps {
  macro: MacroDto;
  isSelected: boolean;
  onSelect: () => void;
  onEdit: () => void;
  onDelete: () => void;
  onDuplicate: () => void;
}

export function MacroItem({
  macro,
  isSelected,
  onSelect,
  onEdit,
  onDelete,
  onDuplicate,
}: Readonly<MacroItemProps>) {
  const { open } = useMacroFormState();

  return (
    <div
      className={cn(
        'group relative border rounded-md transition-all',
        isSelected
          ? 'bg-accent/50 border-primary/50'
          : 'hover:bg-accent/30 border-border'
      )}
      onClick={!open ? onSelect : undefined}
    >
      <div className="p-3">
        <div className="flex items-center justify-between">
          <div className="flex-1 min-w-0">
            <p className="text-sm font-medium truncate">{macro.name}</p>
            <div className="flex items-center gap-2 mt-1">
              <Badge variant="outline" className="text-[10px] px-1.5 py-0">
                <KeyRound className="h-2.5 w-2.5 mr-1" />
                {macro.triggerKey}
              </Badge>
              <Badge variant="secondary" className="text-[10px] px-1.5 py-0">
                <Clock className="h-2.5 w-2.5 mr-1" />
                {macro.sequence.length} pasos
              </Badge>
            </div>
          </div>

          <MacroItemMenu
            key={macro.id}
            onDuplicate={onDuplicate}
            onDelete={onDelete}
            onEdit={onEdit}
          />
        </div>
      </div>
    </div>
  );
}
