import { Link } from 'react-router-dom';
import { AnimatedCard } from '@/components/AnimatedCard';
import styles from './About.module.css';

const OBJECTIVES = [
  {
    title: 'Maîtrise du hardware à l\'UX',
    description: 'Contrôle de toute la chaîne technique, du noyau jusqu\'à l\'interface utilisateur.',
    icon: '🔧',
  },
  {
    title: 'Livraison par couches',
    description: 'Vendre des briques individuelles ou des solutions complètes selon les besoins.',
    icon: '📦',
  },
  {
    title: 'B2B / B2C / B2B2C',
    description: 'Plusieurs modèles de livraison pour différents marchés et cas d\'usage.',
    icon: '🎯',
  },
  {
    title: 'Offline, isolé, low-resource',
    description: 'Fonctionne sur Raspberry Pi, NAS, ou en zones blanches sans internet.',
    icon: '📡',
  },
  {
    title: 'Modulaire et scalable',
    description: 'Architecture qui évolue sans rupture, de l\'usage personnel à l\'entreprise.',
    icon: '📈',
  },
  {
    title: 'Pas de monolithes jetables',
    description: 'Recomposition plutôt qu\'accumulation. Les composants sont durables.',
    icon: '♻️',
  },
];

const PRINCIPLES = [
  {
    quote: 'Dans Miyukini, les utilisateurs n\'installent pas d\'applications. Ils interagissent avec des Opérateurs gouvernés.',
  },
  {
    quote: 'Un Outil fait, mais ne décide jamais.',
  },
  {
    quote: 'La complexité est gérée par la collaboration, pas par l\'accumulation.',
  },
  {
    quote: 'Central = COG, Portail = Web.',
  },
];

export function About() {
  return (
    <div className={styles.wrapper}>
      {/* Hero */}
      <header className={styles.hero}>
        <h1 className={styles.title}>À propos de Miyukini</h1>
        <p className={styles.subtitle}>
          Core-Orchestrated Governance Environment
        </p>
      </header>

      {/* Vision */}
      <section className={styles.section}>
        <h2>Notre vision</h2>
        <AnimatedCard className={styles.visionCard} hoverable={false}>
          <p className={styles.vision}>
            Miyukini est un <strong>écosystème logiciel autonome et gouverné</strong>, capable de remplacer CMS et SaaS tout en offrant <strong>souveraineté technique</strong>, fonctionnement <strong>offline</strong> et contrôle total de la chaîne, du noyau à l'utilisateur.
          </p>
        </AnimatedCard>
      </section>

      {/* COG */}
      <section className={styles.section}>
        <h2>Qu'est-ce que COG ?</h2>
        <div className={styles.cogGrid}>
          <AnimatedCard className={styles.cogCard} delay={0} hoverable={false}>
            <span className={styles.cogLetter}>C</span>
            <div>
              <h3>Core</h3>
              <p>Les Cores sont les unités de gouvernance du système. 8 Cores, chacun avec son domaine exclusif.</p>
            </div>
          </AnimatedCard>
          <AnimatedCard className={styles.cogCard} delay={100} hoverable={false}>
            <span className={styles.cogLetter}>O</span>
            <div>
              <h3>Orchestrated</h3>
              <p>Coordination active entre les composants. Miyukini n'est pas un OS, c'est un orchestre.</p>
            </div>
          </AnimatedCard>
          <AnimatedCard className={styles.cogCard} delay={200} hoverable={false}>
            <span className={styles.cogLetter}>G</span>
            <div>
              <h3>Governance Environment</h3>
              <p>Environnement complet de bout en bout. Pas juste un framework, un écosystème entier.</p>
            </div>
          </AnimatedCard>
        </div>
        <p className={styles.cogTagline}>
          <em>"Miyukini is not an OS. It's the cog that makes digital systems work together."</em>
        </p>
      </section>

      {/* Objectifs */}
      <section className={styles.section}>
        <h2>Objectifs stratégiques</h2>
        <div className={styles.objectives}>
          {OBJECTIVES.map((obj, i) => (
            <AnimatedCard key={i} className={styles.objectiveCard} delay={i * 50} hoverable={false}>
              <span className={styles.objectiveIcon}>{obj.icon}</span>
              <h3>{obj.title}</h3>
              <p>{obj.description}</p>
            </AnimatedCard>
          ))}
        </div>
      </section>

      {/* Principes */}
      <section className={styles.section}>
        <h2>Principes fondateurs</h2>
        <div className={styles.principles}>
          {PRINCIPLES.map((p, i) => (
            <AnimatedCard key={i} className={styles.principleCard} delay={i * 75} hoverable={false}>
              <blockquote>"{p.quote}"</blockquote>
            </AnimatedCard>
          ))}
        </div>
      </section>

      {/* Gouvernance */}
      <section className={styles.section}>
        <h2>Gouvernance de l'écosystème</h2>
        <div className={styles.governance}>
          <AnimatedCard className={styles.govCard} hoverable={false}>
            <h3>Strates 0 → 5</h3>
            <p>Socle non substituable. Uniquement Miyukini. Ces strates garantissent la souveraineté du système.</p>
          </AnimatedCard>
          <AnimatedCard className={styles.govCard} delay={100} hoverable={false}>
            <h3>Strates 6 → 7</h3>
            <p>Extension possible, mais dans le cadre Miyukini. Les tiers codent "à l'intérieur" de Miyukini, pas "au-dessus".</p>
          </AnimatedCard>
        </div>
      </section>

      {/* CTA */}
      <section className={styles.cta}>
        <h2>Prêt à explorer ?</h2>
        <div className={styles.ctaLinks}>
          <Link to="/onboarding" className={styles.primaryBtn}>
            Commencer le parcours
          </Link>
          <Link to="/architecture" className={styles.secondaryBtn}>
            Voir l'architecture
          </Link>
        </div>
      </section>
    </div>
  );
}
