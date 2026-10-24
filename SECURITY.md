# Politique de sécurité — Miyukini COG

Processus de signalement et de traitement des vulnérabilités (ISO/IEC 29147 et 30111, Cyber Resilience Act art. 13-14).

## Signaler une vulnérabilité

- **Ne pas** ouvrir de ticket public pour une vulnérabilité.
- Écrire à : **`<CONTACT-SECURITE — à définir par Studio Miyukini avant toute diffusion>`**
- Indiquer : composant et version, description, étapes de reproduction, impact estimé, preuve de concept si disponible.
- Chiffrement du message : clé publique à publier avec le contact (à définir).

## Engagements de traitement

| Étape | Délai visé |
|---|---|
| Accusé de réception | 3 jours ouvrés |
| Première évaluation (gravité CVSS 4.0) | 10 jours ouvrés |
| Correctif pour une vulnérabilité critique ou haute | 30 jours |
| Publication coordonnée de l'avis | à la livraison du correctif, en accord avec le signaleur |

- Vulnérabilité **activement exploitée** ou incident grave : notification sur la plateforme unique de l'ENISA dans les délais du CRA (alerte précoce 24 h, notification 72 h, rapport final), puis information des utilisateurs.
- Les correctifs de sécurité sont diffusés par le canal officiel signé.

## Périmètre

- Tout le code de ce dépôt : Central, Origin, services, Cores, MAIA, Alicia, application mobile, toolkits.
- Hors périmètre : dépendances tierces (à signaler à leurs mainteneurs ; nous suivons RustSec via `cargo audit` et `cargo deny`).

## Versions prises en charge

- Branche principale et dernière version publiée. Durée de support des versions : à définir (le CRA impose au moins 5 ans, sauf durée d'usage plus courte justifiée).

## Remerciements

Les signaleurs qui le souhaitent sont cités dans l'avis de sécurité.

---

Références internes : `docs/conformite/CONFORMITE-SYNTHESE.md`, `docs/implementation/REVUE-CONSTATS.md`.
