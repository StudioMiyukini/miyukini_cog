# Positionnement professionnel et industriel de COG

<!-- @id market.positionnement -->
<!-- @role market-positioning -->
<!-- @layer 0-9+zones -->
<!-- @human Où se situe COG sur le marché au 2026-10-25 : ce qu'il est, sa maturité, ses concurrents par segment, ses risques industriels, le marché d'entrée recommandé -->
<!-- @do position_cog_on_market -->

- **Date** : 2026-10-25 · **Statut** : analyse de travail, non contractuelle ; concurrents cités de connaissance générale, **à vérifier** avant tout usage externe
- **Sources internes** : [doc gelée](../architecture/GEL-DOC-CONCEPTUELLE-v1.md), [plan d'implémentation](../implementation/PLAN-IMPLEMENTATION.md), [synthèse conformité](../conformite/CONFORMITE-SYNTHESE.md), [Alliances](../architecture/alliance/_index.md)

## Contexte

- COG a une conception gelée et un prototype avancé ; avant de choisir la suite (Alliances, certification, premiers clients), il faut savoir où il se place et contre qui

## Portée / Scope

- Positionnement produit et marché ; pas de chiffrage financier, pas d'étude de marché quantifiée
- Les décisions de positionnement restent au propriétaire (voir §6)

## 1. Ce qu'est COG

- **Plateforme souveraine auto-hébergée avec IA locale**, en Rust, qui réunit des briques vendues séparément ailleurs

| Brique COG | Catégorie de marché |
|---|---|
| Identité signée (passeport, visa, Sceau) | IAM, PKI |
| Douane, Vigies, CDR, quarantaine | pare-feu, IDS/IPS, désarmement de contenu |
| MAIA (IA locale, mémoire chiffrée, petits modèles à Strata) | IA locale d'entreprise |
| Services Jay* (agenda, compta, documents, cloud, messagerie) | suite collaborative |
| Alicia | domotique |
| Fédération | sans équivalent grand public |
| Alliances (v2, en conception) | annuaire d'entreprise (type Active Directory) |

- **Principe différenciant** : l'IA propose, des règles déterministes décident (Cores, quorum, validation humaine) ; refus par défaut ; aucune destruction à distance ; données sur place

## 2. Maturité au 2026-10-25

- **Conception** : doc v1 gelée et amendée, modèle de menaces, conception v2 des Alliances (41 décisions)
- **Code** : ≈ 150 crates ; lots 0, C et 2 atteints ; lots 4 à 7 atteints **au niveau bibliothèque** (sans réseau, en partie non commités)
- **Produit** : pas de déploiement en production, pas de client, pas d'audit externe, pas de certification
- **Lecture** : architecture de référence avec un prototype solide ; pas encore un produit vendable

## 3. Concurrence par segment

| Segment | Acteurs | Avantage COG | Faiblesse COG |
|---|---|---|---|
| Collaboratif souverain | Nextcloud, BlueMind, Linagora (Twake), Suite numérique de l'État | IA locale gouvernée, sécurité par conception | acteurs mûrs, références publiques |
| Annuaire / alternative à AD | Univention (UCS), FreeIPA, Samba AD | quorum tolérant aux membres compromis, IA d'administration bornée | Windows seulement après une Alliance entièrement COG (AL-D07) |
| Sécurité réseau | Stormshield, OPNsense | intégration, modèle « la zone COG tire » | aucune certification ; CRA classe II probable si présenté comme pare-feu |
| IA locale d'entreprise | Ollama et dérivés, intégrateurs | mémoire chiffrée, pas de données personnelles (AL-D37), refus par défaut | matériel et modèles à dimensionner |
| Produit sécurisé certifié | Olvid (CSPN) | — | exemple à suivre : un produit étroit, bien certifié |

- Le différenciateur n'est pas une brique : c'est la **combinaison** IA locale + gouvernance vérifiable + aucun cloud imposé

## 4. Risques industriels

- **Périmètre** : sept marchés à la fois, chacun avec des leaders installés
- **Obligations en vigueur non couvertes** : signalement CRA (depuis le 11/09/2026), mention IA de l'AI Act art. 50 (depuis le 02/08/2026) ; elles bloquent toute diffusion
- **Cryptographie** non alignée sur le référentiel ANSSI (Ed25519, X25519 non cités) : obstacle à une qualification, à justifier pour une CSPN
- **Classe CRA** non tranchée : « pare-feu » ⇒ classe II, évaluation par tiers
- **Coût d'entrée des Alliances** (6 à 7 COG au CC) : trop lourd pour une petite structure ([revue AL-F07](../architecture/alliance/ALLIANCE-REVUE-FAILLES.md))

## 5. Recommandation

1. **Marché d'entrée** : TPE, PME et professions réglementées (santé, droit, expertise comptable, petites collectivités) — un COG seul multi-utilisateurs, IA locale, compta et facturation électronique (réception au 01/09/2026, émission au 01/09/2027 ; agrément via plateforme à prévoir pour JayKonta)
2. **Première certification** : CSPN sur `miyusceau` et le canal de mise à jour (déjà prévue)
3. **Alliances en deuxième temps** : « annuaire souverain avec IA d'administration bornée », pour ETI et collectivités, face à Univention
4. **Immédiatement** : lot 0 bis (SECURITY.md, security.txt, processus CRA, mention IA)

## 6. Décisions à prendre (propriétaire)

- Marché d'entrée retenu (§5.1) ou autre
- Présentation CRA : système d'exploitation (classe I) ou pare-feu (classe II)
- Seuil à partir duquel une Alliance est proposée (AL-Q51)

---

Date de mise a jour : 2026-10-25
