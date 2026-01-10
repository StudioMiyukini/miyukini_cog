export const themes = {
  standard: {
    colors: {
      section: {
        background: '#0b1120',
        card: { background: '#151f33', border: '#1f2937' },
        border: '#1f2937',
        title: '#f8fafc',
        description: '#cbd5f5'
      },
      text: {
        primary: '#f8fafc',
        secondary: '#cbd5f5'
      },
      accent: '#38bdf8'
    }
  },
  dark: {
    colors: {
      section: {
        background: '#05070d',
        card: { background: '#0c1220', border: '#111827' },
        border: '#111827',
        title: '#f1f5f9',
        description: '#94a3b8'
      },
      text: {
        primary: '#f1f5f9',
        secondary: '#cbd5f5'
      },
      accent: '#14b8a6'
    }
  },
  oasis: {
    colors: {
      section: {
        background: '#032c2a',
        card: { background: '#0b3b35', border: '#164d46' },
        border: '#0b3b35',
        title: '#e0f2fe',
        description: '#bae6fd'
      },
      text: {
        primary: '#e0f2fe',
        secondary: '#bae6fd'
      },
      accent: '#34d399'
    }
  }
} as const

export type ThemeName = keyof typeof themes

export function useActiveTheme(themeName: ThemeName = 'standard') {
  return themes[themeName]
}
