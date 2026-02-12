# Miyukini COG — Mode d'emploi

Guide d’installation et de prise en main de la **distribution publique** de Miyukini COG (sans code source).

---

## 1. Prérequis

- **Windows** : aucune dépendance supplémentaire pour lancer les exécutables fournis.
- Les binaires sont compilés pour Windows ; pour d’autres OS, consulter le projet source ou les releases.

---

## 2. Installation

1. Télécharger ou cloner cette distribution (dépôt public).
2. Conserver l’arborescence : le dossier `bin/` doit contenir les exécutables, `docs/` la documentation.

Aucune installation “installeur” n’est nécessaire : il s’agit d’une distribution portable.

---

## 3. Premier lancement

### Option A — Hub seul (sans persistance)

- Double-cliquer sur **`bin\miyukini-central.exe`** (ou lancer depuis un terminal).
- Miyukini Central s’ouvre : catalogue des services, onglets, thème clair/sombre.
- Les jeux (MiyukiniClicker, MiyukiniSurvivor, LifeGame) et la navigation dans le Hub fonctionnent sans serveur.

### Option B — Avec persistance des données (services Jay)

Pour enregistrer les données des services (JayFestival, JayKoa, JayKonta, JayXpose, etc.) :

1. Lancer d’abord **`bin\kindmother-server.exe`** (serveur de données local). Laisser la fenêtre ouverte ou l’exécuter en arrière-plan.
2. Ensuite lancer **`bin\miyukini-central.exe`**.

Sans KindMother, les services “Jay” ne pourront pas persister les données entre les sessions.

---

## 4. Utilisation du Hub (Miyukini Central)

- **Accueil** : grille des services disponibles (JayFestival, JayKoa, JayKonta, JayXpose, jeux, etc.).
- **Onglets** : chaque service ouvert reste en mémoire ; on peut passer d’un onglet à l’autre.
- **Profil / Paramètres** : accessibles depuis le Hub (thème, paramètres utilisateur).
- **Connexion / Rite d’entrée** : au premier lancement, un écran de connexion ou d’onboarding peut s’afficher selon la configuration.

---

## 5. Services disponibles

| Service | Description |
|---------|-------------|
| **JayFestival** | Gestion d’événements et festivals (organisateurs, exposants, visiteurs). |
| **JayKoa** | Calendrier universel (agenda, synchronisation). |
| **JayKonta** | Comptabilité et budget (bourse, mouvements, prévisions). |
| **JayXpose** | Profil exposant et vitrine (catalogue, documents). |
| **JayShop** | Boutique en ligne (en cours). |
| **MiyukiniClicker** | Jeu idle/clicker + stratégie. |
| **MiyukiniSurvivor** | Jeu Survivor / Tower Defense. |
| **Miyukini LifeGame** | Simulation de monde (entités, pouvoirs). |

---

## 6. Où trouver la documentation

Toute la documentation du projet est dans le dossier **`docs/`** :

- **docs/public/** — Documents de référence (architecture, COG, lois d’autonomie, glossaire) en FR et EN.
- **docs/services/** — Documentation par service (fondateur, guides, parcours).
- **docs/reference/** — Références conceptuelles et glossaire étendu.
- **docs/contrats/** — Protocoles et contrats (MIP, implémentation, documentation).
- **docs/cores/**, **docs/tools/**, etc. — Documentation des Cores et Toolkits.

Aucun code source n’est fourni dans cette distribution ; seuls les binaires et la documentation sont inclus.

---

## 7. Dépannage

- **L’application ne démarre pas** : vérifier que les exécutables sont bien dans `bin/` et qu’aucun antivirus ne les bloque.
- **Les données des services ne sont pas sauvegardées** : s’assurer que `kindmother-server.exe` a été lancé et reste actif.
- **Erreur de connexion au serveur** : KindMother écoute en local (localhost) ; ne pas fermer le serveur pendant l’utilisation des services qui en dépendent.

---

## 8. Licence

Usage personnel et domestique gratuit. Usage par une société ou une collectivité : voir [LICENSE](LICENSE) et la politique de licence du projet Miyukini.
