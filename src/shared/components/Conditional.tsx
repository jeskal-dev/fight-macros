import { isElement } from '@/shared/helpers/element';
import {
  Children,
  FC,
  isValidElement,
  memo,
  ReactElement,
  ReactNode,
} from 'react';

interface WhenProps {
  condition: boolean | (() => boolean);
  children: React.ReactNode;
}

export const When: FC<WhenProps> = ({ children }) => <>{children}</>;
When.displayName = 'When';

interface OtherwiseProps {
  children: React.ReactNode;
}

export const Otherwise: FC<OtherwiseProps> = ({ children }) => <>{children}</>;
Otherwise.displayName = 'Otherwise';

interface ChooseProps {
  children: ReactElement<WhenProps | OtherwiseProps>[];
}

export const Choose = memo<ChooseProps>(({ children }) => {
  const childGroup = Children.toArray(children).filter(isValidElement);

  // Primer <When> con condition === true gana
  for (const child of childGroup) {
    if (isElement(child, When)) {
      const condition =
        typeof child.props.condition === 'function'
          ? child.props.condition()
          : child.props.condition;
      if (condition) return child;
    }
  }

  // Devuelve el <Otherwise> si existe
  const otherwise = childGroup.find((c) => c.type === Otherwise);
  return otherwise || null;
});
Choose.displayName = 'Choose';

export interface ShowProps<T = unknown> {
  /** Cualquier valor o función que devuelva un valor */
  when: T;
  /** Contenido a renderizar si `when` es truthy */
  children: ReactNode;
  /** Contenido opcional si `when` es falsy */
  fallback?: ReactNode;
}

export const Show = memo(
  <T,>({ when, children, fallback = null }: ShowProps<T>) => {
    return when ? <>{children}</> : <>{fallback}</>;
  }
);
