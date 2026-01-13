export function ContentStack({ children, className }: { children: React.ReactNode; className?: string }) {
  return (
    <div className={['content-stack', className].filter(Boolean).join(' ')}>
      {children}
    </div>
  )
}
