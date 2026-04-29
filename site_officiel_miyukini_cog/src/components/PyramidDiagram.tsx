import { useState } from 'react';
import styles from './PyramidDiagram.module.css';

interface PyramidStratum {
  level: string;
  name: string;
  description: string;
  examples?: string[];
  color: string;
}

const STRATA: PyramidStratum[] = [
  {
    level: '9',
    name: 'MiyukiniAdmin',
    description: 'Opérateur Souverain. Console d\'administration système.',
    examples: ['miyukini-admin'],
    color: '#ef4444',
  },
  {
    level: '7',
    name: 'Opérateurs',
    description: 'Entités fonctionnelles gouvernées. Services et interfaces utilisateur.',
    examples: ['Miyukini Central', 'JayKonta', 'JayKoa'],
    color: '#f59e0b',
  },
  {
    level: '6',
    name: 'Outils & Kits d\'Outils',
    description: 'Capacités exécutables sans autorité. Gouvernés par les Cores.',
    examples: ['MiyuAuth', 'MiyuContent', 'MiyuBilling'],
    color: '#eab308',
  },
  {
    level: '5',
    name: 'Interfaces & Adaptation',
    description: 'BondingBrother — médiation entre Opérateurs et Cores.',
    examples: ['bondingbrother'],
    color: '#22c55e',
  },
  {
    level: '4',
    name: 'Cores Système',
    description: 'Les 8 Cores de gouvernance. Décident ou gouvernent, n\'exécutent jamais.',
    examples: ['StrongFather', 'KindMother', 'WorrySentinel', 'TAMR'],
    color: '#2dd4bf',
  },
  {
    level: '3',
    name: 'Invariants & Contrats',
    description: 'Principes architecturaux immuables. Règles fondamentales du système.',
    color: '#3b82f6',
  },
  {
    level: 'K',
    name: 'Kernel',
    description: 'Substrat technique neutre. Id, Logger, Clock, Config, Lifecycle.',
    examples: ['miyukini-kernel'],
    color: '#6366f1',
  },
  {
    level: '0',
    name: 'Hardware & OS',
    description: 'Réalité physique. Hors contrôle Miyukini mais contrainte acceptée.',
    color: '#8b5cf6',
  },
];

interface PyramidDiagramProps {
  onSelectStratum?: (stratum: PyramidStratum | null) => void;
}

export function PyramidDiagram({ onSelectStratum }: PyramidDiagramProps) {
  const [selectedLevel, setSelectedLevel] = useState<string | null>(null);

  const handleClick = (stratum: PyramidStratum) => {
    const newSelected = selectedLevel === stratum.level ? null : stratum.level;
    setSelectedLevel(newSelected);
    onSelectStratum?.(newSelected ? stratum : null);
  };

  return (
    <div className={styles.pyramid}>
      {STRATA.map((stratum, index) => {
        const isSelected = selectedLevel === stratum.level;
        const width = 30 + (STRATA.length - index) * 8; // Pyramide inversée pour le code

        return (
          <button
            key={stratum.level}
            className={`${styles.stratum} ${isSelected ? styles.selected : ''}`}
            style={{
              '--stratum-color': stratum.color,
              '--stratum-width': `${width}%`,
            } as React.CSSProperties}
            onClick={() => handleClick(stratum)}
          >
            <span className={styles.level}>{stratum.level}</span>
            <span className={styles.name}>{stratum.name}</span>
          </button>
        );
      })}

      {selectedLevel && (
        <div className={styles.details}>
          {STRATA.filter((s) => s.level === selectedLevel).map((stratum) => (
            <div key={stratum.level} className={styles.detailContent}>
              <div
                className={styles.detailHeader}
                style={{ '--stratum-color': stratum.color } as React.CSSProperties}
              >
                <span className={styles.detailLevel}>Strate {stratum.level}</span>
                <h3 className={styles.detailName}>{stratum.name}</h3>
              </div>
              <p className={styles.detailDesc}>{stratum.description}</p>
              {stratum.examples && stratum.examples.length > 0 && (
                <div className={styles.examples}>
                  <span className={styles.examplesLabel}>Exemples :</span>
                  <div className={styles.examplesList}>
                    {stratum.examples.map((ex) => (
                      <span key={ex} className={styles.example}>
                        {ex}
                      </span>
                    ))}
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
