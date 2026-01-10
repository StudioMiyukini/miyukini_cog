import { useEffect, useState } from 'react'

export function useIsMobile(breakpoint = 800) {
  const [isMobile, setIsMobile] = useState(true)

  useEffect(() => {
    const update = () => setIsMobile(window.innerWidth < breakpoint)
    update()
    window.addEventListener('resize', update)
    return () => window.removeEventListener('resize', update)
  }, [breakpoint])

  return { isMobile }
}
