# P0 Temps 05 - Securite

## Actifs a proteger

- Sauvegardes solo et meta-progression
- Integrite des packages installes via Central
- Execution de binaires de jeu depuis Central
- Donnees profil `Identity` si Sodomight y accede
- Futures sessions coop et economie de trade

## Surfaces d'attaque

- packages `.msp` ou installation locale de binaire
- manifests falsifies ou checksum absent
- mods non signes
- corruption de sauvegarde ou injection de stash
- RPC locale entre Central et le jeu
- futures lobbies multijoueur et synchronisation d'etat

## Niveaux requis

- P0/P3 initial : securite standard renforcee sur integrite package et saves
- P4+ multijoueur : securite durcie avec validation d'entrees, messages signes, checksums d'assets, anti-cheat serveur

## Livrable lie

Voir `gpi/2026-03-06-mge-sodomight-security.md`.
