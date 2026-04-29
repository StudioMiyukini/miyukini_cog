import { useState } from 'react';
import { Link } from 'react-router-dom';
import styles from './Onboarding.module.css';

const STEPS = [
  {
    title: 'Le probleme',
    content:
      'Aujourd\'hui, pour gerer une activite, il faut empiler les abonnements : un agenda (Google), une compta (QuickBooks), un cloud (Dropbox), un site vitrine (Wix), un outil de facturation... Chaque service a ses propres regles, son propre prix, et **vos donnees sont dispersees sur des serveurs que vous ne controlez pas**.',
    icon: '😩',
    cta: 'Et la solution ?',
  },
  {
    title: 'Miyukini : tout au meme endroit',
    content:
      'Miyukini reunit **tous vos services dans un seul Hub desktop**. Calendrier, comptabilite, evenements, vitrine produit, cloud prive, jeux — tout est la. Vous lancez une seule application, et vous avez acces a tout. **Pas de navigateur**, pas de compte cloud, pas d\'abonnement.',
    icon: '🏠',
    cta: 'Et mes donnees ?',
  },
  {
    title: 'Vos donnees restent chez vous',
    content:
      'Contrairement aux SaaS, Miyukini stocke tout **localement, sur votre machine**. Votre base de donnees est chiffree (AES-256). Vos mots de passe sont haches avec Argon2id. **Aucune telemetrie, aucun tracking.** Et surtout : tout fonctionne **meme sans internet**.',
    icon: '🔒',
    cta: 'Quels services ?',
  },
  {
    title: 'Des services concrets',
    content:
      '**JayKoa** — un calendrier qui agrege tout. **JayKonta** — gerez votre budget. **Miyukini Cloud** — votre cloud prive. **MAIA** — une IA locale. Et meme des **jeux** integres (Clicker, Tower Defense).',
    icon: '🎯',
    cta: 'Comment ca marche ?',
  },
  {
    title: 'Une architecture qui vous protege',
    content:
      'Sous le capot, Miyukini fonctionne comme une **nation numerique** : des Cores (les ministeres) definissent les regles, des Toolkits (les outils) executent, et des Services (les guichets) vous servent. Chaque composant a un role strict. Resultat : **stabilite, securite, pas de surprise**.',
    icon: '🏛️',
    cta: 'Je veux voir !',
  },
  {
    title: 'Pret a explorer',
    content:
      'Parcourez le **catalogue des services**, testez la **demo interactive**, ou plongez dans la **documentation**. Miyukini est gratuit pour un usage personnel.',
    icon: '🚀',
    cta: 'Explorer les services',
  },
];

export function Onboarding() {
  const [step, setStep] = useState(0);
  const isLast = step === STEPS.length - 1;
  const current = STEPS[step];

  return (
    <div className={styles.wrapper}>
      {/* Progress bar */}
      <div className={styles.progressBar}>
        <div
          className={styles.progressFill}
          style={{ width: `${((step + 1) / STEPS.length) * 100}%` }}
        />
      </div>

      <div className={styles.progress}>
        {STEPS.map((_, i) => (
          <button
            key={i}
            type="button"
            className={`${styles.dot} ${i <= step ? styles.dotActive : ''}`}
            onClick={() => setStep(i)}
            aria-label={`Etape ${i + 1}`}
          />
        ))}
      </div>

      <div className={styles.card}>
        <div className={styles.stepIcon}>{current.icon}</div>
        <div className={styles.stepCount}>
          {step + 1} / {STEPS.length}
        </div>
        <h1 className={styles.stepTitle}>{current.title}</h1>
        <p
          className={styles.stepContent}
          dangerouslySetInnerHTML={{
            __html: current.content.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>'),
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
            <Link to="/services" className={styles.primaryBtn}>
              {current.cta}
            </Link>
          ) : (
            <button
              type="button"
              className={styles.primaryBtn}
              onClick={() => setStep((s) => s + 1)}
            >
              {current.cta}
            </button>
          )}
        </div>
      </div>

      <p className={styles.skip}>
        <Link to="/services">Passer et voir les services</Link>
        {' · '}
        <Link to="/">Retour a l'accueil</Link>
      </p>
    </div>
  );
}
