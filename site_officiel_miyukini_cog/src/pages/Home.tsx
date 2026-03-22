import { Link } from 'react-router-dom';
import styles from './Home.module.css';

export function Home() {
  return (
    <div className={styles.hero}>
      <p className={styles.eyebrow}>Ecosysteme numerique souverain</p>
      <h1 className={styles.title}>
        <span className={styles.titleAccent}>Miyukini COG</span>
        <br />
        Vos services. Vos donnees. Chez vous.
      </h1>
      <p className={styles.subtitle}>
        Un seul Hub pour gerer votre activite, vos evenements, vos finances,
        votre cloud et vos jeux — sans abonnement, sans cloud tiers,
        sans dependance externe. Tout fonctionne hors-ligne.
      </p>
      <div className={styles.actions}>
        <Link to="/onboarding" className={styles.primaryBtn}>
          Decouvrir en 2 minutes
        </Link>
        <Link to="/services" className={styles.secondaryBtn}>
          Voir les services
        </Link>
      </div>

      {/* Who is it for */}
      <section className={styles.audience}>
        <h2>A qui s'adresse Miyukini ?</h2>
        <div className={styles.grid}>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>🎨</span>
            <h3>Artisans & Createurs</h3>
            <p>Vitrine produit, catalogue, gestion de festivals et marches.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>🎪</span>
            <h3>Organisateurs d'evenements</h3>
            <p>Editions, exposants, billetterie, programme, budget — tout en un.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>💼</span>
            <h3>Independants & TPE</h3>
            <p>Comptabilite, devis, factures, tresorerie. Sans abonnement mensuel.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>🏠</span>
            <h3>Particuliers</h3>
            <p>Budget personnel, cloud prive, calendrier partage, jeux integres.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>🍔</span>
            <h3>Restaurateurs</h3>
            <p>Commande en ligne, reservation de tables, gestion de stock.</p>
          </div>
          <div className={styles.feature}>
            <span className={styles.featureIcon}>🔧</span>
            <h3>Developpeurs</h3>
            <p>Architecture gouvernee en Rust, 70+ crates, moteur de jeu, IA locale.</p>
          </div>
        </div>
      </section>

      {/* Comparison table */}
      <section className={styles.comparison}>
        <h2>Miyukini vs. SaaS classique</h2>
        <div className={styles.tableWrapper}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th></th>
                <th className={styles.thHighlight}>Miyukini</th>
                <th>SaaS classique</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td className={styles.rowLabel}>Vos donnees</td>
                <td className={styles.good}>Locales, chiffrees AES-256</td>
                <td className={styles.neutral}>Serveurs tiers</td>
              </tr>
              <tr>
                <td className={styles.rowLabel}>Internet requis</td>
                <td className={styles.good}>Non — tout fonctionne hors-ligne</td>
                <td className={styles.neutral}>Obligatoire</td>
              </tr>
              <tr>
                <td className={styles.rowLabel}>Cout mensuel</td>
                <td className={styles.good}>Gratuit (usage perso)</td>
                <td className={styles.neutral}>5 a 50 euros/mois/service</td>
              </tr>
              <tr>
                <td className={styles.rowLabel}>Vie privee</td>
                <td className={styles.good}>Zero telemetrie</td>
                <td className={styles.neutral}>Profilage publicitaire</td>
              </tr>
              <tr>
                <td className={styles.rowLabel}>Performance</td>
                <td className={styles.good}>App native Rust — rapide</td>
                <td className={styles.neutral}>App web — lente</td>
              </tr>
              <tr>
                <td className={styles.rowLabel}>Mises a jour</td>
                <td className={styles.good}>Vous decidez</td>
                <td className={styles.neutral}>Imposees</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      {/* Key numbers */}
      <section className={styles.stats}>
        <h2>L'ecosysteme en chiffres</h2>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <span className={styles.statNumber}>11</span>
            <span className={styles.statLabel}>Services & applications</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statNumber}>49</span>
            <span className={styles.statLabel}>Toolkits metier</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statNumber}>9</span>
            <span className={styles.statLabel}>Cores de gouvernance</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statNumber}>70+</span>
            <span className={styles.statLabel}>Crates Rust</span>
          </div>
        </div>
      </section>

      {/* Security highlight */}
      <section className={styles.security}>
        <h2>Securite integree</h2>
        <div className={styles.securityGrid}>
          <div className={styles.securityItem}>
            <span className={styles.securityIcon}>🔒</span>
            <div>
              <strong>Chiffrement AES-256</strong>
              <p>Base de donnees chiffree au repos</p>
            </div>
          </div>
          <div className={styles.securityItem}>
            <span className={styles.securityIcon}>🛡️</span>
            <div>
              <strong>Argon2id</strong>
              <p>Hachage de mots de passe resistant aux GPU</p>
            </div>
          </div>
          <div className={styles.securityItem}>
            <span className={styles.securityIcon}>🔐</span>
            <div>
              <strong>TLS 1.2+</strong>
              <p>Connexions reseau securisees</p>
            </div>
          </div>
          <div className={styles.securityItem}>
            <span className={styles.securityIcon}>📋</span>
            <div>
              <strong>Audit complet</strong>
              <p>Toute ecriture est tracee (WriteIntent)</p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className={styles.ctaSection}>
        <h2>Pret a explorer ?</h2>
        <p className={styles.ctaText}>
          Decouvrez le parcours interactif ou plongez directement dans les services.
        </p>
        <div className={styles.actions}>
          <Link to="/onboarding" className={styles.primaryBtn}>
            Parcours guide
          </Link>
          <Link to="/demo" className={styles.secondaryBtn}>
            Demo interactive
          </Link>
          <Link to="/docs" className={styles.secondaryBtn}>
            Documentation
          </Link>
        </div>
      </section>
    </div>
  );
}
