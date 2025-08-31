import { Config } from "@/shared/bindings/Config";
import { execute } from "../../shared/services/execute";

export function loadConfig() {
  return execute<Config>("load_config");
}

export function saveConfig(config: Config) {
  return execute("save_config", {config});
}

export function changeActiveProfile(id: number) {
  return execute("change_active_profile", {id});
}