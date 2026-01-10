'use client'

import { ReactNode } from 'react'

export interface CardProps {
  children: ReactNode
  className?: string
  shadow?: boolean
  onClick?: () => void
}

export interface CardHeaderProps {
  children: ReactNode
  className?: string
  action?: ReactNode
}

export interface CardBodyProps {
  children: ReactNode
  className?: string
}

export interface CardFooterProps {
  children: ReactNode
  className?: string
}

/**
 * Card - Composant FlyonUI intégré
 * 
 * @layer atoms
 * @tokens theme.colors.base-100, theme.shadow
 * @dependencies FlyonUI
 * 
 * @example
 * ```tsx
 * <Card>
 *   <CardHeader>Titre</CardHeader>
 *   <CardBody>Contenu</CardBody>
 *   <CardFooter>Actions</CardFooter>
 * </Card>
 * ```
 */
export function Card({ children, className = '', shadow = true, onClick }: CardProps) {
  return (
    <div 
      className={`card bg-base-100 border border-base-300 ${shadow ? 'shadow-sm' : ''} ${className}`}
      onClick={onClick}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
    >
      {children}
    </div>
  )
}

export function CardHeader({ children, className = '', action }: CardHeaderProps) {
  return (
    <div className={`card-header flex items-center justify-between gap-2 ${className}`}>
      {typeof children === 'string' ? <h4 className="card-title text-xl">{children}</h4> : children}
      {action}
    </div>
  )
}

export function CardBody({ children, className = '' }: CardBodyProps) {
  return <div className={`card-body ${className}`}>{children}</div>
}

export function CardFooter({ children, className = '' }: CardFooterProps) {
  return <div className={`card-footer ${className}`}>{children}</div>
}
