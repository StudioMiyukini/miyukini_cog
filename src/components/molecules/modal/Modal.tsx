'use client'

import { useEffect } from 'react'

export type ModalProps = {
  open: boolean
  title?: string
  children: React.ReactNode
  footer?: React.ReactNode
  onClose: () => void
}

export function Modal({ open, title, children, footer, onClose }: ModalProps) {
  useEffect(() => {
    if (!open) return
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    document.addEventListener('keydown', onKeyDown)
    return () => document.removeEventListener('keydown', onKeyDown)
  }, [onClose, open])

  if (!open) return null

  return (
    <div className="fixed inset-0 z-[100]">
      <div className="absolute inset-0 bg-black/50" onClick={onClose} />
      <div className="absolute inset-0 flex items-end sm:items-center justify-center p-3">
        <div className="w-full max-w-2xl bg-base-100 rounded-2xl shadow-xl border border-base-300 overflow-hidden">
          {(title || footer) && (
            <div className="p-4 border-b border-base-300 flex items-center justify-between gap-3">
              <div className="min-w-0">
                {title && <div className="font-semibold text-base-content truncate">{title}</div>}
              </div>
              <button className="btn btn-ghost btn-circle btn-sm" onClick={onClose} aria-label="Fermer">
                <span className="icon-[tabler--x] size-5" />
              </button>
            </div>
          )}

          <div className="p-4 max-h-[75vh] overflow-auto">{children}</div>

          {footer && <div className="p-4 border-t border-base-300 bg-base-200/40">{footer}</div>}
        </div>
      </div>
    </div>
  )
}

