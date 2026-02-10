import { CORES, TRUST_LEVELS, SECURITY_LEVELS } from '@/data/cores';
import { Card } from '@/components/Card';
import styles from './Cores.module.css';

export function Cores() {
  return (
    <div className={styles.wrapper}>
      <h1>Les 8 Cores</h1>
      <p className={styles.intro}>
        Les Cores (Strate 4) décident ou gouvernent ; ils n'exécutent jamais. Chaque Core a une autorité exclusive dans son domaine.
      </p>

      <div className={styles.grid}>
        {CORES.map((core) => (
          <Card key={core.id} className={styles.coreCard}>
            <h2 className={styles.coreName}>{core.name}</h2>
            <p className={styles.question}>« {core.question} »</p>
            <p className={styles.domain}>{core.domain}</p>
            <p className={styles.desc}>{core.shortDesc}</p>
            <div className={styles.badges}>
              {core.decide && <span className={styles.badge}>Décide</span>}
              {core.govern && <span className={styles.badge}>Gouverne</span>}
              {core.observe && <span className={styles.badge}>Observe</span>}
            </div>
          </Card>
        ))}
      </div>

      <section className={styles.section}>
        <h2>États de confiance (WorrySentinel)</h2>
        <div className={styles.tableWrap}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th>État</th>
                <th>Nom</th>
                <th>Impact</th>
              </tr>
            </thead>
            <tbody>
              {TRUST_LEVELS.map((t) => (
                <tr key={t.id}>
                  <td><code>{t.id}</code></td>
                  <td>{t.name}</td>
                  <td>{t.impact}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      <section className={styles.section}>
        <h2>Niveaux de sécurité</h2>
        <div className={styles.levels}>
          {SECURITY_LEVELS.map((s) => (
            <span key={s.level} className={styles.levelChip}>
              {s.level} — {s.name}
            </span>
          ))}
        </div>
      </section>
    </div>
  );
}
