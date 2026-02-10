import { STRATA } from '@/data/pyramid';
import { GLOSSARY } from '@/data/glossary';
import { Card } from '@/components/Card';
import styles from './Docs.module.css';

export function Docs() {
  return (
    <div className={styles.wrapper}>
      <h1>Documentation</h1>
      <p className={styles.intro}>
        Références conceptuelles Miyukini COG : pyramide des strates, glossaire et flux.
      </p>

      <section className={styles.section}>
        <h2>Pyramide des strates</h2>
        <p className={styles.lead}>
          Chaque strate a un rôle précis. Les Cores (strate 4) gouvernent ; les Opérateurs (7) exécutent sous gouvernance.
        </p>
        <div className={styles.pyramid}>
          {STRATA.map((s) => (
            <Card key={s.id} className={styles.stratum}>
              <span className={styles.stratumLevel}>{s.level}</span>
              <h3>{s.name}</h3>
              <p>{s.description}</p>
              {s.examples && s.examples.length > 0 && (
                <p className={styles.examples}>
                  Ex. : {s.examples.slice(0, 3).join(', ')}
                </p>
              )}
            </Card>
          ))}
        </div>
      </section>

      <section className={styles.section}>
        <h2>Flux standard</h2>
        <div className={styles.flow}>
          <code>
            Utilisateur → Service → Opérateur(s) → BondingBrother → Cores → Outils → Exécution
          </code>
        </div>
        <p className={styles.muted}>
          L'utilisateur voit des Services. Le système exécute via des Opérateurs gouvernés.
        </p>
      </section>

      <section className={styles.section}>
        <h2>Glossaire (extrait)</h2>
        <dl className={styles.glossary}>
          {GLOSSARY.map((g) => (
            <div key={g.term} className={styles.glossaryItem}>
              <dt>{g.term}</dt>
              <dd>{g.definition}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section className={styles.section}>
        <h2>Lois d'autonomie (LOI-1 à LOI-8)</h2>
        <ul className={styles.laws}>
          <li><strong>LOI-1</strong> — Aucune dépendance externe critique à l'exécution.</li>
          <li><strong>LOI-2</strong> — Le système accepte l'isolement comme état normal.</li>
          <li><strong>LOI-3</strong> — L'état local est souverain.</li>
          <li><strong>LOI-4</strong> — Pas de temps global requis.</li>
          <li><strong>LOI-5</strong> — Le coût doit être proportionnel au hardware.</li>
          <li><strong>LOI-6</strong> — L'autonomie n'empêche pas la fédération.</li>
          <li><strong>LOI-7</strong> — La strate Cores est immuable — évolution par environnement.</li>
          <li><strong>LOI-8</strong> — Migration = diplomatie entre environnements.</li>
        </ul>
      </section>
    </div>
  );
}
