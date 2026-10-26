# Politique de sécurité — Miyukini COG

<!-- @id securite.politique -->
<!-- @role security-policy -->
<!-- @layer 0-9+zones -->
<!-- @human Politique publique de signalement des vulnérabilités : versions couvertes, canal, délais, divulgation coordonnée, périmètre -->
<!-- @do report_vulnerability -->

- **Éditeur** : Studio Miyukini · **Mise à jour** : 2026-10-26
- **Cadre** : ISO/IEC 29147 (divulgation), ISO/IEC 30111 (traitement), règlement (UE) 2024/2847 « CRA » art. 13 et 14
- **Processus interne** : [docs/securite/PROCESSUS-SIGNALEMENT-CRA.md](docs/securite/PROCESSUS-SIGNALEMENT-CRA.md)
- **Fichier machine** : `security.txt` (RFC 9116), à servir sur `https://miyukini.com/.well-known/security.txt` — source : [docs/doc_for_website/.well-known/security.txt](docs/doc_for_website/.well-known/security.txt)

## Versions prises en charge

<!-- @id securite.politique.versions -->
<!-- @role supported-versions -->
<!-- @do list_supported_versions -->

| Version | Correctifs de sécurité |
|---|---|
| `0.9.x` (dernière publiée : `0.9.0-beta.1`) | oui |
| branche `main` | oui (corrigée en premier) |
| < `0.9` | non — mettre à jour |

- **Période de support** : `<À DÉFINIR PAR LE PROPRIÉTAIRE>` — le CRA impose **au moins 5 ans**, sauf durée d'utilisation attendue plus courte (art. 13 §8)
- Les correctifs de sécurité sont gratuits et diffusés par le **canal officiel signé**

## Signaler une vulnérabilité

<!-- @id securite.politique.signaler -->
<!-- @role reporting-channel -->
<!-- @do submit_vulnerability_report -->

- **Ne pas** ouvrir de ticket, de discussion ou de demande de fusion publique
- **Contact de sécurité (point de contact unique, CRA art. 13 §17)** : `<À DÉFINIR PAR LE PROPRIÉTAIRE>`
- **Chiffrement** : aucune clé publique n'est publiée à ce jour ; `<À DÉFINIR PAR LE PROPRIÉTAIRE>` (clé OpenPGP ou équivalent, avec empreinte publiée ici et dans `security.txt`). En attendant, n'envoyez pas de preuve de concept sensible en clair : signalez d'abord l'existence du problème, un canal chiffré sera convenu
- **À fournir** :
  - composant et version (ou commit)
  - description et impact estimé
  - étapes de reproduction, preuve de concept si disponible
  - indice d'exploitation active, si vous en avez connaissance
  - nom ou pseudonyme pour les remerciements (facultatif)
- Langues : français, anglais

## Délais de réponse

<!-- @id securite.politique.delais -->
<!-- @role response-times -->
<!-- @do commit_response_times -->

| Étape | Délai visé |
|---|---|
| Accusé de réception | 3 jours ouvrés |
| Première évaluation (gravité CVSS 4.0) | 10 jours ouvrés |
| Correctif — critique ou haute | 30 jours |
| Correctif — moyenne | 90 jours |
| Correctif — faible | version suivante |
| Point d'avancement au signaleur | au moins tous les 14 jours |

- **Vulnérabilité activement exploitée ou incident grave** : notification sur la plateforme unique de signalement de l'ENISA (alerte précoce **24 h**, notification **72 h**, rapport final — CRA art. 14), puis information des utilisateurs concernés
- Les délais sont des engagements de moyens ; un retard est expliqué au signaleur

## Divulgation coordonnée

<!-- @id securite.politique.divulgation -->
<!-- @role coordinated-disclosure -->
<!-- @do coordinate_disclosure -->

- Démarche ISO/IEC 29147 : nous corrigeons, puis publions un **avis de sécurité** avec le signaleur
- Embargo par défaut : **90 jours** après l'accusé de réception, ou à la livraison du correctif si elle est antérieure ; prolongation possible d'un commun accord
- Exploitation active constatée : l'avis peut être avancé pour protéger les utilisateurs
- Contenu de l'avis : composants et versions touchés, gravité CVSS 4.0, correctif ou contournement, identifiant CVE si attribué, remerciements
- Les avis sont signés et diffusés par le canal officiel ; une vulnérabilité d'une dépendance est aussi signalée à son mainteneur (CRA art. 13 §6)

## Périmètre

<!-- @id securite.politique.perimetre -->
<!-- @role scope -->
<!-- @do define_security_scope -->

- **Dans le périmètre** : tout le code de ce dépôt — Central, Origin, services, Cores, MAIA, Alicia, application mobile, toolkits, paquet d'installation et canal de mise à jour
- **Hors périmètre** :
  - dépendances tierces : à signaler à leurs mainteneurs (nous suivons RustSec via `cargo audit` et `cargo deny`)
  - déni de service volumétrique, ingénierie sociale, attaques physiques
  - tests sur des instances ou données qui ne vous appartiennent pas
- **Recherche de bonne foi** : nous n'engagerons pas de poursuites contre une recherche menée dans ce périmètre, sans atteinte aux données d'autrui ni interruption de service, et signalée selon cette politique (engagement à valider par le propriétaire)

## Remerciements

<!-- @id securite.politique.remerciements -->
<!-- @role acknowledgments -->
<!-- @do credit_reporters -->

- Les signaleurs qui le souhaitent sont cités dans l'avis de sécurité et dans une page de remerciements (`<À DÉFINIR PAR LE PROPRIÉTAIRE>` : emplacement public)
- Pas de programme de prime à ce jour

---

Références internes : `docs/securite/PROCESSUS-SIGNALEMENT-CRA.md`, `docs/conformite/CONFORMITE-SYNTHESE.md`, `docs/conformite/REGLEMENTAIRE-UE-FR.md` (NC-REG-02, NC-REG-03), `docs/conformite/NORMES-ET-STANDARDS.md` (NC-STD-10, NC-STD-11).
