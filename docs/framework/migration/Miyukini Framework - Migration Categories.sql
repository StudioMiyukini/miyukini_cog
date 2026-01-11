-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION CATEGORIES
-- Module: Catégories de Navigation
-- Version: 1.0.0
-- Date: 2026-01-10
-- =============================================
-- 
-- Cette migration crée les tables pour la gestion
-- des catégories de navigation et les préférences
-- utilisateur.
--
-- Dépendance: Migration Initiale (profiles)
-- =============================================

-- =============================================
-- PARTIE 1: TABLE CATEGORIES
-- =============================================

-- Table des catégories de navigation globales
CREATE TABLE public.categories (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  icon_class TEXT NOT NULL,
  path TEXT NOT NULL,
  is_default BOOLEAN DEFAULT false,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- Index pour optimiser le tri
CREATE INDEX idx_categories_order ON public.categories(sort_order);

-- Commentaire descriptif
COMMENT ON TABLE public.categories IS 'Catégories de navigation globales - Module Miyukini';

-- =============================================
-- PARTIE 2: TABLE USER_CATEGORY_PREFERENCES
-- =============================================

-- Table des préférences de catégories par utilisateur
CREATE TABLE public.user_category_preferences (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  category_id TEXT NOT NULL REFERENCES public.categories(id) ON DELETE CASCADE,
  enabled BOOLEAN NOT NULL DEFAULT false,
  custom_order INTEGER,
  created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  
  -- Contrainte: un utilisateur ne peut avoir qu'une préférence par catégorie
  UNIQUE(user_id, category_id)
);

-- Index pour optimiser les requêtes
CREATE INDEX idx_user_category_prefs_user ON public.user_category_preferences(user_id);

-- Commentaire descriptif
COMMENT ON TABLE public.user_category_preferences IS 'Préférences de catégories par utilisateur - Module Miyukini';

-- =============================================
-- PARTIE 3: RLS POLICIES
-- =============================================

-- Activer RLS
ALTER TABLE public.categories ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.user_category_preferences ENABLE ROW LEVEL SECURITY;

-- Policy: Tout le monde peut lire les catégories
CREATE POLICY categories_select_all ON public.categories
  FOR SELECT
  USING (true);

-- Policy: Seuls les admins peuvent modifier les catégories
CREATE POLICY categories_admin_all ON public.categories
  FOR ALL
  USING (
    public.is_admin_user()
  );

-- Policy: Les utilisateurs peuvent voir leurs préférences
CREATE POLICY prefs_select_own ON public.user_category_preferences
  FOR SELECT
  USING (auth.uid() = user_id);

-- Policy: Les utilisateurs peuvent créer leurs préférences
CREATE POLICY prefs_insert_own ON public.user_category_preferences
  FOR INSERT
  WITH CHECK (auth.uid() = user_id);

-- Policy: Les utilisateurs peuvent modifier leurs préférences
CREATE POLICY prefs_update_own ON public.user_category_preferences
  FOR UPDATE
  USING (auth.uid() = user_id);

-- Policy: Les utilisateurs peuvent supprimer leurs préférences
CREATE POLICY prefs_delete_own ON public.user_category_preferences
  FOR DELETE
  USING (auth.uid() = user_id);

-- =============================================
-- PARTIE 4: TRIGGERS
-- =============================================

-- Trigger updated_at sur categories
CREATE TRIGGER categories_updated_at
  BEFORE UPDATE ON public.categories
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- Trigger updated_at sur user_category_preferences
CREATE TRIGGER user_category_prefs_updated_at
  BEFORE UPDATE ON public.user_category_preferences
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- =============================================
-- PARTIE 5: DONNÉES INITIALES
-- =============================================

-- Insérer les 8 catégories par défaut
-- Seule "Accueil" est activée par défaut (is_default = true)
INSERT INTO public.categories (id, name, icon_class, path, is_default, sort_order) VALUES
  ('cat_1', 'Accueil', 'icon-[tabler--home]', '/', true, 1),
  ('cat_2', 'Explorer', 'icon-[tabler--compass]', '/category/explorer', false, 2),
  ('cat_3', 'Favoris', 'icon-[tabler--heart]', '/category/favoris', false, 3),
  ('cat_4', 'Messages', 'icon-[tabler--message]', '/category/messages', false, 4),
  ('cat_5', 'Notifications', 'icon-[tabler--bell]', '/category/notifications', false, 5),
  ('cat_6', 'Recherche', 'icon-[tabler--search]', '/category/recherche', false, 6),
  ('cat_7', 'Paramètres', 'icon-[tabler--settings]', '/category/parametres', false, 7),
  ('cat_8', 'Aide', 'icon-[tabler--help]', '/category/aide', false, 8);

-- =============================================
-- FIN DE LA MIGRATION CATEGORIES
-- =============================================
