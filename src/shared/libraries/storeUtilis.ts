import { LazyStore } from "@tauri-apps/plugin-store";

export const store = new LazyStore('settings.json')

export function setUserData(userData: object) {
    sessionStorage.setItem('userData', JSON.stringify(userData));
}
