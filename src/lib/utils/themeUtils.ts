export function colorToRgba(color: string, opacity: number) {
  if (color.startsWith('#')) {
    const hex = color.replace('#', '')
    const bigint = parseInt(hex, 16)
    const r = (bigint >> 16) & 255
    const g = (bigint >> 8) & 255
    const b = bigint & 255
    return `rgba(${r}, ${g}, ${b}, ${opacity})`
  }
  return color
}

export function getModalOverlayStyle(theme: ReturnType<typeof colorToRgba> | null, opacity = 0.5) {
  return {
    background: `rgba(0,0,0,${opacity})`
  }
}
