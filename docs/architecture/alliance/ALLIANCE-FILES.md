# Alliance-Files — synchronisation des fichiers dans l'Alliance

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.files -->
<!-- @role design-file-sync -->
<!-- @layer 5-7 -->
<!-- @human Service de synchronisation des fichiers entre COG d'une Alliance, droits gérés par Alliance-Admin -->
<!-- @do sync_alliance_files -->

## Contexte

- Les données d'un utilisateur restent sur son poste (AL-D23) ; une équipe a pourtant besoin de travailler sur les mêmes fichiers, depuis des postes différents
- Une synchronisation entre postes est aussi le vecteur classique de propagation d'un rançongiciel : ce qui se synchronise vite se chiffre vite partout

## Portée / Scope

- Espaces synchronisés, droits, transport, contrôle des fichiers reçus, versions, défense contre la propagation
- Hors périmètre : compatibilité Windows (partages SMB), traitée avec [Alliance-Admin §4](ALLIANCE-ADMIN.md) après AL-D07

## 1. Ce qui est décidé

<!-- @id arch.alliance.files-decide -->
<!-- @role owner-decisions -->
<!-- @do record_files_decisions -->

- **Décidé (propriétaire, AL-D28)**
  - Nouveau service **Alliance-Files** : **synchronisation des fichiers entre COG** de l'Alliance
  - **Droits** gérés en fonction d'**Alliance-Admin**

## 2. Principes

<!-- @id arch.alliance.files-principes -->
<!-- @role principles -->
<!-- @do bound_file_sync -->

- **Proposé**
  - **Espace synchronisé** : seul ce qu'un utilisateur place **explicitement** dans un espace est synchronisé ; le reste de ses fichiers reste sur son poste (AL-D23 intact)
  - Un espace a un **propriétaire** (compte ou groupe) et des **droits** tirés d'Alliance-Admin : lecture, écriture, partage ; aucun droit propre au service
  - **Jamais synchronisé** : mémoire MAIA (AL-D24, AL-D25), secrets, clés, bases KindMother
  - Distinct de **MiyuCloud** (cloud d'un COG, façades H2 vers l'extérieur) : Alliance-Files reste **dans** l'Alliance ; un fichier n'en sort que par la COG-Gateway et ses règles

## 3. Architecture

<!-- @id arch.alliance.files-architecture -->
<!-- @role architecture -->
<!-- @do place_file_sync -->

- **Décidé (AL-D29)**
  - **Le COG-CC** tient l'**index** des espaces : membres, droits, et pour chaque fichier la liste de ses versions (empreinte, auteur, date) dans un journal Merkle ; il ne stocke **pas** le contenu
  - **Le contenu** circule de COG à COG, par des **voies déclarées** des Portails (la zone COG tire, D4) ; un poste nomade passe par la COG-Gateway (AL-D13)
  - **COG de stockage** facultatifs, désignés par politique : gardent une copie de chaque espace pour qu'un fichier reste disponible quand les postes de l'équipe sont éteints ; ils servent aussi de sauvegarde des fichiers, y compris des fichiers hors espace, chiffrés pour le compte (AL-D41, [registre](ALLIANCE-DECISIONS.md)) ; une Alliance qui veut des sauvegardes en désigne donc au moins un
- **Proposé** (précisions)
  - **Chiffrement** : une **clé par espace**, remise aux comptes autorisés ; un COG de stockage garde des données **chiffrées** qu'il ne lit pas
  - **Signature** : chaque version est signée par la **clé d'appareil de son auteur** ; une version non signée par un membre autorisé à écrire est refusée

## 4. Contrôle des fichiers reçus

<!-- @id arch.alliance.files-controle -->
<!-- @role inbound-control -->
<!-- @do check_synced_files -->

- **Constat** : le CDR v1 (A-004) refuse PDF, bureautique, archives et exécutables ; appliqué à toute synchronisation interne, il rendrait le service inutilisable pour le travail courant
- **Décidé (AL-D30) — contrôle selon la provenance**
  - Chaque fichier porte une **marque de provenance** : **interne** (créé dans l'Alliance par un auteur signataire) ou **externe** (entré par la COG-Gateway, une clé USB, une Alliance liée)
  - **Externe** : CDR à l'entrée, non pris en charge ⇒ quarantaine (règle v1) ; la marque **suit** le fichier, on ne la retire pas en le recopiant dans un espace
  - **Interne** : accepté sur signature d'un auteur autorisé ; analyse sur le poste qui reçoit (Vigies, signatures connues) ; jamais **ouvert** ni exécuté par un composant du COG
  - Types exécutables (binaires, scripts, macros) : **refusés** dans tout espace, sauf espace dédié ouvert par politique de classe 3

## 5. Versions, conflits, propagation

<!-- @id arch.alliance.files-versions -->
<!-- @role versioning -->
<!-- @do protect_against_mass_change -->

- **Proposé**
  - **Historique** : chaque version est gardée (durée par politique) ; une version ne remplace jamais silencieusement la précédente
  - **Conflit** (deux auteurs modifient la même version) : les deux versions sont gardées, l'utilisateur choisit ; aucune fusion automatique
  - **Défense contre la propagation** (rançongiciel, effacement de masse) :
    - modifications ou suppressions **en masse** d'un même auteur ou appareil au-delà d'un seuil, ou contenus devenus soudain **illisibles** (entropie élevée) ⇒ synchronisation de cet auteur **suspendue** dans tous les espaces, signal à WorrySentinel, procédure HECTOR P-005
    - les autres postes **ne reçoivent pas** les versions suspectes ; restauration par l'historique
    - reprise sur **autorisation explicite** (classe 2), comme le réarmement d'une voie v1
  - Suppression = version « supprimée » dans l'historique ; effacement réel seulement à l'échéance de conservation

## 6. Droits retirés et départs

<!-- @id arch.alliance.files-retrait -->
<!-- @role revocation -->
<!-- @do revoke_space_access -->

- **Décidé (AL-D31)**
  - Retrait d'un compte d'un groupe (Alliance-Admin) ⇒ plus aucune synchronisation pour lui ; **nouvelle clé d'espace** pour les versions suivantes
  - Les copies déjà sur son poste : **jamais effacées à distance** (INV-Z11) ; rendues illisibles hors de son compte (chiffrées), et l'organisation, propriétaire de la racine du poste (AL-D06), traite le poste par cérémonie locale si nécessaire

## 7. Points à trancher

- Aucun point ouvert de conception ; à l'implémentation : seuils de la détection de masse, durée de conservation des versions (par politique), format de la marque de provenance

---

Date de mise a jour : 2026-10-25
