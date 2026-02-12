import { useState } from 'react';
import styles from './Timeline.module.css';

export type TimelineStatus = 'completed' | 'in-progress' | 'planned';

export interface TimelineItem {
  id: string;
  title: string;
  description: string;
  status: TimelineStatus;
}

export interface TimelinePhase {
  id: string;
  name: string;
  description: string;
  status: TimelineStatus;
  items: TimelineItem[];
}

interface TimelineProps {
  phases: TimelinePhase[];
  statusLabels: Record<TimelineStatus, string>;
  statusColors: Record<TimelineStatus, string>;
}

export function Timeline({ phases, statusLabels, statusColors }: TimelineProps) {
  const [expandedPhase, setExpandedPhase] = useState<string | null>(
    phases.find((p) => p.status === 'in-progress')?.id || null
  );

  const togglePhase = (phaseId: string) => {
    setExpandedPhase(expandedPhase === phaseId ? null : phaseId);
  };

  return (
    <div className={styles.timeline}>
      {phases.map((phase, index) => {
        const isExpanded = expandedPhase === phase.id;
        const color = statusColors[phase.status];

        return (
          <div
            key={phase.id}
            className={`${styles.phase} ${isExpanded ? styles.expanded : ''}`}
            style={{ '--phase-color': color } as React.CSSProperties}
          >
            {/* Timeline line */}
            <div className={styles.line}>
              <div className={styles.dot} />
              {index < phases.length - 1 && <div className={styles.connector} />}
            </div>

            {/* Phase content */}
            <div className={styles.content}>
              <button
                className={styles.phaseHeader}
                onClick={() => togglePhase(phase.id)}
                aria-expanded={isExpanded}
              >
                <div className={styles.phaseInfo}>
                  <h3 className={styles.phaseName}>{phase.name}</h3>
                  <p className={styles.phaseDesc}>{phase.description}</p>
                </div>
                <span className={styles.status}>{statusLabels[phase.status]}</span>
                <span className={`${styles.chevron} ${isExpanded ? styles.chevronUp : ''}`}>
                  ▼
                </span>
              </button>

              {isExpanded && (
                <div className={styles.items}>
                  {phase.items.map((item) => (
                    <div
                      key={item.id}
                      className={styles.item}
                      style={{ '--item-color': statusColors[item.status] } as React.CSSProperties}
                    >
                      <span className={styles.itemDot} />
                      <div className={styles.itemContent}>
                        <div className={styles.itemHeader}>
                          <span className={styles.itemTitle}>{item.title}</span>
                          <span className={styles.itemStatus}>{statusLabels[item.status]}</span>
                        </div>
                        <p className={styles.itemDesc}>{item.description}</p>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
