# Miyukini COG — Distribution publique

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

Cette distribution contient une **version fonctionnelle** de Miyukini COG : binaires et documentation complète, **sans code source**. Elle est destinée aux utilisateurs qui veulent exécuter l’application et consulter toute la documentation du projet.

---

## Contenu de la distribution

| Élément | Description |
|--------|-------------|
| **bin/** | Exécutables : Miyukini Central (Hub) et KindMother (serveur de données, optionnel). |
| **docs/** | Documentation complète du projet (conceptuelle, technique, services, références). |
| **docs/legal/** | Documents juridiques : politique de licence, licence pro service-tier (validation Relay, tarifs 2026). |
| **MODE_EMPLOI.md** | Installation, premier lancement et prise en main. |
| **LICENSE** | Licence d’utilisation (personnelle et domestique). |

Aucun code source (crates, apps) n’est inclus.

---

## Démarrage rapide

1. **Windows** : lancer `bin\miyukini-central.exe` pour ouvrir le Hub Miyukini Central.
2. Pour une utilisation complète (persistance des données des services), démarrer aussi `bin\kindmother-server.exe` avant ou en arrière-plan — voir [MODE_EMPLOI.md](MODE_EMPLOI.md).
3. **Documentation** : tout est dans le dossier `docs/` (architecture, services, guides).

---

## Documentation

- **Mode d’emploi** (installation, premier lancement, services) : [MODE_EMPLOI.md](MODE_EMPLOI.md)
- **Documentation conceptuelle** : `docs/public/` (FR et EN)
- **Documentation des services** : `docs/services/`
- **Références et contrats** : `docs/reference/`, `docs/contrats/`
- **Documents juridiques** : `docs/legal/` (politique de licence, licence pro service-tier)

---

## Licence

Voir [LICENSE](LICENSE). Usage personnel et domestique gratuit ; usage par une société ou une collectivité soumis à une licence commerciale. Pour proposer un service-tier aux COGs (validation Relay, mise en ligne) : voir [docs/legal/](docs/legal/) (Licence Pro Service-Tier — tarifs 2026, gratuité sur demande pour non-lucratif et intérêt général).
