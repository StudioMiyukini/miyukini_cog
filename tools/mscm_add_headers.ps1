Set-Location "C:\Users\miyuk\Cursor\Miyukini-COG"

# Map: relative path -> (id, do_, human)
$data = @{
  'apps/central/src/services/jayfestival/components.rs'      = @('jf_components',         'export_shared_ui_components',    'Composants UI reutilisables JayFestival: StatCard, Badge, TabButton, EmptyState.')
  'apps/central/src/services/jayfestival/exp_agenda.rs'      = @('jf_exp_agenda',          'render_exp_agenda',               'Ecrans EXP-E07/E08 JayFestival: agenda exposant avec animations et creneaux.')
  'apps/central/src/services/jayfestival/exp_candidatures.rs'= @('jf_exp_candidatures',    'render_exp_candidatures',         'Ecrans EXP-E05/E08/E10 JayFestival: candidatures exposant et annuaire evenements.')
  'apps/central/src/services/jayfestival/exp_compte.rs'      = @('jf_exp_compte',          'render_exp_compte',               'Ecran EXP-E13 JayFestival: mon compte exposant (profil, preferences).')
  'apps/central/src/services/jayfestival/exp_dashboard.rs'   = @('jf_exp_dashboard',       'render_exp_dashboard',            'Ecran EXP-E04 JayFestival: tableau de bord exposant avec candidatures et alertes.')
  'apps/central/src/services/jayfestival/exp_documents.rs'   = @('jf_exp_documents',       'render_exp_documents',            'Ecrans EXP-E09/E10 JayFestival: liste et upload des documents exposant.')
  'apps/central/src/services/jayfestival/exp_factures.rs'    = @('jf_exp_factures',        'render_exp_factures',             'Ecrans EXP-E11/E12 JayFestival: factures et suivi des paiements exposant.')
  'apps/central/src/services/jayfestival/exp_fiche_publique.rs'= @('jf_exp_fiche_publique','render_exp_fiche_publique',       'Ecran EXP-E14 JayFestival: apercu et edition de la fiche publique exposant.')
  'apps/central/src/services/jayfestival/exp_notifications.rs'= @('jf_exp_notifications',  'render_exp_notifications',        'Ecran EXP-E15 JayFestival: centre de notifications exposant.')
  'apps/central/src/services/jayfestival/exp_participations.rs'= @('jf_exp_participations','render_exp_participations',       'Ecrans EXP-E05/E06 JayFestival: liste et detail des participations exposant.')
  'apps/central/src/services/jayfestival/mod.rs'             = @('jf_service_mod',         'declare_jayfestival_ui_service',  'Module UI JayFestival: routing multi-role UNC/ORG/EXP/VIS avec donnees reelles KindMother.')
  'apps/central/src/services/jayfestival/org_annonces.rs'    = @('jf_org_annonces',        'render_org_annonces',             'Ecran ORG-E23 JayFestival: envoi annonces et notifications organisateur.')
  'apps/central/src/services/jayfestival/org_budget.rs'      = @('jf_org_budget',          'render_org_budget',               'Ecran ORG-E19 JayFestival: tableau budgetaire edition (revenus, depenses, balance).')
  'apps/central/src/services/jayfestival/org_compte.rs'      = @('jf_org_compte',          'render_org_compte',               'Ecran ORG-E20 JayFestival: mon compte organisateur (profil, parametres).')
  'apps/central/src/services/jayfestival/org_dashboard.rs'   = @('jf_org_dashboard',       'render_org_dashboard',            'Ecran ORG-E04 JayFestival: tableau de bord organisateur avec stats et editions recentes.')
  'apps/central/src/services/jayfestival/org_documents.rs'   = @('jf_org_documents',       'render_org_documents',            'Ecran ORG-E22 JayFestival: documents legaux organisateur (upload/telechargement).')
  'apps/central/src/services/jayfestival/org_edition_hub.rs' = @('jf_org_edition_hub',     'render_org_edition_hub',          'Ecran ORG-E07 JayFestival: hub edition avec navigation par onglets.')
  'apps/central/src/services/jayfestival/org_editions.rs'    = @('jf_org_editions',        'render_org_editions',             'Ecrans ORG-E05/E06 JayFestival: liste des editions et formulaire de creation.')
  'apps/central/src/services/jayfestival/org_equipe.rs'      = @('jf_org_equipe',          'render_org_equipe',               'Ecran ORG-E21 JayFestival: gestion de l equipe organisatrice et invitations.')
  'apps/central/src/services/jayfestival/org_exposants.rs'   = @('jf_org_exposants',       'render_org_exposants',            'Ecrans ORG-E09/E18 JayFestival: gestion des exposants (candidatures, fiches, import CSV).')
  'apps/central/src/services/jayfestival/org_parametres.rs'  = @('jf_org_parametres',      'render_org_parametres',           'Ecran ORG-E08 JayFestival: parametrage edition (dates, lieu, theme, reglement).')
  'apps/central/src/services/jayfestival/org_plan.rs'        = @('jf_org_plan',            'render_org_plan',                 'Ecrans ORG-E14/E16 JayFestival: plan de salle (zones, attribution stands, export).')
  'apps/central/src/services/jayfestival/org_programme.rs'   = @('jf_org_programme',       'render_org_programme',            'Ecran ORG-E17 JayFestival: programme et animations d une edition.')
  'apps/central/src/services/jayfestival/org_publication.rs' = @('jf_org_publication',     'render_org_publication',          'Ecran ORG-E25 JayFestival: publication et cloture (workflow de validation).')
  'apps/central/src/services/jayfestival/org_services.rs'    = @('jf_org_services',        'render_org_services',             'Ecran ORG-E24 JayFestival: activation des services visiteur (jeux, concours, reservations).')
  'apps/central/src/services/jayfestival/sidebar.rs'         = @('jf_sidebar',             'render_sidebar_navigation',       'Sidebar JayFestival: navigation par role (ORG/EXP/VIS) et sections.')
  'apps/central/src/services/jayfestival/unc_auth.rs'        = @('jf_unc_auth',            'render_unc_auth',                 'Ecrans UNC-E11/E14 JayFestival: authentification et modals CTA (connexion, inscription, mentions).')
  'apps/central/src/services/jayfestival/unc_directory.rs'   = @('jf_unc_directory',       'render_unc_directory',            'Ecrans UNC-E06/E09 JayFestival: annuaires publics organisateurs et exposants.')
  'apps/central/src/services/jayfestival/unc_events.rs'      = @('jf_unc_events',          'render_unc_events',               'Ecrans UNC-E02/E03 JayFestival: catalogue des evenements et fiche evenement.')
  'apps/central/src/services/jayfestival/unc_landing.rs'     = @('jf_unc_landing',         'render_unc_landing',              'Ecran UNC-E01 JayFestival: landing page publique (hero, recherche, CTA inscription).')
  'apps/central/src/services/jayfestival/unc_search.rs'      = @('jf_unc_search',          'render_unc_search',               'Ecran UNC-E10 JayFestival: recherche globale (evenements, organisateurs, exposants).')
  'apps/central/src/services/jayfestival/vis_activites.rs'   = @('jf_vis_activites',       'render_vis_activites',            'Ecran VIS-E09 JayFestival: activites visiteur (jeux, concours, pass).')
  'apps/central/src/services/jayfestival/vis_agenda.rs'      = @('jf_vis_agenda',          'render_vis_agenda',               'Ecran VIS-E05 JayFestival: agenda visiteur (animations, creneaux).')
  'apps/central/src/services/jayfestival/vis_billets.rs'     = @('jf_vis_billets',         'render_vis_billets',              'Ecran VIS-E06 JayFestival: billets visiteur (achat, gestion).')
  'apps/central/src/services/jayfestival/vis_catalogue.rs'   = @('jf_vis_catalogue',       'render_vis_catalogue',            'Ecran VIS JayFestival: catalogue des editions publiees et agenda visiteur.')
  'apps/central/src/services/jayfestival/vis_compte.rs'      = @('jf_vis_compte',          'render_vis_compte',               'Ecran VIS-E12 JayFestival: compte visiteur (profil, preferences).')
  'apps/central/src/services/jayfestival/vis_dashboard.rs'   = @('jf_vis_dashboard',       'render_vis_dashboard',            'Ecran VIS-E04 JayFestival: tableau de bord visiteur.')
  'apps/central/src/services/jayfestival/vis_reservations.rs'= @('jf_vis_reservations',    'render_vis_reservations',         'Ecran VIS-E07 JayFestival: reservations visiteur.')
  'apps/central/src/services/jayxpose/catalogue.rs'          = @('jx_catalogue',           'render_catalogue_produits',       'Ecrans XP-E03/E05 JayXpose: liste des produits et gestion des categories.')
  'apps/central/src/services/jayxpose/components.rs'         = @('jx_components',          'export_shared_ui_components',     'Composants UI reutilisables JayXpose: StatCard, Badge, ActionButton, EmptyState, FormField.')
  'apps/central/src/services/jayxpose/dashboard.rs'          = @('jx_dashboard',           'render_dashboard',                'Ecran XP-E01 JayXpose: tableau de bord exposant avec donnees reelles KindMother.')
  'apps/central/src/services/jayxpose/documents.rs'          = @('jx_documents',           'render_documents_vault',          'Ecrans XP-E09/E10 JayXpose: coffre-fort documentaire (liste, upload, partages).')
  'apps/central/src/services/jayxpose/entreprise.rs'         = @('jx_entreprise',          'render_entreprise_profile',       'Ecran XP-E02 JayXpose: gestion du profil exposant enrichi.')
  'apps/central/src/services/jayxpose/fiche_publique.rs'     = @('jx_fiche_publique',      'render_fiche_publique',           'Ecran XP-E12 JayXpose: fiche publique exposant (visibilite, confidentialite, apercu).')
  'apps/central/src/services/jayxpose/mod.rs'                = @('jx_service_mod',         'declare_jayxpose_ui_service',     'Module UI JayXpose: routing par section XP-E01..XP-E12 avec donnees reelles KindMother.')
  'apps/central/src/services/jayxpose/onboarding.rs'         = @('jx_onboarding',          'render_onboarding_flow',          'Onboarding JayXpose: parcours guide Miou en 6 etapes pour creation du profil exposant.')
  'apps/central/src/services/jayxpose/produit_form.rs'       = @('jx_produit_form',        'render_produit_form',             'Ecran XP-E04 JayXpose: formulaire de creation et modification de produit.')
  'apps/central/src/services/jayxpose/sidebar.rs'            = @('jx_sidebar',             'render_sidebar_navigation',       'Sidebar JayXpose: navigation par section (XP-E01 a XP-E12).')
  'apps/central/src/services/jayxpose/vitrine.rs'            = @('jx_vitrine',             'render_vitrine',                  'Ecrans XP-E06/07/08 JayXpose: parametres vitrine, pages et previsualisation.')
}

