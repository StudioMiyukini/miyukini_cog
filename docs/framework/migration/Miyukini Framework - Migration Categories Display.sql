-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION CATEGORIES DISPLAY
-- Module: Catégories de Navigation
-- Version: 1.1.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Ajouter une visibilité globale par catégorie (afficher / masquer)
-- - Permettre au back-office de piloter quelles catégories sont proposées
--   (en complément des préférences utilisateur).
--
-- Dépendance :
-- - Miyukini Framework - Migration Categories.sql
-- =============================================

ALTER TABLE public.categories
  ADD COLUMN IF NOT EXISTS is_visible BOOLEAN NOT NULL DEFAULT true;

COMMENT ON COLUMN public.categories.is_visible IS
  'Visibilité globale de la catégorie (true = affichable dans la navigation).';

CREATE INDEX IF NOT EXISTS idx_categories_is_visible ON public.categories(is_visible);

-- =============================================
-- FIN DE LA MIGRATION CATEGORIES DISPLAY
-- =============================================

