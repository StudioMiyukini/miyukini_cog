'use client'

import { ReactNode } from 'react'

export type TrendDirection = 'up' | 'down' | 'neutral'

export interface StatsCardProps {
  title: string
  value: string | number
  icon?: ReactNode
  trend?: {
    value: number
    direction: TrendDirection
  }
  subtitle?: string
  className?: string
}

/**
 * StatsCard - Carte de statistique FlyonUI
 * 
 * @layer atoms
 * @tokens theme.colors.*, theme.spacing.*
 * @dependencies FlyonUI, @iconify-json/tabler
 * 
 * @example
 * ```tsx
 * <StatsCard 
 *   title="Pageviews" 
 *   value="17,356" 
 *   icon={<span className="icon-[tabler--eye]" />}
 *   trend={{ value: 25.6, direction: 'up' }}
 *   subtitle="EPC: 308.20"
 * />
 * ```
 */
export function StatsCard({
  title,
  value,
  icon,
  trend,
  subtitle,
  className = '',
}: StatsCardProps) {
  const trendColor = trend?.direction === 'up' 
    ? 'text-success' 
    : trend?.direction === 'down' 
      ? 'text-error' 
      : 'text-base-content/50'

  const trendIcon = trend?.direction === 'up' 
    ? 'icon-[tabler--arrow-up]' 
    : trend?.direction === 'down' 
      ? 'icon-[tabler--arrow-down]' 
      : ''

  return (
    <div className={`flex flex-1 flex-col gap-4 ${className}`}>
      <div className="text-base-content flex items-center gap-2">
        {icon && (
          <div className="avatar avatar-placeholder">
            <div className="bg-base-200 rounded-field size-9 flex items-center justify-center">
              {icon}
            </div>
          </div>
        )}
        <h5 className="text-lg font-medium">{title}</h5>
      </div>
      <div>
        <div className="text-base-content text-xl font-semibold">{value}</div>
        {(trend || subtitle) && (
          <div className="flex items-center gap-2 text-sm font-semibold">
            {trend && (
              <span className={`${trendColor} inline-flex items-center gap-1`}>
                {trendIcon && <span className={`${trendIcon} size-4`} />}
                {trend.value}%
              </span>
            )}
            {subtitle && (
              <span className="text-base-content/50 font-medium">{subtitle}</span>
            )}
          </div>
        )}
      </div>
    </div>
  )
}
