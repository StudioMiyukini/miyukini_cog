import { Timeline } from '@/components/Timeline';
import { AnimatedCard } from '@/components/AnimatedCard';
import {
  ROADMAP_PHASES,
  PROJECT_STATS,
  STATUS_LABELS,
  STATUS_COLORS,
} from '@/data/roadmap';
import styles from './Roadmap.module.css';

export function Roadmap() {
  // Transformer les données pour le composant Timeline
  const timelinePhases = ROADMAP_PHASES.map((phase) => ({
    id: phase.id,
    name: phase.name,
    description: phase.description,
    status: phase.status,
    items: phase.milestones,
  }));

  return (
    <div className={styles.wrapper}>
      <header className={styles.header}>
        <h1>Roadmap</h1>
        <p className={styles.intro}>
          L'évolution de Miyukini COG, des fondations jusqu'à la fédération Inter-COG.
        </p>
      </header>

      {/* Stats */}
      <section className={styles.statsSection}>
        <h2>Chiffres clés</h2>
        <div className={styles.stats}>
          {PROJECT_STATS.map((stat, i) => (
            <AnimatedCard key={i} className={styles.statCard} delay={i * 50} hoverable={false}>
              <span className={styles.statIcon}>{stat.icon}</span>
              <span className={styles.statValue}>{stat.value}</span>
              <span className={styles.statLabel}>{stat.label}</span>
            </AnimatedCard>
          ))}
        </div>
      </section>

      {/* Légende */}
      <section className={styles.legendSection}>
        <div className={styles.legend}>
          {(Object.keys(STATUS_LABELS) as Array<keyof typeof STATUS_LABELS>).map((status) => (
            <div key={status} className={styles.legendItem}>
              <span
                className={styles.legendDot}
                style={{ background: STATUS_COLORS[status] }}
              />
              <span>{STATUS_LABELS[status]}</span>
            </div>
          ))}
        </div>
      </section>

      {/* Timeline */}
      <section className={styles.timelineSection}>
        <Timeline
          phases={timelinePhases}
          statusLabels={STATUS_LABELS}
          statusColors={STATUS_COLORS}
        />
      </section>

      {/* Note */}
      <section className={styles.note}>
        <AnimatedCard className={styles.noteCard} hoverable={false}>
          <p>
            Cette roadmap est indicative et évolue selon les priorités du projet.
            Les phases "En cours" peuvent voir leurs milestones terminées progressivement.
          </p>
        </AnimatedCard>
      </section>
    </div>
  );
}
