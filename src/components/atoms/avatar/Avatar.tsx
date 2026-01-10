'use client'

import { ReactNode } from 'react'

export type AvatarSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'
export type AvatarStatus = 'online' | 'offline' | 'away' | 'busy' | 'none'

export interface AvatarProps {
  src?: string
  alt?: string
  size?: AvatarSize
  status?: AvatarStatus
  placeholder?: ReactNode
  rounded?: 'full' | 'field'
  className?: string
}

/**
 * Avatar - Composant FlyonUI intégré
 * 
 * @layer atoms
 * @tokens theme.colors.*
 * @dependencies FlyonUI
 * 
 * @example
 * ```tsx
 * <Avatar src="/avatar.png" size="md" status="online" />
 * <Avatar placeholder={<span>JD</span>} />
 * ```
 */
export function Avatar({
  src,
  alt = 'Avatar',
  size = 'md',
  status = 'none',
  placeholder,
  rounded = 'full',
  className = '',
}: AvatarProps) {
  const sizeClasses = {
    xs: 'size-6',
    sm: 'size-8',
    md: 'size-10',
    lg: 'size-12',
    xl: 'size-16',
  }

  const statusClass = status !== 'none' ? `avatar-${status === 'online' ? 'online-top' : status}` : ''
  const roundedClass = rounded === 'full' ? 'rounded-full' : 'rounded-field'

  return (
    <div className={`avatar ${statusClass} ${placeholder ? 'avatar-placeholder' : ''} ${className}`}>
      <div className={`${sizeClasses[size]} ${roundedClass}`}>
        {src ? (
          <img src={src} alt={alt} />
        ) : placeholder ? (
          placeholder
        ) : (
          <span className="icon-[tabler--user] size-5" />
        )}
      </div>
    </div>
  )
}
