import { useActiveTheme } from '@/components/hooks/useActiveTheme'

export function useTextStyles() {
  const theme = useActiveTheme()

  const textPrimaryStyle = {
    color: theme.colors.section.title
  }

  const textSecondaryStyle = {
    color: theme.colors.section.description
  }

  return { textPrimaryStyle, textSecondaryStyle }
}
