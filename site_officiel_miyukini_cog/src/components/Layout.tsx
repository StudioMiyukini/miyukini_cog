import { Link } from 'react-router-dom';
import styles from './Layout.module.css';

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps) {
  return (
    <div className={styles.wrapper}>
      <header className={styles.header}>
        <Link to="/" className={styles.logo}>
          <span className={styles.logoIcon}>◇</span>
          <span>Miyukini COG</span>
        </Link>
        <nav className={styles.nav}>
          <Link to="/">Accueil</Link>
          <Link to="/onboarding">Decouvrir</Link>
          <Link to="/services">Services</Link>
          <Link to="/cores">Architecture</Link>
          <Link to="/docs">Docs</Link>
          <Link to="/demo" className={styles.cta}>Demo</Link>
        </nav>
      </header>
      <main className={styles.main}>{children}</main>
      <footer className={styles.footer}>
        <p>Miyukini COG — Votre ecosysteme numerique souverain.</p>
        <p className={styles.footerMuted}>
          Gratuit pour usage personnel. Licence commerciale disponible.
        </p>
      </footer>
    </div>
  );
}
