import { memorize } from '@/shared/helpers/memorize';
import { Fragment, useMemo } from 'react';

export interface SmartListProps<T> {
  data: T[] | (() => T[]);
  children: (item: T, index: number) => React.ReactNode;
  fallback?: React.ReactNode;
  keyFn?: (item: T, index: number) => React.Key;
}

export const SmartList = memorize(
  <T,>({ data, children, fallback = null, keyFn }: SmartListProps<T>) => {
    const items = useMemo(
      () => (typeof data === 'function' ? data() : data),
      [data]
    );

    if (!items || items.length === 0) return <>{fallback}</>;

    return (
      <>
        {items.map((item, idx) => (
          <Fragment key={keyFn ? keyFn(item, idx) : JSON.stringify(item)}>
            {children(item, idx)}
          </Fragment>
        ))}
      </>
    );
  }
);
