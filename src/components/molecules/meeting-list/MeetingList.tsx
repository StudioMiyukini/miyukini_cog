'use client'

import { Avatar } from '@/components/atoms/avatar'
import { Badge, BadgeVariant } from '@/components/atoms/badge'

export interface Meeting {
  id: string
  title: string
  date: string
  time: string
  avatar?: string
  category?: {
    label: string
    variant: BadgeVariant
  }
}

export interface MeetingListProps {
  meetings: Meeting[]
  className?: string
  onMeetingClick?: (meeting: Meeting) => void
}

/**
 * MeetingList - Liste de réunions FlyonUI
 * 
 * @layer molecules
 * @tokens theme.colors.*, theme.spacing.*
 * @dependencies FlyonUI, Avatar, Badge
 * 
 * @example
 * ```tsx
 * <MeetingList 
 *   meetings={[
 *     { id: '1', title: 'Call with John', date: '1 Jul', time: '08:20-10:20', 
 *       category: { label: 'Business', variant: 'primary' } }
 *   ]}
 * />
 * ```
 */
export function MeetingList({ meetings, className = '', onMeetingClick }: MeetingListProps) {
  return (
    <ul className={`flex h-full flex-col justify-between gap-6 ${className}`}>
      {meetings.map((meeting) => (
        <li key={meeting.id}>
          <div 
            className={`flex items-center gap-3 ${onMeetingClick ? 'cursor-pointer hover:bg-base-200 rounded-box p-2 -m-2 transition-colors' : ''}`}
            onClick={() => onMeetingClick?.(meeting)}
          >
            <Avatar 
              src={meeting.avatar} 
              size="md" 
              rounded="field"
            />
            <div className="grow">
              <h6 className="text-base-content mb-px font-medium">{meeting.title}</h6>
              <div className="text-base-content/50 flex items-center gap-1 text-sm">
                <span className="icon-[tabler--calendar] size-4.5" />
                <span>{meeting.date} | {meeting.time}</span>
              </div>
            </div>
            {meeting.category && (
              <Badge 
                variant={meeting.category.variant} 
                style="soft" 
                rounded
              >
                {meeting.category.label}
              </Badge>
            )}
          </div>
        </li>
      ))}
    </ul>
  )
}
