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
          <Link to="/docs">Documentation</Link>
          <Link to="/cores">Les 8 Cores</Link>
          <Link to="/services">Services</Link>
          <Link to="/demo" className={styles.cta}>Démo COG</Link>
        </nav>
      </header>
      <main className={styles.main}>{children}</main>
      <footer className={styles.footer}>
        <p>Miyukini COG — Environnement de gouvernance orchestré par des Cores.</p>
        <p className={styles.footerMuted}>Documentation et site officiel. Ce site est informatif.</p>
      </footer>
    </div>
  );
}
