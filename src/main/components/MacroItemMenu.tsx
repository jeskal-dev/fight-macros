import { Copy, Edit, MoreVertical, Trash2 } from 'lucide-react';
import { Button } from '../../shared/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../../shared/components/ui/dropdown-menu';

interface Props {
  onDuplicate: () => void;
  onDelete: () => void;

  onEdit: () => void;
}

export const MacroItemMenu: React.FC<Props> = ({
  onDuplicate,
  onDelete,
  onEdit,
}) => (
  <DropdownMenu>
    <DropdownMenuTrigger asChild>
      <Button
        size="sm"
        variant="ghost"
        className="h-6 w-6 p-0 opacity-0 group-hover:opacity-100 transition-opacity"
      >
        <MoreVertical className="h-3 w-3" />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end">
      <DropdownMenuItem onClick={onEdit}>
        <Edit className="h-3 w-3 mr-2" />
        Editar
      </DropdownMenuItem>
      <DropdownMenuItem onClick={onDuplicate}>
        <Copy className="h-3 w-3 mr-2" />
        Duplicar
      </DropdownMenuItem>
      <DropdownMenuItem onClick={onDelete} className="text-destructive">
        <Trash2 className="h-3 w-3 mr-2" />
        Eliminar
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
);
