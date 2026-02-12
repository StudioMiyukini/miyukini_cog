import { Link } from 'react-router-dom';
import { PyramidDiagram } from '@/components/PyramidDiagram';
import { AnimatedCard } from '@/components/AnimatedCard';
import styles from './Architecture.module.css';

const LAWS = [
  {
    id: 'LOI-1',
    title: 'Aucune dépendance externe critique',
    description: 'Le système ne doit dépendre d\'aucun service externe pour fonctionner. Pas de cloud obligatoire, pas d\'API tierce critique.',
  },
  {
    id: 'LOI-2',
    title: 'L\'isolement est un état normal',
    description: 'Le système accepte l\'isolement réseau comme un état de fonctionnement normal, pas comme une erreur.',
  },
  {
    id: 'LOI-3',
    title: 'L\'état local est souverain',
    description: 'La vérité locale prime. En cas de conflit, c\'est l\'état local qui fait autorité jusqu\'à réconciliation explicite.',
  },
  {
    id: 'LOI-4',
    title: 'Pas de temps global requis',
    description: 'Le système fonctionne avec le temps local. Pas de NTP obligatoire, horloges logiques pour la causalité.',
  },
  {
    id: 'LOI-5',
    title: 'Coût proportionnel au hardware',
    description: 'Un Raspberry Pi 4 avec 4 Go de RAM doit pouvoir faire tourner le système. Pas de ressources démesurées.',
  },
  {
    id: 'LOI-6',
    title: 'L\'autonomie n\'empêche pas la fédération',
    description: 'La fédération entre COGs est possible, mais elle est explicite, contrôlée et réversible.',
  },
  {
    id: 'LOI-7',
    title: 'La strate Cores est immuable',
    description: 'Les Cores ne changent pas. L\'évolution se fait par création d\'un nouvel environnement complet.',
  },
  {
    id: 'LOI-8',
    title: 'Migration = diplomatie',
    description: 'Toute migration entre environnements est un processus formel de diplomatie, jamais une copie brute automatique.',
  },
];

export function Architecture() {
  return (
    <div className={styles.wrapper}>
      <header className={styles.header}>
        <h1>Architecture Miyukini</h1>
        <p className={styles.intro}>
          La pyramide des strates et les lois d'autonomie définissent la structure fondamentale de l'écosystème Miyukini COG.
        </p>
      </header>

      {/* Pyramide */}
      <section className={styles.section}>
        <h2>Pyramide des strates</h2>
        <p className={styles.sectionIntro}>
          Chaque strate a un rôle précis. Les dépendances vont uniquement vers le bas : une strate ne dépend que des strates inférieures. Cliquez sur une strate pour voir les détails.
        </p>
        <PyramidDiagram />
      </section>

      {/* Flux d'exécution */}
      <section className={styles.section}>
        <h2>Flux d'exécution standard</h2>
        <AnimatedCard className={styles.flowCard} hoverable={false}>
          <div className={styles.flow}>
            <span className={styles.flowStep}>Utilisateur</span>
            <span className={styles.flowArrow}>→</span>
            <span className={styles.flowStep}>Service</span>
            <span className={styles.flowArrow}>→</span>
            <span className={styles.flowStep}>Opérateur</span>
            <span className={styles.flowArrow}>→</span>
            <span className={styles.flowStep}>BondingBrother</span>
            <span className={styles.flowArrow}>→</span>
            <span className={styles.flowStep}>Cores</span>
            <span className={styles.flowArrow}>→</span>
            <span className={styles.flowStep}>Outils</span>
          </div>
          <p className={styles.flowDesc}>
            L'utilisateur voit des Services. Le système exécute via des Opérateurs gouvernés. Les Cores décident, les Outils font.
          </p>
        </AnimatedCard>
      </section>

      {/* Règle fondamentale */}
      <section className={styles.section}>
        <h2>Règle fondamentale</h2>
        <AnimatedCard className={styles.ruleCard} hoverable={false}>
          <blockquote className={styles.quote}>
            "Les Cores décident ou gouvernent, mais n'exécutent jamais."
          </blockquote>
          <p className={styles.ruleExplain}>
            Cette séparation stricte garantit que la gouvernance reste découplée de l'exécution. Un Core ne peut pas "faire" — il peut seulement "autoriser", "interdire" ou "observer".
          </p>
        </AnimatedCard>
      </section>

      {/* Lois d'autonomie */}
      <section className={styles.section}>
        <h2>Les 8 Lois d'autonomie</h2>
        <p className={styles.sectionIntro}>
          Ces lois garantissent que Miyukini reste autonome, souverain et capable de fonctionner même en isolation complète.
        </p>
        <div className={styles.laws}>
          {LAWS.map((law, index) => (
            <AnimatedCard key={law.id} className={styles.lawCard} delay={index * 50} hoverable={false}>
              <div className={styles.lawHeader}>
                <span className={styles.lawId}>{law.id}</span>
                <h3 className={styles.lawTitle}>{law.title}</h3>
              </div>
              <p className={styles.lawDesc}>{law.description}</p>
            </AnimatedCard>
          ))}
        </div>
      </section>

      {/* Question de validation */}
      <section className={styles.section}>
        <AnimatedCard className={styles.validationCard} hoverable={false}>
          <h3 className={styles.validationTitle}>Question de validation architecturale</h3>
          <p className={styles.validationQuestion}>
            « Est-ce que ça fonctionne encore si le système est seul, lent et isolé ? »
          </p>
          <p className={styles.validationDesc}>
            Si la réponse est non, le design viole les lois d'autonomie.
          </p>
        </AnimatedCard>
      </section>

      {/* Liens */}
      <section className={styles.links}>
        <Link to="/cores" className={styles.link}>
          Découvrir les 8 Cores →
        </Link>
        <Link to="/docs" className={styles.link}>
          Documentation complète →
        </Link>
      </section>
    </div>
  );
}
