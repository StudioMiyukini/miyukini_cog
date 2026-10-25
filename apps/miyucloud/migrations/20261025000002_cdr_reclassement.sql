-- @id     miyucloud_migration_cdr_reclassement
-- @do     indexer_fichiers_cdr_inconnus
-- @role   securite
-- @layer  app
-- @human  miyucloud — index du reclassement CDR des fichiers hérités `inconnu` (plan §5) ; idempotente.
--
-- Plan §5 (lot 3, reste) : les fichiers antérieurs au CDR (état `inconnu`,
-- défaut de 20261025000001_cdr_desarmement) ne sont jamais servis ; la tâche
-- de reclassement (ReclassementCdrService) les repasse par le CDR :
-- désarmable ⇒ `desarme`, non désarmable ⇒ `quarantaine` (décision du
-- propriétaire, A-004). Aucune colonne nouvelle : le compte rendu
-- `cdr_rapport` porte `"reclassement": true`.
--
-- Index partiel : la tâche parcourt les seuls fichiers `inconnu` par
-- identifiant croissant (corbeille comprise), sans balayer la table. Il se
-- vide à mesure du reclassement.
--
-- Idempotente (IF NOT EXISTS) ; ne modifie aucune donnée.

CREATE INDEX IF NOT EXISTS idx_files_etat_cdr_inconnu
    ON storage.files (id)
    WHERE etat_cdr = 'inconnu';

COMMENT ON COLUMN storage.files.etat_cdr IS
    'EX-45 : desarme | quarantaine | inconnu (antérieur au CDR : jamais servi, repassé par le CDR par la tâche de reclassement)';
