import { useParams, Link, Navigate } from 'react-router-dom';
import { getCoreById, getNextCore, getPrevCore } from '@/data/cores';
import { AnimatedCard } from '@/components/AnimatedCard';
import styles from './CoreDetail.module.css';

export function CoreDetail() {
  const { coreId } = useParams<{ coreId: string }>();
  const core = coreId ? getCoreById(coreId) : undefined;

  if (!core) {
    return <Navigate to="/cores" replace />;
  }

  const prevCore = getPrevCore(core.id);
  const nextCore = getNextCore(core.id);

  return (
    <div className={styles.wrapper}>
      {/* Navigation */}
      <nav className={styles.breadcrumb}>
        <Link to="/cores">Les 8 Cores</Link>
        <span className={styles.separator}>›</span>
        <span>{core.name}</span>
      </nav>

      {/* Header */}
      <header className={styles.header}>
        <span className={styles.icon}>{core.icon}</span>
        <div>
          <h1 className={styles.name}>{core.name}</h1>
          <p className={styles.domain}>{core.domain}</p>
        </div>
      </header>

      {/* Question fondamentale */}
      <AnimatedCard className={styles.questionCard} hoverable={false}>
        <p className={styles.questionLabel}>Question fondamentale</p>
        <p className={styles.question}>« {core.question} »</p>
      </AnimatedCard>

      {/* Badges */}
      <div className={styles.badges}>
        {core.decide && <span className={styles.badge}>Décide</span>}
        {core.govern && <span className={styles.badge}>Gouverne</span>}
        {core.observe && <span className={styles.badge}>Observe</span>}
      </div>

      {/* Description longue */}
      <section className={styles.section}>
        <h2>Description</h2>
        <p className={styles.longDesc}>{core.longDesc}</p>
      </section>

      {/* Responsabilités */}
      <section className={styles.section}>
        <h2>Responsabilités</h2>
        <ul className={styles.list}>
          {core.responsibilities.map((r, i) => (
            <li key={i} className={styles.listItem}>
              <span className={styles.bullet}>◇</span>
              {r}
            </li>
          ))}
        </ul>
      </section>

      {/* Invariants */}
      <section className={styles.section}>
        <h2>Invariants</h2>
        <div className={styles.invariants}>
          {core.invariants.map((inv, i) => (
            <AnimatedCard key={i} className={styles.invariantCard} delay={i * 50} hoverable={false}>
              <span className={styles.invariantNum}>INV-{i + 1}</span>
              <p>{inv}</p>
            </AnimatedCard>
          ))}
        </div>
      </section>

      {/* Interactions */}
      <section className={styles.section}>
        <h2>Interactions avec les autres Cores</h2>
        <ul className={styles.interactions}>
          {core.interactions.map((int, i) => (
            <li key={i} className={styles.interactionItem}>
              <span className={styles.arrow}>→</span>
              {int}
            </li>
          ))}
        </ul>
      </section>

      {/* Navigation Core précédent/suivant */}
      <nav className={styles.coreNav}>
        {prevCore ? (
          <Link to={`/cores/${prevCore.id}`} className={styles.navLink}>
            <span className={styles.navLabel}>Core précédent</span>
            <span className={styles.navName}>
              <span className={styles.navIcon}>{prevCore.icon}</span>
              {prevCore.name}
            </span>
          </Link>
        ) : (
          <div />
        )}
        {nextCore ? (
          <Link to={`/cores/${nextCore.id}`} className={`${styles.navLink} ${styles.navLinkNext}`}>
            <span className={styles.navLabel}>Core suivant</span>
            <span className={styles.navName}>
              {nextCore.name}
              <span className={styles.navIcon}>{nextCore.icon}</span>
            </span>
          </Link>
        ) : (
          <div />
        )}
      </nav>
    </div>
  );
}
