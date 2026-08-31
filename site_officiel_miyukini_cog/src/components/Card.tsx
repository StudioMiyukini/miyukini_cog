import styles from './Card.module.css';

interface CardProps {
  children: React.ReactNode;
  className?: string;
}

/*
 * @id     cog.compCard
 * @do     afficher_carte
 * @role   ui
 * @layer  ui
 * @human  Carte : conteneur encadré réutilisable du site.
 */
export function Card({ children, className = '' }: CardProps) {
  return <div className={`${styles.card} ${className}`}>{children}</div>;
}
