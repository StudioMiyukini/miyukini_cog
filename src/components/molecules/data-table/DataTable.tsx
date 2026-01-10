'use client'

import { ReactNode } from 'react'
import { Badge, BadgeVariant } from '@/components/atoms/badge'

export interface TableColumn<T> {
  key: keyof T | string
  header: string
  render?: (item: T) => ReactNode
  className?: string
}

export interface TableAction {
  icon: string
  label: string
  onClick: (item: any) => void
}

export interface DataTableProps<T> {
  data: T[]
  columns: TableColumn<T>[]
  actions?: TableAction[]
  className?: string
  striped?: boolean
  hoverable?: boolean
}

/**
 * DataTable - Tableau de données FlyonUI
 * 
 * @layer molecules
 * @tokens theme.colors.*, theme.spacing.*
 * @dependencies FlyonUI, Badge
 * 
 * @example
 * ```tsx
 * <DataTable 
 *   data={users}
 *   columns={[
 *     { key: 'name', header: 'Nom' },
 *     { key: 'email', header: 'Email' },
 *     { key: 'status', header: 'Statut', render: (item) => <Badge>{item.status}</Badge> }
 *   ]}
 *   actions={[
 *     { icon: 'icon-[tabler--pencil]', label: 'Edit', onClick: (item) => {} }
 *   ]}
 * />
 * ```
 */
export function DataTable<T extends Record<string, any>>({
  data,
  columns,
  actions,
  className = '',
  striped = false,
  hoverable = true,
}: DataTableProps<T>) {
  return (
    <div className={`rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md ${className}`}>
      <div className="overflow-x-auto">
        <table className={`table ${striped ? 'table-striped' : ''} ${hoverable ? 'table-hover' : ''}`}>
          <thead>
            <tr>
              {columns.map((col) => (
                <th key={String(col.key)} className={col.className}>
                  {col.header}
                </th>
              ))}
              {actions && actions.length > 0 && <th>Actions</th>}
            </tr>
          </thead>
          <tbody>
            {data.map((item, rowIndex) => (
              <tr key={rowIndex}>
                {columns.map((col) => (
                  <td key={`${rowIndex}-${String(col.key)}`} className={col.className}>
                    {col.render ? col.render(item) : item[col.key]}
                  </td>
                ))}
                {actions && actions.length > 0 && (
                  <td>
                    {actions.map((action, actionIndex) => (
                      <button
                        key={actionIndex}
                        className="btn btn-circle btn-text btn-sm"
                        aria-label={action.label}
                        onClick={() => action.onClick(item)}
                      >
                        <span className={`${action.icon} size-5`} />
                      </button>
                    ))}
                  </td>
                )}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

// Helper pour le rendu de badges de statut
export function StatusBadge({ 
  status, 
  variant 
}: { 
  status: string
  variant?: BadgeVariant 
}) {
  const autoVariant: BadgeVariant = 
    status.toLowerCase().includes('success') || status.toLowerCase().includes('professional') ? 'success' :
    status.toLowerCase().includes('error') || status.toLowerCase().includes('rejected') ? 'error' :
    status.toLowerCase().includes('warning') || status.toLowerCase().includes('current') ? 'warning' :
    status.toLowerCase().includes('info') || status.toLowerCase().includes('applied') ? 'info' :
    'neutral'

  return <Badge variant={variant || autoVariant} size="xs">{status}</Badge>
}
