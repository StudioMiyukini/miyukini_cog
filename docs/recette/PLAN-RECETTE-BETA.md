# Plan de recette — COG 0.9.0-beta.1

<!-- @id recette.plan -->
<!-- @role acceptance-plan -->
<!-- @layer 0-9+zones -->
<!-- @human Plan de la recette de la beta 0.9.0-beta.1 : périmètre, prérequis, critères, rôles, procédure, anomalies -->
<!-- @do plan_beta_acceptance -->

- **Statut** : v1.0, 2026-10-25 · **Cible** : COG `0.9.0-beta.1`, déploiement de test chez un testeur
- **Base** : [vue globale](../architecture/COG-VUE-GLOBALE.md) · [doc gelée, amendements A-001 à A-015](../architecture/GEL-DOC-CONCEPTUELLE-v1.md) · [menaces EX-01..63](../securite/MODELE-MENACES-COG.md) · [matrice](../implementation/MATRICE-EXIGENCES.md) · [plan d'implémentation](../implementation/PLAN-IMPLEMENTATION.md) · [`deploy/cog-vm/`](../../deploy/cog-vm/README.md) · [banc lot 3](../../tests/intrusion-lot3/README.md)
- **Documents liés** : [cas de test](CAS-DE-TEST-BETA.md) · [limites connues](LIMITES-CONNUES-BETA.md) · [modèle de PV](PV-RECETTE-MODELE.md)
- **Nature** : recette d'un déploiement de test. Ce n'est **pas** une mise en production.

## 1. Avertissements

<!-- @id recette.plan.avertissements -->
<!-- @role warnings -->
<!-- @do warn_tester -->

- La matrice EX date du 2026-10-24 (avant les lots 3 à 7) : la recette **ne s'appuie pas** sur ses statuts, seulement sur les comportements observés
- Machine de recette **dédiée** (physique ou VM) : installateur `deploy/recette/installer.sh` du paquet signé (voir [notes de version](NOTES-DE-VERSION-0.9.0-beta.1.md))
  - il refuse une machine qui ressemble à une production et exige `--confirme-machine-de-recette` (voir [L-14](LIMITES-CONNUES-BETA.md))
  - `deploy/cog-vm/installer.sh` reste réservé à la VM jetable du mainteneur
- Jamais sur un serveur de production, un poste de travail ni une VM durable : l'installation crée des comptes, réécrit des unités, installe PostgreSQL, charge des règles nftables
- Version `0.9.0-beta.1` (workspace) ; le paquet recetté est identifié par le **commit et l'empreinte de son `MANIFESTE` signé** (mainteneur : étiqueter ce commit avant la remise)

## 2. Périmètre

<!-- @id recette.plan.perimetre -->
<!-- @role scope -->
<!-- @do define_scope -->

### 2.1 Dans le périmètre (installé et recetté)

| Composant | Où | Ce qui est recetté |
|---|---|---|
| KindMother | `127.0.0.1:50051` | authentification des appelants, chiffrement, étiquette `publiable` (EX-47) |
| MAIA, **sans modèle** | `127.0.0.1:11435` | refus fail-closed, routes mémoire signées, TAMR (EX-50..56) |
| MiyuCloud + PostgreSQL | `127.0.0.1:8086`, `:5432` | chiffrement au repos `MIYUCENC`, CDR, quarantaine, WebDAV, ramasse-miettes |
| Portail web | `127.0.0.1:8090` | page, en-têtes CSP (EX-23) ; aucun service PortalContract raccordé |
| Douane, zone COG | table `inet miyudouane_hote` | aucune sortie des comptes COG (EX-12, EX-20) |
| Portail / sas (`portail-sas`) | binaire installé | refus sans argument ; banc d'intrusion |
| Vigies, relais NFLOG (`miyurelais`) | banc et essai VM | voie aveugle coupée (EX-49) ; **pas** un service installé |
| AppArmor | 5 profils, mode enforce | chargement, confinement, absence de `DENIED` |

### 2.2 Hors périmètre (explicite)

- Central desktop (application de bureau) et Central distant
- Fédération réseau entre COG (aucun second COG, aucun Origin)
- Modèles LLM : MAIA est livrée sans GGUF, aucune réponse de modèle n'est attendue
- Guichet des visas (non implémenté, A-011)
- NFQUEUE, verdicts en ligne (décision d'architecture, A-015)
- Publication de Fiches par le pont (aucune route, A-013 et A-015)
- Services de classe H3 et Origin
- Mise en forme du débit (`tc`), installateur d'OS, sauvegarde hors site (EX-27)

## 3. Prérequis de la machine de recette

<!-- @id recette.plan.prerequis -->
<!-- @role prerequisites -->
<!-- @do list_prerequisites -->

| Élément | Exigence |
|---|---|
| Système | Ubuntu 26.04, noyau avec AppArmor, nftables, Landlock |
| Type | VM jetable (QEMU/KVM ou équivalent), instantané « vierge » pris avant l'installation |
| Ressources | 2 vCPU, 4 Go de RAM, 20 Go de disque, **valeurs indicatives non mesurées** (aucun modèle LLM) |
| Compte | utilisateur `cog` avec `sudo` sans mot de passe ; paquet attendu dans `/home/cog/cog-paquet` |
| Réseau | accès SSH d'administration ; Internet sortant nécessaire **pour le compte `cog` seulement** (contrôle positif de RB-038) |
| Paquets | `postgresql`, `apparmor`, `nftables`, `curl`, `jq`, `openssl`, `python3` ; banc : `iproute2 socat tcpdump nmap` |
| Banc | `br_netfilter` **non chargé** (Docker interdit sur la VM) ; root |
| Livraison | répertoire de paquet (`bin/`, `config/`, `static/`, `banc/`, `MANIFESTE`) + empreinte du `MANIFESTE` transmise **hors du canal du paquet** |

> `pousser.sh` ne vise que la VM `cog-essai` du développeur (`127.0.0.1:2222`). Chez le testeur, le paquet est livré par archive puis installé avec `installer.sh --vm-jetable --bundle … --manifeste-sha256 …`.

## 4. Rôles

<!-- @id recette.plan.roles -->
<!-- @role roles -->
<!-- @do assign_roles -->

| Rôle | Tâches |
|---|---|
| Testeur | exécute les cas, remplit le PV, ouvre les anomalies |
| Mainteneur | construit et étiquette le paquet, transmet l'empreinte, qualifie et corrige les anomalies |
| Propriétaire | valide ou refuse les amendements A-010 à A-015, tranche les écarts, décide GO / NO-GO |
| Relecteur sécurité (facultatif) | relit les journaux du banc et les refus AppArmor |

## 5. Critères d'entrée

<!-- @id recette.plan.entree -->
<!-- @role entry-criteria -->
<!-- @do gate_entry -->

- Paquet construit depuis un arbre **propre** (`# arbre propre` dans le `MANIFESTE`), commit étiqueté `0.9.0-beta.1`
- Côté mainteneur : `tester.sh` (100 contrôles) et `tester.sh --banc` (100 contrôles) sans `not ok` sur la VM de développement ; `scenarios.sh` sans `not ok`
- Empreinte du `MANIFESTE` reçue hors bande
- VM vierge, instantané pris, horloge synchronisée
- [Limites connues](LIMITES-CONNUES-BETA.md) lues et acceptées par le testeur
- Aucune anomalie bloquante connue et non documentée

## 6. Critères de sortie

<!-- @id recette.plan.sortie -->
<!-- @role exit-criteria -->
<!-- @do gate_exit -->

- 100 % des cas **bloquants** (colonne « Gravité d'un échec » = bloquante) passés
- Au moins 95 % des autres cas passés ; chaque échec a une anomalie ouverte
- 0 anomalie bloquante ouverte ; majeures : contournement écrit et accepté par le propriétaire
- Aucun refus AppArmor `DENIED` inexpliqué ; aucune écoute hors boucle locale
- Retour arrière et restauration démontrés (RB-062, RB-067)
- PV signé par le testeur, le mainteneur et le propriétaire

## 7. Procédure

<!-- @id recette.plan.procedure -->
<!-- @role procedure -->
<!-- @do run_acceptance -->

Ordre imposé : chaque phase suppose la précédente passée.

### 7.1 Installation (phase A)

1. Machine vierge (ou instantané) ; noter l'environnement dans le PV
2. Vérifier l'archive (`sha256sum -c cog-….tar.zst.sha256`), l'extraire ; comparer l'empreinte de la clé publique aux notes de version
3. `sudo ./installer.sh --confirme-machine-de-recette --cle-publique-pem <clé publiée>`
4. Relancer l'installateur une seconde fois (idempotence, secrets conservés)
5. Cas : RB-001 à RB-006

### 7.2 Scénarios automatiques (phase B)

1. `/home/cog/cog-paquet/tester.sh` : 100 contrôles TAP attendus `ok`
2. `scenarios.sh --distant` depuis l'hôte du mainteneur (ou `bash scenarios.sh` dans la VM) avec `COG_PROPOSER` fourni pour la section 10 bis
3. `tester.sh --banc` (root, environ 2 min) : 100 contrôles
4. Conserver les sorties TAP complètes en annexe du PV
5. Cas : RB-007 à RB-055 (colonne « Mode » = automatisé)

### 7.3 Scénarios manuels (phase C)

- Cas marqués « manuel » : navigateur par tunnel SSH (`ssh -L 8086:127.0.0.1:8086 -L 8090:127.0.0.1:8090`), `curl`, `systemctl`, `nft`
- Relais NFLOG : `crates/miyurelais/tests/vm/essai_nflog.sh` avec `essai_vm` et `miyurelais` fournis par le mainteneur
- Tests de bibliothèque (étiquette `publiable`) : exécutés par le mainteneur, résultats joints

### 7.4 Redémarrage (phase D)

- Redémarrage de la VM puis `tester.sh` ; redémarrage unitaire ; coupure brutale
- Cas : RB-056 à RB-060

### 7.5 Sauvegarde et restauration (phase E)

- `sudo /opt/miyukini/exploitation/sauvegarder.sh --phrase-fichier <fichier 0600>` : `pg_dump`, `/etc/miyukini`, `/var/lib/miyukini`, code installé ; archive chiffrée relue
- `restaurer.sh --archive … --phrase-fichier … --confirme-restauration` (ajouter `--avec-code` pour revenir au code sauvegardé)
- Copier l'archive hors de la machine
- Cas : RB-061 à RB-063

### 7.6 Mise à jour et retour arrière (phase F)

- `sudo /opt/miyukini/exploitation/mettre-a-jour.sh --paquet <nouveau .tar.zst> --cle-publique-pem <clé> --confirme-machine-de-recette`
- Paquet altéré ou mauvaise clé : refus **avant** toute sauvegarde ; sinon sauvegarde préalable automatique, puis retour arrière automatique si le contrôle de santé échoue (code 3)
- Essai complet : `sudo tests/mise-a-jour.sh --paquet-maj <paquet B> --cle-publique-pem <clé> --confirme-machine-de-recette` (refus, retour arrière forcé, réussite)
- Cas : RB-064 à RB-067

### 7.7 Désinstallation (phase G)

- `sudo /opt/miyukini/exploitation/desinstaller.sh` (données conservées) ou `--purger --confirme-purge` ; ou retour à l'instantané vierge
- Dernière phase, car elle détruit l'environnement

## 8. Gestion des anomalies

<!-- @id recette.plan.anomalies -->
<!-- @role defect-management -->
<!-- @do triage_defects -->

| Gravité | Définition | Effet sur la décision |
|---|---|---|
| Bloquante | une garantie de sécurité n'est pas tenue (sortie COG possible, refus fail-closed contourné, secret en clair, écoute externe), perte de données, service qui ne démarre pas | NO-GO tant que non corrigée |
| Majeure | fonction du périmètre inutilisable ou incohérente, avec contournement | GO possible si contournement écrit et accepté par le propriétaire |
| Mineure | défaut d'affichage, message, documentation, sans effet sur la sécurité ni les données | GO ; corrigée dans une version ultérieure |

- Identifiant `AN-NNN` ; champs : cas `RB-NNN`, gravité proposée, étapes, résultat obtenu, journaux (`journalctl -u …`, sortie TAP), version
- Qualification : le testeur propose, le mainteneur confirme sous 2 jours ouvrés, le propriétaire tranche en cas de désaccord
- Un `not ok` d'un contrôle de sécurité de `tester.sh` ou du banc est **bloquant** jusqu'à preuve contraire
- Écart déjà listé dans les [limites connues](LIMITES-CONNUES-BETA.md) : pas d'anomalie, mentionné au PV comme « limite attendue »
- Défaut connu à constater, non à corriger : reconnexion de MAIA à KindMother (RB-059)
- Correction : nouveau paquet `0.9.0-beta.N`, rejeu des cas touchés et du lot de non-régression (RB-007 à RB-012, RB-037, RB-042, RB-047)
