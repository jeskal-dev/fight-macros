import { execute } from '@/shared/services/execute';

export function showKeyboardWindow() {
  return execute('show_keyboard_window');
}
