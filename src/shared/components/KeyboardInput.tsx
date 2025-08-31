import { Key } from '@/shared/bindings/Key';
import { ModifierKey } from '@/shared/bindings/ModifierKey';
import { KEY_ALIASES, KEY_LIST } from '@/shared/constants/keys_aliases';
import { MODIFIER_LIST } from '@/shared/constants/modifiers_aliases';
import { cn } from '@/shared/lib/utils';
import { Keyboard } from 'lucide-react';
import { forwardRef, useImperativeHandle, useRef, useState } from 'react';
import { Badge } from './ui/badge';
import { Input } from './ui/input';

const isModifier = (key: Key) => MODIFIER_LIST.includes(key as ModifierKey);
const isAllowed = (
  key: Key,
  allowedKeys: Key[] | undefined,
  disableModifiers: boolean
) => {
  if (disableModifiers && isModifier(key)) return false;

  if (allowedKeys)
    return (
      allowedKeys.includes(key.toUpperCase() as Key) ||
      allowedKeys.includes(key)
    );
  return true;
};

interface KeyboardInputProps {
  value: string;
  onKeyChange: (value: string) => void;
  onKeyError?: (error: string) => void;
  placeholder?: string;
  className?: string;
  disabled?: boolean;
  disableModifiers?: boolean;
  allowedKeys?: Key[];
}

export interface KeyboardInputHandle {
  focus: () => void;
  startRecording: () => void;
  cancel: () => void;
}

export interface KeyEvent {
  key: string;
  ctrlKey: boolean;
  shiftKey: boolean;
  altKey: boolean;
  metaKey: boolean;
}

export const KeyboardInput = forwardRef<
  KeyboardInputHandle,
  KeyboardInputProps
>(
  (
    {
      value,
      onKeyChange,
      className,
      disabled,
      placeholder,
      onKeyError,
      allowedKeys = KEY_LIST,
      disableModifiers = false,
    },
    ref
  ) => {
    const inputRef = useRef<HTMLInputElement>(null);
    const [state, setState] = useState({
      isRecording: false,
      display: value,
      pressed: new Set<Key>(),
    });

    useImperativeHandle(ref, () => ({
      focus: () => inputRef.current?.focus?.(),
      startRecording: () =>
        !disabled &&
        setState({
          isRecording: true,
          pressed: new Set(),
          display: '',
        }),
      cancel: () => cancelRecording(),
    }));

    const getNonModifierKeys = (keys: ReadonlySet<Key>): Key[] => {
      return Array.from(keys.keys()).filter((k) => !isModifier(k));
    };

    const formatKeys = (keys: ReadonlySet<Key>) => {
      const parts: string[] = [];
      MODIFIER_LIST.forEach((m) => {
        if (keys.has(m as Key)) {
          parts.push(KEY_ALIASES[m as Key]);
        }
      });
      keys.forEach((k) => {
        if (!MODIFIER_LIST.includes(k as ModifierKey))
          parts.push(KEY_ALIASES[k] || k.toString().toUpperCase());
      });
      return parts.join(' + ');
    };

    const cancelRecording = () =>
      setState({
        isRecording: false,
        pressed: new Set(),
        display: value,
      });

    const handleKeyDown = (e: React.KeyboardEvent) => {
      if (disabled || !state.isRecording) return;
      if (e.key === 'Escape') return cancelRecording();
      if (e.repeat) return;

      const key = e.key as Key;

      if (!isAllowed(key, allowedKeys, disableModifiers)) {
        onKeyError?.('La combinación de teclas no es válida');
        return;
      }
      e.preventDefault();

      const newSet = new Set(state.pressed);
      newSet.add(e.key as Key);

      if (disableModifiers && newSet.size > 1) {
        onKeyError?.('No se pueden usar modificadores con múltiples teclas');
        return;
      }

      if (!disableModifiers) {
        const nonModifiers = getNonModifierKeys(newSet);
        if (nonModifiers.length > 1) {
          onKeyError?.('No se pueden usar múltiples teclas sin modificadores');
          return;
        }
      }

      const formatted = formatKeys(newSet);
      setState((prev) => ({ ...prev, display: formatted, pressed: newSet }));
    };

    const handleKeyUp = (e: React.KeyboardEvent) => {
      if (disabled || !state.isRecording) return;
      e.preventDefault();
      const capturedCombo = new Set(state.pressed);
      const remaining = new Set(state.pressed);
      remaining.delete(e.key as Key);

      setState((prev) => ({
        ...prev,
        pressed: remaining,
      }));

      if (capturedCombo.size > 0) {
        const finalCombo = formatKeys(capturedCombo);
        onKeyChange(finalCombo);
        setState((prev) => ({
          ...prev,
          isRecording: false,
          display: finalCombo,
        }));
      }
    };

    const handleClick = () => {
      if (!disabled)
        setState((prev) => ({
          ...prev,
          isRecording: true,
          pressed: new Set(),
          display: '',
        }));
    };

    const handleBlur = () => state.isRecording && cancelRecording();

    return (
      <div className="relative w-full">
        <Input
          ref={inputRef}
          type="text"
          value={state.isRecording ? state.display || 'Recording…' : value}
          onClick={handleClick}
          onKeyDown={handleKeyDown}
          onKeyUp={handleKeyUp}
          onBlur={handleBlur}
          placeholder={placeholder}
          className={cn(
            'cursor-pointer select-none font-mono',
            state.isRecording && 'ring-1 ring-primary ring-offset-2',
            disabled && 'cursor-not-allowed opacity-50',
            className
          )}
          readOnly
          aria-describedby="kbd-hint"
          disabled={disabled}
        />

        <div
          className={cn(
            'absolute right-1 top-1/2 -translate-y-1/2 pointer-events-none transition-transform',
            state.isRecording && 'translate-y-5/6'
          )}
        >
          <Badge
            variant={state.isRecording ? 'default' : 'outline'}
            className={cn('text-xs flex items-center gap-1')}
          >
            <Keyboard className="h-3 w-3" />
            {state.isRecording && 'ESC to cancel'}
          </Badge>
        </div>

        {/* <Show when={state.value.isRecording && state.value.pressed.size > 0}>
        <SmartList
          data={Array.from(state.value.pressed)}
          keyFn={(key, i) => `${i}-${key}`}
        >
          {(key) => (
            <Badge variant="secondary" className="text-xs">
              {getDisplayKey(key)}
            </Badge>
          )}
        </SmartList>
      </Show> */}
      </div>
    );
  }
);

KeyboardInput.displayName = 'KeyboardInput';
