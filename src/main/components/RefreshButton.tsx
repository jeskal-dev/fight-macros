import { cn } from '@/shared/lib/utils';
import { ComponentProps, useState } from 'react';
import { Button } from '../../shared/components/ui/button';

type Props = ComponentProps<typeof Button>;

export function RefreshButton({ onClick, children, ...props }: Props) {
  const [rotate, setRotate] = useState(false);

  const handleClick = (ev: React.MouseEvent<HTMLButtonElement>) => {
    ev.stopPropagation();
    onClick?.(ev);
    if (rotate) return; // evita doble-trigger
    setRotate(true);
  };

  const handleTransitionEnd = () => {
    setRotate(false);
  };

  return (
    <Button onClick={handleClick} {...props}>
      <div
        className={cn(
          rotate && 'transition-transform duration-500 ease-in-out rotate-360'
        )}
        onTransitionEnd={handleTransitionEnd}
      >
        {children}
      </div>
    </Button>
  );
}
