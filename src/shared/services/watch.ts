import { ListenableChannel } from "@/shared/bindings/ListenableChannel";
import { Event, listen } from "@tauri-apps/api/event";

export function watch<T>(
  channel: ListenableChannel,
  callback: (data: Event<T>) => void
) {
  return listen<T>(channel, callback);
}
