'use client'

import { useMemo } from 'react'

export type CalendarViewMode = 'week' | 'day'

export type CalendarSlotLike = {
  id: string
  startAt: string // ISO
  endAt: string // ISO
}

export type CalendarGridProps<TSlot extends CalendarSlotLike> = {
  mode: CalendarViewMode
  /**
   * Date de référence (semaine/jour)
   */
  anchorDate: Date
  timezone?: string // affichage (best-effort)

  /**
   * Les créneaux à afficher (déjà filtrés par le module appelant)
   */
  slots: TSlot[]

  /**
   * Plage horaire visible. Si absent, déduit de la data (min/max) avec fallback 8-20.
   */
  dayStartHour?: number
  dayEndHour?: number

  /**
   * Rendu d'une carte "slot" dans la grille.
   */
  renderSlot: (slot: TSlot) => React.ReactNode
  onSlotClick?: (slot: TSlot) => void
}

function startOfDay(d: Date) {
  const x = new Date(d)
  x.setHours(0, 0, 0, 0)
  return x
}

function addDays(d: Date, days: number) {
  const x = new Date(d)
  x.setDate(x.getDate() + days)
  return x
}

function startOfISOWeek(d: Date) {
  // ISO week starts Monday
  const x = startOfDay(d)
  const day = x.getDay() // 0..6 (Sun..Sat)
  const diff = day === 0 ? -6 : 1 - day
  return addDays(x, diff)
}

function sameDay(a: Date, b: Date) {
  return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate()
}

export function CalendarGrid<TSlot extends CalendarSlotLike>({
  mode,
  anchorDate,
  slots,
  dayStartHour,
  dayEndHour,
  renderSlot,
  onSlotClick,
}: CalendarGridProps<TSlot>) {
  const days = useMemo(() => {
    if (mode === 'day') return [startOfDay(anchorDate)]
    const start = startOfISOWeek(anchorDate)
    return Array.from({ length: 7 }).map((_, i) => addDays(start, i))
  }, [anchorDate, mode])

  const slotsByDay = useMemo(() => {
    const map = new Map<string, TSlot[]>()
    for (const d of days) map.set(d.toDateString(), [])
    for (const s of slots) {
      const start = new Date(s.startAt)
      for (const d of days) {
        if (sameDay(start, d)) {
          map.get(d.toDateString())?.push(s)
          break
        }
      }
    }
    // tri
    for (const [k, list] of map.entries()) {
      list.sort((a, b) => new Date(a.startAt).getTime() - new Date(b.startAt).getTime())
      map.set(k, list)
    }
    return map
  }, [days, slots])

  const derivedHours = useMemo(() => {
    if (dayStartHour != null && dayEndHour != null) return { start: dayStartHour, end: dayEndHour }
    if (slots.length === 0) return { start: 8, end: 20 }
    let min = 23
    let max = 0
    for (const s of slots) {
      const st = new Date(s.startAt)
      const en = new Date(s.endAt)
      min = Math.min(min, st.getHours())
      max = Math.max(max, en.getHours() + (en.getMinutes() > 0 ? 1 : 0))
    }
    return { start: Math.max(0, Math.min(min, 8)), end: Math.min(24, Math.max(max, 20)) }
  }, [dayEndHour, dayStartHour, slots])

  const hours = useMemo(
    () => Array.from({ length: Math.max(0, derivedHours.end - derivedHours.start) }).map((_, i) => derivedHours.start + i),
    [derivedHours.end, derivedHours.start]
  )

  return (
    <div className="rounded-xl border border-base-300 bg-base-100 overflow-hidden">
      {/* Header days */}
      <div className="grid" style={{ gridTemplateColumns: `120px repeat(${days.length}, minmax(140px, 1fr))` }}>
        <div className="p-3 border-b border-base-300 bg-base-200" />
        {days.map((d) => (
          <div key={d.toISOString()} className="p-3 border-b border-base-300 bg-base-200">
            <div className="text-sm font-semibold text-base-content">
              {d.toLocaleDateString('fr-FR', { weekday: 'short', day: '2-digit', month: 'short' })}
            </div>
          </div>
        ))}
      </div>

      {/* Grid */}
      <div className="grid" style={{ gridTemplateColumns: `120px repeat(${days.length}, minmax(140px, 1fr))` }}>
        {hours.map((h) => (
          <div key={h} className="contents">
            <div className="border-b border-base-300 p-3 text-xs text-base-content/60 bg-base-100">
              {String(h).padStart(2, '0')}:00
            </div>
            {days.map((d) => (
              <div key={`${d.toISOString()}-${h}`} className="border-b border-base-300 border-l border-base-300 p-2 min-h-16">
                {/* Slots that start within this hour */}
                <div className="space-y-2">
                  {(slotsByDay.get(d.toDateString()) ?? [])
                    .filter((s) => new Date(s.startAt).getHours() === h)
                    .map((s) => (
                      <button
                        key={s.id}
                        type="button"
                        className="w-full text-left"
                        onClick={() => onSlotClick?.(s)}
                      >
                        {renderSlot(s)}
                      </button>
                    ))}
                </div>
              </div>
            ))}
          </div>
        ))}
      </div>
    </div>
  )
}

