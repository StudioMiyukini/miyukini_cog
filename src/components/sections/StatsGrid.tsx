'use client'

/**
 * StatsGrid - Grille de statistiques
 * 
 * @layer sections
 * @responsive Auto-fit grid
 */

const defaultStats = [
  { label: 'Utilisateurs', value: '2 483' },
  { label: 'Sessions', value: '197' },
  { label: 'Rôles actifs', value: '5' }
]

export interface StatItem {
  label: string
  value: string | number
}

export interface StatsGridProps {
  stats?: StatItem[]
}

export function StatsGrid({ stats = defaultStats }: StatsGridProps) {
  return (
    <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
      {stats.map((stat) => (
        <article 
          key={stat.label} 
          className="card bg-base-200 border border-base-300"
        >
          <div className="card-body p-4">
            <p className="text-sm text-base-content/60">{stat.label}</p>
            <p className="text-2xl font-bold text-base-content">{stat.value}</p>
          </div>
        </article>
      ))}
    </div>
  )
}
