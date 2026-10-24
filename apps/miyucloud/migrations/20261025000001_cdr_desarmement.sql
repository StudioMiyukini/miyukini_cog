-- @id     miyucloud_migration_cdr_desarmement
-- @do     ajouter_etat_cdr_fichiers
-- @role   securite
-- @layer  app
-- @human  miyucloud — état de désarmement (CDR) de chaque fichier : désarmé, quarantaine ou inconnu.
--
-- Décision A-004 / EX-45 : tout fichier entrant dans MiyuCloud passe par le CDR
-- (crates/miyucdr). L'état est porté par la ligne storage.files (et non par le
-- blob : des octets identiques peuvent être désarmés pour un fichier et stockés
-- en quarantaine pour un autre).
--
--   desarme     : octets stockés = octets reconstruits par le CDR ;
--   quarantaine : type non pris en charge ou trop volumineux, octets d'origine
--                 stockés à part, jamais ouverts par COG, téléchargement sur
--                 accord explicite uniquement ;
--   inconnu     : fichiers antérieurs à cette migration (valeur par défaut) et
--                 toute écriture qui ne déclare pas d'état — traités comme non
--                 désarmés pour tout traitement de contenu (fermé par défaut).

ALTER TABLE storage.files
    ADD COLUMN IF NOT EXISTS etat_cdr TEXT NOT NULL DEFAULT 'inconnu';

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'files_etat_cdr_check'
    ) THEN
        ALTER TABLE storage.files
            ADD CONSTRAINT files_etat_cdr_check
            CHECK (etat_cdr IN ('desarme', 'quarantaine', 'inconnu'));
    END IF;
END
$$;

ALTER TABLE storage.files
    ADD COLUMN IF NOT EXISTS cdr_rapport JSONB;

COMMENT ON COLUMN storage.files.etat_cdr IS
    'EX-45 : desarme | quarantaine | inconnu (antérieur au CDR, traité comme non désarmé)';
COMMENT ON COLUMN storage.files.cdr_rapport IS
    'EX-45 : compte rendu résumé du CDR (retraits, anomalies) ou motif de quarantaine';

-- Copie récursive de dossier : reprise de la fonction initiale, en conservant
-- l'état CDR des fichiers copiés.
CREATE OR REPLACE FUNCTION storage.copy_folder_tree(
    p_source_id UUID,
    p_target_parent_id UUID,       -- NULL = copy to root
    p_dest_name TEXT DEFAULT NULL   -- NULL = keep source folder name
) RETURNS TABLE(new_root_id TEXT, folders_copied BIGINT, files_copied BIGINT) AS $$
DECLARE
    v_root_lpath   ltree;
    v_root_depth   INT;
    v_max_depth    INT;
    v_level        INT;
    v_folders      BIGINT := 0;
    v_files        BIGINT := 0;
    v_inserted     BIGINT;
    v_new_root     UUID;
BEGIN
    -- Validate source exists
    SELECT fo.lpath, nlevel(fo.lpath)
      INTO v_root_lpath, v_root_depth
      FROM storage.folders fo
     WHERE fo.id = p_source_id AND NOT fo.is_trashed;

    IF v_root_lpath IS NULL THEN
        RAISE EXCEPTION 'Source folder not found: %', p_source_id
            USING ERRCODE = 'P0002';  -- no_data_found
    END IF;

    -- Temp mapping: every folder in the subtree → new UUID
    CREATE TEMP TABLE IF NOT EXISTS _copy_map(
        old_id UUID PRIMARY KEY,
        new_id UUID NOT NULL DEFAULT gen_random_uuid()
    ) ON COMMIT DROP;
    TRUNCATE _copy_map;

    INSERT INTO _copy_map(old_id)
    SELECT fo.id
      FROM storage.folders fo
     WHERE NOT fo.is_trashed
       AND fo.lpath <@ v_root_lpath;

    -- Remember new root ID
    SELECT cm.new_id INTO v_new_root
      FROM _copy_map cm WHERE cm.old_id = p_source_id;

    -- Max depth for level iteration
    SELECT MAX(nlevel(fo.lpath))
      INTO v_max_depth
      FROM storage.folders fo
      JOIN _copy_map cm ON fo.id = cm.old_id;

    -- ── Insert folders level by level ──
    -- Each level is a separate INSERT so that the BEFORE INSERT trigger
    -- (trg_folders_path) can resolve the parent's path/lpath from rows
    -- inserted in the previous level.
    FOR v_level IN v_root_depth .. v_max_depth LOOP
        INSERT INTO storage.folders(id, name, parent_id, user_id)
        SELECT cm.new_id,
               CASE WHEN fo.id = p_source_id AND p_dest_name IS NOT NULL
                    THEN p_dest_name ELSE fo.name END,
               CASE WHEN fo.id = p_source_id THEN p_target_parent_id
                    ELSE pm.new_id END,
               fo.user_id
          FROM storage.folders fo
          JOIN _copy_map cm ON fo.id = cm.old_id
          LEFT JOIN _copy_map pm ON fo.parent_id = pm.old_id
         WHERE NOT fo.is_trashed
           AND nlevel(fo.lpath) = v_level;

        GET DIAGNOSTICS v_inserted = ROW_COUNT;
        v_folders := v_folders + v_inserted;
    END LOOP;

    -- ── Batch copy all files (zero-copy: same blob_hash) ──
    -- EX-45 : la copie conserve l'état CDR et le compte rendu de la source
    -- (mêmes octets, même blob).
    INSERT INTO storage.files(name, folder_id, user_id, blob_hash, size, mime_type, media_sort_date,
                              etat_cdr, cdr_rapport)
    SELECT f.name, cm.new_id, f.user_id, f.blob_hash, f.size, f.mime_type, f.media_sort_date,
           f.etat_cdr, f.cdr_rapport
      FROM storage.files f
      JOIN _copy_map cm ON f.folder_id = cm.old_id
     WHERE NOT f.is_trashed;

    GET DIAGNOSTICS v_files = ROW_COUNT;

    -- ── Batch increment blob ref_counts ──
    IF v_files > 0 THEN
        UPDATE storage.blobs b
           SET ref_count = ref_count + hc.cnt
          FROM (
              SELECT f.blob_hash, COUNT(*)::int AS cnt
                FROM storage.files f
                JOIN _copy_map cm ON f.folder_id = cm.new_id
               WHERE NOT f.is_trashed
               GROUP BY f.blob_hash
          ) hc
         WHERE b.hash = hc.blob_hash;
    END IF;

    RETURN QUERY SELECT v_new_root::text, v_folders, v_files;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION storage.copy_folder_tree(UUID, UUID, TEXT)
    IS 'Atomic recursive folder copy using ltree — O(depth) + 1 batch file copy + 1 batch ref_count update; keeps etat_cdr (EX-45)';
