import { useState } from 'react';
import { SERVICES, CATEGORY_LABELS } from '@/data/services';
import { CORES } from '@/data/cores';
import styles from './MockCog.module.css';

type View = 'home' | string;

export function MockCog() {
  const [view, setView] = useState<View>('home');
  const [filter, setFilter] = useState<string>('all');
  const selectedService = SERVICES.find((s) => s.id === view);

  return (
    <div className={styles.wrapper}>
      <div className={styles.banner}>
        <span className={styles.badge}>Démo</span>
        Mock Miyukini Central — basé sur les fonctionnalités réelles du Hub. Catalogue et navigation simulés.
      </div>

      <div className={styles.mockApp}>
        <aside className={styles.sidebar}>
          <div className={styles.sidebarHeader}>
            <span className={styles.logo}>◇ Miyukini Central</span>
          </div>
          <div className={styles.filters}>
            <button
              type="button"
              className={filter === 'all' ? styles.filterActive : ''}
              onClick={() => setFilter('all')}
            >
              Tous
            </button>
            {(Object.keys(CATEGORY_LABELS) as Array<keyof typeof CATEGORY_LABELS>).map((cat) => (
              <button
                key={cat}
                type="button"
                className={filter === cat ? styles.filterActive : ''}
                onClick={() => setFilter(cat)}
              >
                {CATEGORY_LABELS[cat]}
              </button>
            ))}
          </div>
          <nav className={styles.serviceList}>
            <button
              type="button"
              className={`${styles.serviceItem} ${view === 'home' ? styles.serviceItemActive : ''}`}
              onClick={() => setView('home')}
            >
              <span className={styles.serviceName}>Hub</span>
            </button>
            {SERVICES.filter((s) => filter === 'all' || s.category === filter).map((s) => (
              <button
                key={s.id}
                type="button"
                className={`${styles.serviceItem} ${view === s.id ? styles.serviceItemActive : ''}`}
                onClick={() => setView(s.id)}
              >
                <span className={styles.serviceName}>{s.name}</span>
              </button>
            ))}
          </nav>
          <div className={styles.coresStrip}>
            <span className={styles.coresLabel}>Cores (strate 4)</span>
            <div className={styles.coreDots}>
              {CORES.slice(0, 6).map((c) => (
                <span key={c.id} className={styles.coreDot} title={c.name} />
              ))}
            </div>
          </div>
        </aside>

        <main className={styles.main}>
          {view === 'home' ? (
            <div className={styles.homePanel}>
              <h1>Hub</h1>
              <p>Sélectionnez un Service dans la barre latérale pour simuler son ouverture.</p>
              <p className={styles.muted}>
                En production, chaque Service est un Opérateur gouverné ; l'accès passe par BondingBrother et les Cores (StrongFather, Master Butler, WorrySentinel).
              </p>
              <div className={styles.flux}>
                <code>Utilisateur → Central → BondingBrother → Cores → Opérateur</code>
              </div>
            </div>
          ) : selectedService ? (
            <div className={styles.servicePanel}>
              <h1>{selectedService.name}</h1>
              <p className={styles.serviceId}>{selectedService.id}</p>
              <p>{selectedService.shortDesc}</p>
              <div className={styles.placeholder}>
                [ Interface du Service — mock : pas d’exécution réelle ]
              </div>
            </div>
          ) : null}
        </main>
      </div>
    </div>
  );
}
