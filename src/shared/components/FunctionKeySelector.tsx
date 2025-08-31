import { cn } from '@/shared/lib/utils';
import { Button } from './ui/button';
import { Label } from './ui/label';
import { FunctionKey } from '@/shared/bindings/Key';

interface FunctionKeySelectorProps {
  keys: FunctionKey[];
  usedKeys: FunctionKey[];
  selectedKey: FunctionKey | null;
  onSelect: (key: FunctionKey | null) => void;
}

export const FunctionKeySelector: React.FC<FunctionKeySelectorProps> = ({
  keys,
  usedKeys,
  selectedKey,
  onSelect,
}) => {
  return (
    <div className="space-y-2">
      <Label className="text-sm font-medium">Tecla rápida (opcional)</Label>
      <div className="grid grid-cols-4 gap-2">
        {keys.map((key) => (
          <Button
            key={key}
            size="sm"
            variant={selectedKey === key ? 'default' : 'outline'}
            className={cn(
              'h-8 text-xs',
              usedKeys.includes(key) && 'opacity-50 cursor-not-allowed'
            )}
            onClick={() => {
              if (!usedKeys.includes(key)) {
                onSelect(key === selectedKey ? null : key);
              }
            }}
            disabled={usedKeys.includes(key)}
          >
            {key}
          </Button>
        ))}
      </div>
      {selectedKey && usedKeys.includes(selectedKey) && (
        <p className="text-xs text-destructive">Esta tecla ya está en uso</p>
      )}
    </div>
  );
};
