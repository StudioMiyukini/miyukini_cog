import { useState, useMemo } from 'react';
import { Link } from 'react-router-dom';
import {
  SERVICES,
  CATEGORY_LABELS,
  CATEGORY_COLORS,
  CATEGORY_ICONS,
  STATUS_LABELS,
  STATUS_COLORS,
  MIOU_QUOTES,
  MIOU_DEFAULT,
  type ServiceMeta,
} from '@/data/services';
import { getServiceById } from '@/data/services-detailed';
import { MiouMascot } from '@/components/MiouMascot';
import styles from './Services.module.css';

type CategoryFilter = 'all' | ServiceMeta['category'];

export function Services() {
  const [activeCategory, setActiveCategory] = useState<CategoryFilter>('all');
  const [selectedService, setSelectedService] = useState<ServiceMeta | null>(null);
  const [miouText, setMiouText] = useState(MIOU_DEFAULT);

  const filteredServices = useMemo(() => {
    if (activeCategory === 'all') return SERVICES;
    return SERVICES.filter((s) => s.category === activeCategory);
  }, [activeCategory]);

  const categories: { key: CategoryFilter; label: string; icon?: string }[] = [
    { key: 'all', label: 'Tous', icon: '◈' },
    ...(['fundamental', 'jay', 'tool', 'game'] as const).map((cat) => ({
      key: cat as CategoryFilter,
      label: CATEGORY_LABELS[cat],
      icon: CATEGORY_ICONS[cat],
    })),
  ];

  const handleCardHover = (service: ServiceMeta) => {
    setMiouText(MIOU_QUOTES[service.id] || MIOU_DEFAULT);
  };

  const handleCardLeave = () => {
    if (!selectedService) {
      setMiouText(MIOU_DEFAULT);
    } else {
      setMiouText(MIOU_QUOTES[selectedService.id] || MIOU_DEFAULT);
    }
  };

  const handleCardClick = (service: ServiceMeta) => {
    setSelectedService(service);
    setMiouText(MIOU_QUOTES[service.id] || MIOU_DEFAULT);
  };

  const detailedInfo = selectedService ? getServiceById(selectedService.id) : undefined;

  return (
    <div className={styles.inventoryPage}>
      {/* Category filter tabs */}
      <div className={styles.categoryBar}>
        {categories.map((cat) => {
          const count = cat.key === 'all'
            ? SERVICES.length
            : SERVICES.filter((s) => s.category === cat.key).length;

          return (
            <button
              key={cat.key}
              className={`${styles.categoryTab} ${activeCategory === cat.key ? styles.categoryTabActive : ''}`}
              onClick={() => setActiveCategory(cat.key)}
            >
              {cat.icon && <span className={styles.categoryTabIcon}>{cat.icon}</span>}
              {cat.label}
              <span className={styles.tabCount}>{count}</span>
            </button>
          );
        })}
      </div>

      {/* Main 3-panel layout */}
      <div className={styles.mainContent}>
        {/* LEFT: Inventory grid */}
        <div className={styles.inventoryPanel}>
          <div className={styles.inventoryGrid}>
            {filteredServices.map((service) => {
              const catColors = CATEGORY_COLORS[service.category];
              const isSelected = selectedService?.id === service.id;
              const statusColor = STATUS_COLORS[service.status];

              return (
                <div
                  key={service.id}
                  className={`${styles.itemCard} ${isSelected ? styles.itemCardSelected : ''}`}
                  style={{
                    background: `linear-gradient(145deg, ${catColors.bg} 0%, rgba(15,18,25,0.9) 100%)`,
                    borderColor: isSelected ? catColors.border : `${catColors.border}40`,
                    '--card-glow': catColors.glow,
                  } as React.CSSProperties}
                  onClick={() => handleCardClick(service)}
                  onMouseEnter={() => handleCardHover(service)}
                  onMouseLeave={handleCardLeave}
                >
                  {/* Status dot */}
                  <span
                    className={styles.itemCardStatus}
                    style={{ background: statusColor, color: statusColor }}
                  />

                  {/* New/Beta badge */}
                  {(service.status === 'beta' || service.status === 'development') && (
                    <span
                      className={styles.itemCardBadge}
                      style={{
                        background: service.status === 'beta'
                          ? 'rgba(251, 191, 36, 0.9)'
                          : 'rgba(96, 165, 250, 0.9)',
                      }}
                    >
                      {service.status === 'beta' ? 'β' : 'dev'}
                    </span>
                  )}

                  <span className={styles.itemCardIcon}>{service.icon}</span>
                  <span className={styles.itemCardName}>{service.name}</span>

                  {/* Rarity strip */}
                  <span
                    className={styles.itemCardStrip}
                    style={{
                      background: `linear-gradient(90deg, transparent, ${catColors.border}, transparent)`,
                    }}
                  />
                </div>
              );
            })}
          </div>
        </div>

        {/* CENTER: Miou character */}
        <div className={styles.centerPanel}>
          {/* Decorative particles */}
          {[...Array(6)].map((_, i) => (
            <span
              key={i}
              className={styles.particle}
              style={{
                left: `${20 + Math.random() * 60}%`,
                top: `${10 + Math.random() * 80}%`,
                animationDelay: `${i * 1.2}s`,
                animationDuration: `${6 + Math.random() * 4}s`,
              }}
            />
          ))}

          <div className={styles.miouContainer}>
            {/* Speech bubble */}
            <div className={styles.miouBubble} key={miouText}>
              <div className={styles.miouName}>Miou</div>
              {miouText}
            </div>

            {/* Character */}
            <div className={styles.miouCharacter}>
              <span className={styles.miouGlow} />
              <MiouMascot size={160} className={styles.miouSvg} />
            </div>
          </div>
        </div>

        {/* RIGHT: Detail panel */}
        {selectedService ? (
          <div className={styles.detailPanel} key={selectedService.id}>
            <DetailView service={selectedService} detailed={detailedInfo} />
          </div>
        ) : (
          <div className={styles.detailEmpty}>
            Sélectionne un service
          </div>
        )}
      </div>
    </div>
  );
}

