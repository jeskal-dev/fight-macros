import { execute } from '../../shared/services/execute';

export const sendKeydownEvent = (key: string) => {
  return execute('send_keydown_event', { key });
};