$ok = 0; $skip = 0
foreach ($rel in $data.Keys) {
  $fullpath = Join-Path (Get-Location) $rel
  if (-not (Test-Path $fullpath)) { Write-Host "MISSING: $rel"; $skip++; continue }
  $lines = Get-Content $fullpath -Encoding UTF8
  if ($lines | Where-Object { $_ -match '@id:' }) { Write-Host "SKIP(already @id): $rel"; $skip++; continue }

  # Find last consecutive //! line at top
  $lastDocLine = -1
  for ($i = 0; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match '^//!') { $lastDocLine = $i }
    elseif ($i -eq 0) { break }
    elseif ($lastDocLine -ge 0) { break }
  }
  if ($lastDocLine -lt 0) { Write-Host "NODOC: $rel"; $skip++; continue }

  $id   = $data[$rel][0]
  $do_  = $data[$rel][1]
  $hum  = $data[$rel][2]
  $mscm = @("//!", "//! @id: $id @do: $do_", "//! @role: ui @layer: service", "//! @human: $hum")

  $before = $lines[0..$lastDocLine]
  if ($lastDocLine + 1 -lt $lines.Count) {
    $after = $lines[($lastDocLine + 1)..($lines.Count - 1)]
  } else {
    $after = @()
  }
  $newlines = $before + $mscm + $after
  Set-Content $fullpath -Value $newlines -Encoding UTF8
  Write-Host "OK: $rel"
  $ok++
}
Write-Host "`n=== $ok updated, $skip skipped ==="
