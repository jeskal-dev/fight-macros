import { Button } from '@/shared/components/ui/button';
import { Keyboard } from 'lucide-react';
import { useCallback } from 'react';
import { showKeyboardWindow } from '../services/keyboard';
import { useGlobalStoreActions } from '../stores/GlobalStore';
import { memorize } from '@/shared/helpers/memorize';

export const KeyboardButton = memorize(() => {
  const { setError } = useGlobalStoreActions();

  const handleClick = useCallback(async () => {
    try {
      const result = await showKeyboardWindow();
      if (result.isErr()) setError(result.error);
    } catch (e) {
      if (e instanceof Error) {
        setError(e.message);
        return;
      }
      setError('Unknown error showing keyboard');
    }
  }, [setError]);

  return (
    <Button size="sm" variant="ghost" className="size-6" onClick={handleClick}>
      <Keyboard className="size-4" />
    </Button>
  );
});
