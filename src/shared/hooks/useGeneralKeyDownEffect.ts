import { sendKeydownEvent } from '@/main/services/key';
import { useCallback, useEffect } from 'react';

export const useGeneralKeyDownEffect = () => {
  const handleKeyDown = useCallback(async (event: KeyboardEvent) => {
    if (event.key.startsWith('F') && event.key.length > 1) return;
    const result = await sendKeydownEvent(event.key);
    if (result.isErr()) console.log(result.error);
  }, []);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
};
