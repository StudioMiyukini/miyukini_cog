<!-- @id mem.lessons.sequences
     @do capture_sequence_lessons
     @role overview
     @layer memory
     @human Lecons apprises par sequence -->

# Lecons apprises

## Lecons

- Le pattern `academy + evidence + sync` permet de scaler 37 certifications sans monolithique.
- Les scripts de synchronisation reduisent les ecarts entre statut documentaire et statut machine.
- Les vagues par domaine (security, architecture, delivery, ops) simplifient le pilotage FULL autopilot.
- Le pattern `toolkits API-first + service UX` facilite la reutilisation (Alicia, Central) et isole le backlog UX.
- Une cloture "accepte avec reserves" permet de terminer sans masquer les manques non bloquants.
- Le pattern `migration legacy one-shot sur login` permet de basculer une auth historique sans freeze utilisateur.
- Exposer l'etat runtime auth (`ONLINE/DEGRADED/ISOLATED/SUSPICIOUS`) directement dans l'UI reduit les ambiguïtes de support.
- Les reserves P4 doivent rester visibles jusqu'au rapport P6, meme si le gate P5 est valide.
- Une sequence T5 avec etapes numerotees (E0-E10 + BUF) et un DAG explicite permet un suivi granulaire sans perte de visibilite.
- Separer `store_blob` (insert ou incr refcount) et `record_file_blob` (lien file<->blob) permet un dedup propre sans couplage fort.
- Les tests d'integration avec FK doivent creer toutes les entites parentes (`file_create`) avant de tester les tables dependantes.
- Un score securite >= 95/100 est atteignable sur un serveur cloud self-hosted Rust avec: CSP nonce/req, HSTS, rate limiting, HMAC constant-time, IP SHA-256, WAL SQLite, validate_path, quick-xml sans expansion entites.
