import { useParams, Link, Navigate } from 'react-router-dom';
import {
  getServiceById,
  getNextService,
  getPrevService,
  CATEGORY_LABELS,
  STATUS_LABELS,
  STATUS_COLORS,
} from '@/data/services-detailed';
import { AnimatedCard } from '@/components/AnimatedCard';
import { FeatureList } from '@/components/FeatureList';
import styles from './ServiceDetail.module.css';

export function ServiceDetail() {
  const { serviceId } = useParams<{ serviceId: string }>();
  const service = serviceId ? getServiceById(serviceId) : undefined;

  if (!service) {
    return <Navigate to="/services" replace />;
  }

  const prevService = getPrevService(service.id);
  const nextService = getNextService(service.id);

  return (
    <div className={styles.wrapper}>
      {/* Navigation */}
      <nav className={styles.breadcrumb}>
        <Link to="/services">Services</Link>
        <span className={styles.separator}>›</span>
        <span>{CATEGORY_LABELS[service.category]}</span>
        <span className={styles.separator}>›</span>
        <span>{service.name}</span>
      </nav>

      {/* Header */}
      <header className={styles.header}>
        <div
          className={styles.iconWrapper}
          style={{ '--service-color': service.color } as React.CSSProperties}
        >
          <span className={styles.icon}>{service.icon}</span>
        </div>
        <div className={styles.headerContent}>
          <div className={styles.meta}>
            <span className={styles.category}>{CATEGORY_LABELS[service.category]}</span>
            <span
              className={styles.status}
              style={{ '--status-color': STATUS_COLORS[service.status] } as React.CSSProperties}
            >
              {STATUS_LABELS[service.status]}
            </span>
          </div>
          <h1 className={styles.name}>{service.name}</h1>
          <p className={styles.tagline}>{service.tagline}</p>
        </div>
      </header>

      {/* Description */}
      <section className={styles.section}>
        <p className={styles.longDesc}>{service.longDesc}</p>
      </section>

      {/* Type de service */}
      <AnimatedCard className={styles.typeCard} hoverable={false}>
        <div className={styles.typeLabel}>Type de Service</div>
        <div className={styles.typeValue}>
          {service.type === 1 && 'Service interne COG — accessible uniquement via Miyukini Central'}
          {service.type === 2 && 'Service à surface web — accessible via Central + Portail'}
          {service.type === 3 && 'Service Inter-COG — interactions entre COGs'}
        </div>
      </AnimatedCard>

      {/* Features */}
      {service.features.length > 0 && (
        <section className={styles.section}>
          <h2>Fonctionnalités</h2>
          <FeatureList features={service.features} columns={2} />
        </section>
      )}

      {/* Intégrations */}
      {service.integrations.length > 0 && (
        <section className={styles.section}>
          <h2>Intégrations</h2>
          <div className={styles.integrations}>
            {service.integrations.map((int, i) => (
              <AnimatedCard key={i} className={styles.integrationCard} delay={i * 50} hoverable={false}>
                <div className={styles.integrationName}>{int.serviceName}</div>
                <p className={styles.integrationDesc}>{int.description}</p>
              </AnimatedCard>
            ))}
          </div>
        </section>
      )}

      {/* Use Cases */}
      {service.useCases.length > 0 && (
        <section className={styles.section}>
          <h2>Cas d'usage</h2>
          <div className={styles.useCases}>
            {service.useCases.map((uc, i) => (
              <AnimatedCard key={i} className={styles.useCaseCard} delay={i * 50} hoverable={false}>
                <div className={styles.useCaseHeader}>
                  <span className={styles.useCaseTitle}>{uc.title}</span>
                  <span className={styles.persona}>{uc.persona}</span>
                </div>
                <p className={styles.useCaseDesc}>{uc.description}</p>
              </AnimatedCard>
            ))}
          </div>
        </section>
      )}

      {/* Navigation Service précédent/suivant */}
      <nav className={styles.serviceNav}>
        {prevService ? (
          <Link to={`/services/${prevService.id}`} className={styles.navLink}>
            <span className={styles.navLabel}>Service précédent</span>
            <span className={styles.navName}>
              <span className={styles.navIcon}>{prevService.icon}</span>
              {prevService.name}
            </span>
          </Link>
        ) : (
          <div />
        )}
        {nextService ? (
          <Link to={`/services/${nextService.id}`} className={`${styles.navLink} ${styles.navLinkNext}`}>
            <span className={styles.navLabel}>Service suivant</span>
            <span className={styles.navName}>
              {nextService.name}
              <span className={styles.navIcon}>{nextService.icon}</span>
            </span>
          </Link>
        ) : (
          <div />
        )}
      </nav>
    </div>
  );
}
