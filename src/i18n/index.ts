import { createMemo, createResource } from "solid-js";
import { en } from "./en";

const translations: Record<string, Record<string, any>> = {
  en,
};

export function useTranslation(lang: () => string) {
  const [data] = createResource(lang, async (l) => {
    if (translations[l]) return translations[l];
    try {
      const mod = await import(`./${l}.ts`);
      translations[l] = mod[l] || mod.default;
      return translations[l];
    } catch {
      return en;
    }
  });

  const t = createMemo(() => (key: string) => {
    const dict = data();
    if (!dict) return key;
    const keys = key.split(".");
    let val: any = dict;
    for (const k of keys) {
      val = val?.[k];
    }
    return val ?? key;
  });

  return t;
}
