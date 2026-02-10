import { Link } from 'react-router-dom';
import styles from './Home.module.css';

export function Home() {
  return (
    <div className={styles.hero}>
      <h1 className={styles.title}>
        <span className={styles.titleAccent}>Miyukini COG</span>
        <br />
        Environnement de gouvernance orchestré par des Cores
      </h1>
      <p className={styles.subtitle}>
        Découvrez l'écosystème Miyukini : architecture pyramidale, 8 Cores de gouvernance,
        Services et Opérateurs. Documentation, avantages et démo interactive.
      </p>
      <div className={styles.actions}>
        <Link to="/onboarding" className={styles.primaryBtn}>
          Commencer le parcours
        </Link>
        <Link to="/demo" className={styles.secondaryBtn}>
          Voir la démo COG
        </Link>
      </div>

      <section className={styles.features}>
        <h2>Pourquoi Miyukini COG ?</h2>
        <div className={styles.grid}>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>◇</span>
            <h3>Gouvernance claire</h3>
            <p>Les Cores décident ou gouvernent, n'exécutent jamais. Séparation nette des responsabilités.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>◇</span>
            <h3>Autonomie & souveraineté</h3>
            <p>Lois d'autonomie LOI-1 à LOI-8 : pas de dépendance externe critique, état local souverain.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>◇</span>
            <h3>Services orchestrés</h3>
            <p>Miyukini Central comme hub unique. Registre d'Opérateurs, Mandats de Permission.</p>
          </div>
        </div>
      </section>

      <section className={styles.quickLinks}>
        <h2>Explorer</h2>
        <div className={styles.linkGrid}>
          <Link to="/docs" className={styles.quickLink}>Documentation</Link>
          <Link to="/cores" className={styles.quickLink}>Les 8 Cores</Link>
          <Link to="/services" className={styles.quickLink}>Services & Opérateurs</Link>
          <Link to="/demo" className={styles.quickLink}>Mock COG (démo)</Link>
        </div>
      </section>
    </div>
  );
}
