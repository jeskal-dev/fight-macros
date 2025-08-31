import { useGeneralKeyDownEffect } from '@/shared/hooks/useGeneralKeyDownEffect';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { PropsWithChildren, useEffect } from 'react';
import { useGlobalStoreInitializer } from '../stores/GlobalStore';
export function GlobalProvider({ children }: Readonly<PropsWithChildren>) {
  useGeneralKeyDownEffect();
  useGlobalStoreInitializer();

  useEffect(() => {
    const handleLoad = () => {
      WebviewWindow.getByLabel('splash').then((w) => {
        if (w) w.close();
        else console.log('Splash screen not found');
      });
      WebviewWindow.getCurrent().show();
    };
    handleLoad();
  }, []);

  return children;
}
