import { cn } from '@/shared/lib/utils';
import { cva } from 'class-variance-authority';
import {
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  Command,
  CornerDownLeft,
  Space,
  Undo2,
} from 'lucide-react';
import { forwardRef, useState } from 'react';

const keyVariants = cva(
  cn(
    'inline-flex items-center justify-center rounded-md border border-border',
    'bg-card text-card-foreground shadow-sm',
    'transition-all hover:bg-accent hover:text-accent-foreground',
    'active:scale-95 focus:outline-none focus:ring-2 focus:ring-ring',
    'select-none font-medium'
  ),
  {
    variants: {
      size: {
        default: 'h-12 min-w-[3rem] text-sm',
        sm: 'h-10 min-w-[2.5rem] text-xs',
        lg: 'h-12 min-w-[4.5rem] text-sm',
        xl: 'h-12 min-w-[6rem] text-sm',
        '2xl': 'h-12 min-w-[8rem] text-sm',
        space: 'h-12 flex-1 text-sm',
        jumbo: 'h-12 min-w-[12rem] text-sm',
      },
      active: {
        true: 'bg-accent text-accent-foreground',
      },
    },
    defaultVariants: {
      size: 'default',
    },
  }
);

export interface KeyProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  size?: 'default' | 'sm' | 'lg' | 'xl' | '2xl' | 'space' | 'jumbo';
  active?: boolean;
}

const Key = forwardRef<HTMLButtonElement, KeyProps>(
  ({ className, size, active, children, ...props }, ref) => (
    <button
      ref={ref}
      type="button"
      className={cn(keyVariants({ size, active, className }))}
      {...props}
    >
      {children}
    </button>
  )
);

type KeyboardProps = {
  onInput?: (text: string) => void;
};

