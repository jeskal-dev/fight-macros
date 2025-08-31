import { SequenceStepDto } from '@/main/domain/dtos/sequence';
import { cn } from '@/shared/lib/utils';
import { useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import {
  ArrowDown,
  ArrowUp,
  Clock,
  Edit,
  GripVertical,
  Trash2,
} from 'lucide-react';
import {
  Choose,
  Otherwise,
  Show,
  When,
} from '../../shared/components/Conditional';
import { Badge } from '../../shared/components/ui/badge';
import { Button } from '../../shared/components/ui/button';

interface Props {
  step: SequenceStepDto;
  index: number;
  onUpdate?: () => void;
  onDelete?: () => void;
}

export function SequenceStepItem({
  step,
  index,
  onUpdate,
  onDelete,
}: Readonly<Props>) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: step.id.toString() });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    zIndex: isDragging ? 100 : 'auto',
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={cn(
        'group grid grid-cols-[30px_40px_80px_80px_1fr] items-center p-3 border rounded-md bg-background hover:bg-accent/50 transition-all gap-2',
        isDragging && 'shadow-lg scale-105'
      )}
    >
      <div
        {...attributes}
        {...listeners}
        className="cursor-grab active:cursor-grabbing justify-self-start"
      >
        <GripVertical className="h-4 w-4 text-muted-foreground" />
      </div>

      <Badge
        variant="secondary"
        className="text-xs w-6 h-6 p-0 flex items-center justify-center justify-self-start"
      >
        {index + 1}
      </Badge>

      <Choose>
        <When condition={() => step.type === 'delay'}>
          <div className="flex flex-row items-center gap-2">
            <Clock className="h-4 w-4 text-info flex-shrink-0" />
            <span className="text-sm truncate">Delay</span>
          </div>
          <Badge variant="outline" className="text-sm flex-shrink-0">
            {step.delay}ms
          </Badge>
        </When>
        <Otherwise>
          <div className="flex flex-row items-center gap-2">
            <Choose>
              <When condition={() => step.type === 'keydown'}>
                <ArrowDown className="h-4 w-4 text-success flex-shrink-0" />
              </When>
              <Otherwise>
                <ArrowUp className="h-4 w-4 text-destructive flex-shrink-0" />
              </Otherwise>
            </Choose>
            <span className="text-sm capitalize truncate">
              {step.type.replace('key', '')}
            </span>
          </div>
          <Badge variant="outline" className="text-sm font-mono flex-shrink-0">
            {step.key}
          </Badge>
        </Otherwise>
      </Choose>

      <div className="opacity-0 group-hover:opacity-100 transition-opacity flex items-center gap-1 justify-self-end">
        <Show when={onUpdate}>
          <Button
            size="sm"
            variant="ghost"
            className="h-6 w-6 p-0"
            onClick={onUpdate}
          >
            <Edit className="h-3 w-3" />
          </Button>
        </Show>

        <Show when={onDelete}>
          <Button
            size="sm"
            variant="ghost"
            className="h-6 w-6 p-0 text-destructive"
            onClick={onDelete}
          >
            <Trash2 className="h-3 w-3" />
          </Button>
        </Show>
      </div>
    </div>
  );
}
