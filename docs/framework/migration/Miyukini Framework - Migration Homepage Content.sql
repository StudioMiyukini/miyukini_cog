-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION HOMEPAGE CONTENT
-- Module: Home / Landing Page
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Stocker une configuration éditable de la homepage (/)
-- - Permettre l'édition via le back-office (admin)
-- - Exposer une lecture publique uniquement si publié
--
-- Dépendances :
-- - Miyukini Framework - Migration Initiale.sql (handle_updated_at + helpers RLS)
-- =============================================

CREATE TABLE IF NOT EXISTS public.homepage_content (
  id TEXT PRIMARY KEY,
  is_published BOOLEAN NOT NULL DEFAULT true,
  config JSONB NOT NULL DEFAULT '{}'::jsonb,
  updated_by UUID NULL REFERENCES auth.users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMENT ON TABLE public.homepage_content IS 'Configuration éditable de la homepage (landing).';

CREATE INDEX IF NOT EXISTS idx_homepage_content_published ON public.homepage_content(is_published);

-- updated_at trigger
DROP TRIGGER IF EXISTS homepage_content_updated_at ON public.homepage_content;
CREATE TRIGGER homepage_content_updated_at
  BEFORE UPDATE ON public.homepage_content
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- RLS
ALTER TABLE public.homepage_content ENABLE ROW LEVEL SECURITY;

-- Lecture publique uniquement si publié
DROP POLICY IF EXISTS homepage_content_select_published ON public.homepage_content;
CREATE POLICY homepage_content_select_published ON public.homepage_content
  FOR SELECT
  USING (is_published = true);

-- Lecture admin (y compris brouillons)
DROP POLICY IF EXISTS homepage_content_select_admin ON public.homepage_content;
CREATE POLICY homepage_content_select_admin ON public.homepage_content
  FOR SELECT
  USING (public.is_admin_user());

-- Ecriture admin uniquement
DROP POLICY IF EXISTS homepage_content_write_admin ON public.homepage_content;
CREATE POLICY homepage_content_write_admin ON public.homepage_content
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

-- Seed (1 ligne singleton : id = 'home')
INSERT INTO public.homepage_content (id, is_published, config)
VALUES (
  'home',
  true,
  '{
    "hero": {
      "badgeText": "Framework modulaire depuis 2016",
      "title": { "line1": "Creez. Deployez.", "line2": "Impressionnez.", "highlight": true },
      "subtitle": "Miyukini est un framework modulaire pense pour creer des applications web et mobiles modernes, performantes et portables.",
      "primaryCta": { "label": "Explorer les fonctionnalites", "href": "/mockcontent" },
      "secondaryCta": { "label": "Voir le dashboard", "href": "/admin" }
    },
    "sectionsEnabled": { "stats": true, "quickActions": true, "faq": true, "onboarding": true },
    "stats": [
      { "value": "8+", "label": "Annees d''experience" },
      { "value": "150+", "label": "Composants UI" },
      { "value": "4.9", "label": "Note moyenne" },
      { "value": "10K+", "label": "Utilisateurs satisfaits" }
    ],
    "quickActions": [
      { "id": "content", "label": "Contenu", "icon": "icon-[tabler--article]", "path": "/mockcontent", "description": "Explorez le contenu" },
      { "id": "profile", "label": "Profil", "icon": "icon-[tabler--user]", "path": "/profile", "description": "Gerez votre compte" },
      { "id": "admin", "label": "Admin", "icon": "icon-[tabler--dashboard]", "path": "/admin", "description": "Tableau de bord" }
    ],
    "faq": [
      { "title": "Comment demarrer avec Miyukini ?", "content": "Creez un compte, explorez les modules depuis la page d''accueil, et personnalisez votre profil dans les parametres.", "defaultOpen": true },
      { "title": "Quelles sont les fonctionnalites principales ?", "content": "Miyukini offre un systeme d''authentification, un back-office complet, des composants UI reutilisables et une architecture portable." },
      { "title": "L''application fonctionne-t-elle sur mobile ?", "content": "Oui ! Miyukini est concu responsive et portable vers les applications natives Android et iOS grace a son architecture \"Screens\"." }
    ],
    "onboardingSteps": [
      { "id": 1, "title": "Bienvenue sur Miyukini", "description": "Decouvrez un framework modulaire concu pour creer des applications web et mobiles rapidement.", "icon": "icon-[tabler--sparkles]" },
      { "id": 2, "title": "Navigation intuitive", "description": "Utilisez la barre de navigation pour acceder a toutes les fonctionnalites en un clic.", "icon": "icon-[tabler--layout-navbar]" },
      { "id": 3, "title": "Pret pour le mobile", "description": "Chaque ecran est pense pour etre portable vers Android et iOS nativement.", "icon": "icon-[tabler--device-mobile]" }
    ]
  }'::jsonb
)
ON CONFLICT (id) DO NOTHING;

-- =============================================
-- FIN DE LA MIGRATION HOMEPAGE CONTENT
-- =============================================