function KeyboardLayout({ onInput }: Readonly<KeyboardProps>) {
  const [buffer, setBuffer] = useState('');
  const [caps, setCaps] = useState(false);
  const [shift, setShift] = useState(false);

  const toggleCaps = () => setCaps((c) => !c);
  const toggleShift = () => setShift((s) => !s);

  const handleKey = (key: string) => {
    let newBuffer = buffer;

    switch (key) {
      case 'Backspace':
        newBuffer = buffer.slice(0, -1);
        break;
      case 'Enter':
        newBuffer += '\n';
        break;
      case 'Tab':
        newBuffer += '\t';
        break;
      case ' ':
        newBuffer += ' ';
        break;
      default:
        if (key.length === 1) {
          const char = shift ? key.toUpperCase() : key.toLowerCase();
          newBuffer += caps ? char.toUpperCase() : char;
        } else {
          newBuffer += key;
        }
    }

    setBuffer(newBuffer);
    onInput?.(newBuffer);

    // Reset shift after typing a character (like real keyboards)
    if (key.length === 1 && shift) {
      setShift(false);
    }
  };

  // Function keys row
  const functionKeys = [
    'Esc',
    'F1',
    'F2',
    'F3',
    'F4',
    'F5',
    'F6',
    'F7',
    'F8',
    'F9',
    'F10',
    'F11',
    'F12',
  ];

  // Number row with shift symbols
  const numberRow = [
    { key: '`', shift: '~' },
    { key: '1', shift: '!' },
    { key: '2', shift: '@' },
    { key: '3', shift: '#' },
    { key: '4', shift: '$' },
    { key: '5', shift: '%' },
    { key: '6', shift: '^' },
    { key: '7', shift: '&' },
    { key: '8', shift: '*' },
    { key: '9', shift: '(' },
    { key: '0', shift: ')' },
    { key: '-', shift: '_' },
    { key: '=', shift: '+' },
  ];

  // Main letter rows
  const topRow = [
    'q',
    'w',
    'e',
    'r',
    't',
    'y',
    'u',
    'i',
    'o',
    'p',
    '[',
    ']',
    '\\',
  ];
  const homeRow = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', "'"];
  const bottomRow = ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'];

  return (
    <div className="flex max-w-5xl flex-col gap-2 rounded-lg bg-card p-4 shadow-lg">
      {/* Function Keys */}
      <div className="flex gap-1">
        {functionKeys.map((k) => (
          <Key
            key={k}
            size={k === 'Esc' ? 'lg' : 'sm'}
            onClick={() => handleKey(k)}
          >
            {k}
          </Key>
        ))}
      </div>

      {/* Number Row */}
      <div className="flex gap-1">
        {numberRow.map(({ key, shift: shiftChar }) => (
          <Key key={key} onClick={() => handleKey(shift ? shiftChar : key)}>
            <div className="flex flex-col items-center">
              <span className="text-xs text-muted-foreground">{shiftChar}</span>
              <span>{key}</span>
            </div>
          </Key>
        ))}
        <Key size="xl" onClick={() => handleKey('Backspace')}>
          <Undo2 className="h-4 w-4" />
        </Key>
      </div>

      {/* Top Row */}
      <div className="flex gap-1">
        <Key size="xl" onClick={() => handleKey('Tab')}>
          Tab
        </Key>
        {topRow.map((k) => (
          <Key key={k} onClick={() => handleKey(k)}>
            {shift ? k.toUpperCase() : k}
          </Key>
        ))}
      </div>

      {/* Home Row */}
      <div className="flex gap-1">
        <Key size="xl" active={caps} onClick={toggleCaps}>
          Caps
        </Key>
        {homeRow.map((k) => (
          <Key key={k} onClick={() => handleKey(k)}>
            {shift ? k.toUpperCase() : k}
          </Key>
        ))}
        <Key size="xl" onClick={() => handleKey('Enter')}>
          <CornerDownLeft className="h-4 w-4" />
        </Key>
      </div>

      {/* Bottom Row */}
      <div className="flex gap-1">
        <Key size="2xl" active={shift} onClick={toggleShift}>
          Shift
        </Key>
        {bottomRow.map((k) => (
          <Key key={k} onClick={() => handleKey(k)}>
            {shift ? k.toUpperCase() : k}
          </Key>
        ))}
        <Key size="2xl" active={shift} onClick={toggleShift}>
          Shift
        </Key>
      </div>

      {/* Bottom Row with Space */}
      <div className="flex gap-1">
        <Key size="lg" onClick={() => handleKey('Ctrl')}>
          Ctrl
        </Key>
        <Key size="lg" onClick={() => handleKey('Win')}>
          <Command className="h-4 w-4" />
        </Key>
        <Key size="lg" onClick={() => handleKey('Alt')}>
          Alt
        </Key>
        <Key size="space" onClick={() => handleKey(' ')}>
          <Space className="h-4 w-4" />
        </Key>
        <Key size="lg" onClick={() => handleKey('Alt')}>
          Alt
        </Key>
        <Key size="lg" onClick={() => handleKey('Fn')}>
          Fn
        </Key>
        <Key size="lg" onClick={() => handleKey('Ctrl')}>
          Ctrl
        </Key>
      </div>

      {/* Arrow Keys and Navigation */}
      <div className="flex gap-1 justify-between">
        <div className="flex gap-1">
          <Key size="sm" onClick={() => handleKey('Insert')}>
            Ins
          </Key>
          <Key size="sm" onClick={() => handleKey('Home')}>
            Home
          </Key>
          <Key size="sm" onClick={() => handleKey('PageUp')}>
            PgUp
          </Key>
        </div>

        <div className="flex gap-1 items-center">
          <div className="flex flex-col gap-1">
            <Key size="sm" onClick={() => handleKey('PrintScreen')}>
              PrtSc
            </Key>
            <Key size="sm" onClick={() => handleKey('Delete')}>
              Del
            </Key>
          </div>
          <div className="flex flex-col gap-1">
            <Key size="sm" className="justify-center">
              <ArrowUp className="h-4 w-4" />
            </Key>
            <div className="flex gap-1">
              <Key size="sm">
                <ArrowLeft className="h-4 w-4" />
              </Key>
              <Key size="sm">
                <ArrowDown className="h-4 w-4" />
              </Key>
              <Key size="sm">
                <ArrowRight className="h-4 w-4" />
              </Key>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export { KeyboardLayout };
