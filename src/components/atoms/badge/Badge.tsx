'use client'

import { ReactNode } from 'react'

export type BadgeVariant = 'primary' | 'secondary' | 'success' | 'error' | 'warning' | 'info' | 'neutral'
export type BadgeStyle = 'solid' | 'soft' | 'outline'
export type BadgeSize = 'xs' | 'sm' | 'md' | 'lg'

export interface BadgeProps {
  children: ReactNode
  variant?: BadgeVariant
  style?: BadgeStyle
  size?: BadgeSize
  rounded?: boolean
  className?: string
  icon?: ReactNode
}

/**
 * Badge - Composant FlyonUI intégré
 * 
 * @layer atoms
 * @tokens theme.colors.*
 * @dependencies FlyonUI
 * 
 * @example
 * ```tsx
 * <Badge variant="success" style="soft">Active</Badge>
 * <Badge variant="error" size="sm">Rejected</Badge>
 * ```
 */
export function Badge({
  children,
  variant = 'primary',
  style = 'soft',
  size = 'md',
  rounded = false,
  className = '',
  icon,
}: BadgeProps) {
  const sizeClass = {
    xs: 'badge-xs',
    sm: 'badge-sm',
    md: '',
    lg: 'badge-lg',
  }[size]

  const styleClass = style === 'soft' ? 'badge-soft' : style === 'outline' ? 'badge-outline' : ''
  const roundedClass = rounded ? 'rounded-field' : ''

  return (
    <span className={`badge badge-${variant} ${styleClass} ${sizeClass} ${roundedClass} ${className}`}>
      {icon && <span className="mr-1">{icon}</span>}
      {children}
    </span>
  )
}
