import { useEffect, useRef, useState } from 'react';
import styles from './AnimatedCard.module.css';

interface AnimatedCardProps {
  children: React.ReactNode;
  className?: string;
  delay?: number;
  onClick?: () => void;
  hoverable?: boolean;
  glowColor?: string;
}

/**
 * Card avec animations d'entrée (fade-in + slide-up) via Intersection Observer.
 * Supporte hover glow et clic.
 */
export function AnimatedCard({
  children,
  className = '',
  delay = 0,
  onClick,
  hoverable = true,
  glowColor,
}: AnimatedCardProps) {
  const ref = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          // Ajouter un délai si spécifié
          setTimeout(() => setIsVisible(true), delay);
          observer.disconnect();
        }
      },
      { threshold: 0.1, rootMargin: '50px' }
    );

    if (ref.current) {
      observer.observe(ref.current);
    }

    return () => observer.disconnect();
  }, [delay]);

  const cardClasses = [
    styles.card,
    isVisible ? styles.visible : '',
    hoverable ? styles.hoverable : '',
    onClick ? styles.clickable : '',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  const style = glowColor
    ? ({ '--glow-color': glowColor } as React.CSSProperties)
    : undefined;

  return (
    <div
      ref={ref}
      className={cardClasses}
      onClick={onClick}
      style={style}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
      onKeyDown={onClick ? (e) => e.key === 'Enter' && onClick() : undefined}
    >
      {children}
    </div>
  );
}
