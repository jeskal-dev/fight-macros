import { cn } from '@/shared/lib/utils';
import { ReactNode } from 'react';

interface SectionHeaderProps {
  title: string;
  subtitle?: ReactNode;
  actions?: ReactNode;
  className?: string;
}

export function SectionHeader({
  title,
  subtitle,
  actions,
  className,
}: Readonly<SectionHeaderProps>) {
  return (
    <div
      className={cn(
        'flex items-center justify-between px-4 py-3 border-b border-border bg-background min-h-[4rem]',
        className
      )}
    >
      <div className="flex-1 min-w-0">
        <h3 className="font-semibold text-lg leading-none">{title}</h3>
        {subtitle && (
          <p className="text-xs text-muted-foreground mt-0.5">{subtitle}</p>
        )}
      </div>
      {actions && <div className="flex items-center gap-1 ml-2">{actions}</div>}
    </div>
  );
}
