import React, { createContext, useContext, useState, useEffect, useMemo, ReactNode } from 'react';
import { Language, Translations, translations, moduleNames } from './translations';

interface LanguageContextType {
  language: Language;
  setLanguage: (lang: Language) => void;
  t: Translations;
  getModuleName: (id: string, fallbackName: string) => string;
}

const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

const STORAGE_KEY = 'sound_to_osc_lang';

export const LanguageProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [language, setLanguageState] = useState<Language>(() => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored === 'es' || stored === 'en') {
        return stored;
      }
    } catch {
      // Local storage might be disabled or unavailable
    }
    // Default to Spanish as requested
    return 'es';
  });

  const setLanguage = (lang: Language) => {
    setLanguageState(lang);
    try {
      localStorage.setItem(STORAGE_KEY, lang);
    } catch {
      // Ignore local storage error
    }
  };

  useEffect(() => {
    document.documentElement.lang = language;
  }, [language]);

  const t = useMemo(() => translations[language], [language]);

  const getModuleName = (id: string, fallbackName: string): string => {
    const localized = moduleNames[language]?.[id];
    return localized || fallbackName;
  };

  const value = useMemo(
    () => ({
      language,
      setLanguage,
      t,
      getModuleName,
    }),
    [language, t]
  );

  return <LanguageContext.Provider value={value}>{children}</LanguageContext.Provider>;
};

export const useLanguage = (): LanguageContextType => {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error('useLanguage must be used within a LanguageProvider');
  }
  return context;
};
