import styles from './FeatureList.module.css';

export interface Feature {
  title: string;
  description: string;
  icon: string;
}

interface FeatureListProps {
  features: Feature[];
  columns?: 1 | 2 | 3;
  compact?: boolean;
}

/**
 * Liste de features avec icônes et descriptions.
 * Affichage en grille configurable.
 */
/*
 * @id     cog.compFeatureList
 * @do     lister_fonctionnalites
 * @role   ui
 * @layer  ui
 * @human  Liste de fonctionnalités : grille de points-clés, en colonnes.
 */
export function FeatureList({ features, columns = 2, compact = false }: FeatureListProps) {
  return (
    <ul
      className={`${styles.list} ${compact ? styles.compact : ''}`}
      style={{ '--columns': columns } as React.CSSProperties}
    >
      {features.map((feature, index) => (
        <li key={index} className={styles.item}>
          <span className={styles.icon}>{feature.icon}</span>
          <div className={styles.content}>
            <h4 className={styles.title}>{feature.title}</h4>
            <p className={styles.description}>{feature.description}</p>
          </div>
        </li>
      ))}
    </ul>
  );
}