/* ===== Detail View sub-component ===== */

interface DetailViewProps {
  service: ServiceMeta;
  detailed?: ReturnType<typeof getServiceById>;
}

function DetailView({ service, detailed }: DetailViewProps) {
  const catColors = CATEGORY_COLORS[service.category];
  const statusColor = STATUS_COLORS[service.status];

  const typeLabels: Record<number, string> = {
    1: 'Interne COG — accessible via Miyukini Central',
    2: 'Surface Web — accessible via Central + Portail',
    3: 'Inter-COG — interactions entre COGs',
  };

  return (
    <>
      {/* Header */}
      <div className={styles.detailHeader}>
        <div
          className={styles.detailIcon}
          style={{
            background: catColors.bg,
            borderColor: catColors.border,
          }}
        >
          {service.icon}
        </div>
        <div className={styles.detailHeaderInfo}>
          <div
            className={styles.detailCategory}
            style={{ color: catColors.border }}
          >
            {CATEGORY_LABELS[service.category]}
          </div>
          <h2 className={styles.detailName}>{service.name}</h2>
          <div className={styles.detailMeta}>
            <span className={styles.detailBadge}>
              <span
                className={styles.detailStatusDot}
                style={{ background: statusColor }}
              />
              {STATUS_LABELS[service.status]}
            </span>
            <span className={styles.detailBadge}>
              Type {service.type}
            </span>
          </div>
        </div>
      </div>

      {/* Description */}
      <p className={styles.detailDesc}>
        {detailed?.longDesc || service.shortDesc}
      </p>

      {/* Type */}
      <div className={styles.detailTypeCard}>
        <div className={styles.detailTypeLabel}>Type de Service</div>
        <div className={styles.detailTypeValue}>
          {typeLabels[service.type]}
        </div>
      </div>

      {/* Features */}
      {detailed && detailed.features.length > 0 && (
        <div className={styles.detailSection}>
          <div className={styles.detailSectionTitle}>Fonctionnalités</div>
          {detailed.features.slice(0, 4).map((f, i) => (
            <div key={i} className={styles.detailFeature}>
              <span className={styles.detailFeatureIcon}>{f.icon}</span>
              <span className={styles.detailFeatureText}>
                <strong>{f.title}</strong> — {f.description}
              </span>
            </div>
          ))}
        </div>
      )}

      {/* Integrations */}
      {detailed && detailed.integrations.length > 0 && (
        <div className={styles.detailSection}>
          <div className={styles.detailSectionTitle}>Intégrations</div>
          {detailed.integrations.map((int, i) => (
            <div key={i} className={styles.detailFeature}>
              <span className={styles.detailFeatureIcon}>🔗</span>
              <span className={styles.detailFeatureText}>
                <strong>{int.serviceName}</strong> — {int.description}
              </span>
            </div>
          ))}
        </div>
      )}

      {/* CTA */}
      <div className={styles.detailCta}>
        <Link
          to={`/services/${service.id}`}
          className={`${styles.detailCtaBtn} ${styles.detailCtaPrimary}`}
        >
          En savoir plus →
        </Link>
      </div>
    </>
  );
}
