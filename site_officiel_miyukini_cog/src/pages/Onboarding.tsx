import { useState } from 'react';
import { Link } from 'react-router-dom';
import styles from './Onboarding.module.css';

const STEPS = [
  {
    title: 'Qu’est-ce que Miyukini COG ?',
    content: 'Un **environnement de gouvernance** orchestré par des Cores (C = Core, O = Orchestrated, G = Governance). Ce n’est pas un OS : les Cores décident ou gouvernent, ils n’exécutent jamais.',
    cta: 'Suivant',
  },
  {
    title: 'La pyramide des strates',
    content: 'De la strate 0 (Hardware) à la strate 9 (MiyukiniAdmin), en passant par le Kernel, les Cores (strate 4), BondingBrother (5), les Outils (6) et les Opérateurs (7). Chaque couche a un rôle précis.',
    cta: 'Suivant',
  },
  {
    title: 'Les 8 Cores',
    content: 'StrongFather (décision), KindMother (données), Caring Nanny (observation), Master Butler (permissions), Border Guard (frontières), Ever Buddy (cycle de vie), WorrySentinel (sécurité), TAMR (intervention humaine).',
    cta: 'Suivant',
  },
  {
    title: 'Central & Portail',
    content: '**Miyukini Central** = hub pour l’utilisateur du COG. **Miyukini Web Portal** = point d’entrée web pour les utilisateurs externes. Règle : Central = COG, Portail = Web.',
    cta: 'Suivant',
  },
  {
    title: 'Vous êtes prêt',
    content: 'Explorez la documentation, les Cores, les Services, et essayez le mock COG pour voir un aperçu du Hub en action.',
    cta: 'Explorer',
  },
];

export function Onboarding() {
  const [step, setStep] = useState(0);
  const isLast = step === STEPS.length - 1;

  return (
    <div className={styles.wrapper}>
      <div className={styles.progress}>
        {STEPS.map((_, i) => (
          <div
            key={i}
            className={`${styles.dot} ${i <= step ? styles.dotActive : ''}`}
            aria-hidden
          />
        ))}
      </div>
      <div className={styles.card}>
        <h1 className={styles.stepTitle}>{STEPS[step].title}</h1>
        <p
          className={styles.stepContent}
          dangerouslySetInnerHTML={{
            __html: STEPS[step].content.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>'),
          }}
        />
        <div className={styles.actions}>
          {step > 0 && (
            <button
              type="button"
              className={styles.backBtn}
              onClick={() => setStep((s) => s - 1)}
            >
              Retour
            </button>
          )}
          {isLast ? (
            <Link to="/docs" className={styles.primaryBtn}>
              {STEPS[step].cta}
            </Link>
          ) : (
            <button
              type="button"
              className={styles.primaryBtn}
              onClick={() => setStep((s) => s + 1)}
            >
              {STEPS[step].cta}
            </button>
          )}
        </div>
      </div>
      <p className={styles.skip}>
        <Link to="/">Retour à l’accueil</Link>
      </p>
    </div>
  );
}
