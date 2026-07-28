import { createStore } from "solid-js/store";

interface SettingsState {
  language: string;
  theme: "dark" | "light" | "system";
  memoryEnabled: boolean;
  temperature: number;
}

function loadSettings(): SettingsState {
  return {
    language: localStorage.getItem("mar_language") || "en",
    theme: (localStorage.getItem("mar_theme") as "dark" | "light" | "system") || "dark",
    memoryEnabled: localStorage.getItem("mar_memory") !== "false",
    temperature: parseFloat(localStorage.getItem("mar_temperature") || "0.7"),
  };
}

function createSettingsStoreSingleton() {
  const [settings, setSettings] = createStore<SettingsState>(loadSettings());

  return {
    ...settings,
    setLanguage: (lang: string) => {
      localStorage.setItem("mar_language", lang);
      setSettings("language", lang);
    },
    setTheme: (theme: "dark" | "light" | "system") => {
      localStorage.setItem("mar_theme", theme);
      setSettings("theme", theme);
      document.documentElement.className = theme === "system"
        ? (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light")
        : theme;
    },
    setMemoryEnabled: (enabled: boolean) => {
      localStorage.setItem("mar_memory", String(enabled));
      setSettings("memoryEnabled", enabled);
    },
    setTemperature: (temp: number) => {
      localStorage.setItem("mar_temperature", String(temp));
      setSettings("temperature", temp);
    },
  };
}

const store = createSettingsStoreSingleton();
export function createSettingsStore() {
  return store;
}
