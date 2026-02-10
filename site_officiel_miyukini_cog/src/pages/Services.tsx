import { SERVICES, CATEGORY_LABELS } from '@/data/services';
import { Card } from '@/components/Card';
import { Link } from 'react-router-dom';
import styles from './Services.module.css';

export function Services() {
  const byCategory = SERVICES.reduce<Record<string, typeof SERVICES>>((acc, s) => {
    const cat = s.category;
    if (!acc[cat]) acc[cat] = [];
    acc[cat].push(s);
    return acc;
  }, {});

  const order: (keyof typeof CATEGORY_LABELS)[] = ['fundamental', 'jay', 'tool', 'game'];

  return (
    <div className={styles.wrapper}>
      <h1>Services & Opérateurs</h1>
      <p className={styles.intro}>
        Le Registre d'Opérateurs expose les Services disponibles. Miyukini Central en est le hub pour l'utilisateur du COG ; le Portail expose les façades publiques pour les utilisateurs externes.
      </p>

      {order.map((cat) => {
        const list = byCategory[cat];
        if (!list?.length) return null;
        return (
          <section key={cat} className={styles.section}>
            <h2>{CATEGORY_LABELS[cat]}</h2>
            <div className={styles.grid}>
              {list.map((s) => (
                <Card key={s.id} className={styles.serviceCard}>
                  <h3>{s.name}</h3>
                  <p>{s.shortDesc}</p>
                  <span className={styles.id}>{s.id}</span>
                </Card>
              ))}
            </div>
          </section>
        );
      })}

      <p className={styles.cta}>
        <Link to="/demo">Voir le mock COG →</Link> pour un aperçu du Hub et du catalogue en action.
      </p>
    </div>
  );
}
