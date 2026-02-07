# MiyukiniAdmin — Serveur HTTP / HTTPS

## 1. Contexte

MiyukiniAdmin expose une interface web (dashboard, base de données, tests, etc.) servie par un **serveur HTTP**. Une **option HTTPS** permet de servir l’interface en TLS pour un affichage sécurisé (sans exposition d’API publique vers l’extérieur — usage interne / admin).

Ce document décrit le mode de fonctionnement du serveur et la configuration HTTP/HTTPS.

---

## 2. Portée / Scope

Ce document couvre :
- Rôle du serveur (affichage de l’interface admin).
- Configuration HTTP (hôte, port).
- Configuration HTTPS optionnelle (certificat, clé, port dédié éventuel).
- Variables d’environnement et bonnes pratiques.

Ce document **ne couvre pas** :
- L’exposition d’API publique (interdite par invariant INV-MA-3).
- Les protocoles applicatifs entre MiyukiniAdmin et les cores (voir contrats d’intégration).

---

## 3. Rôle du serveur

- **Servir l’interface utilisateur** : pages HTML du dashboard, Database, Tests, etc.
- **Exposer des endpoints internes** : `/health`, `/api/status`, `/api/tables`, etc., utilisés par l’UI elle-même ou par des outils de monitoring internes.
- **Ne pas exposer d’API publique** : le serveur est destiné à l’administration interne, pas à des clients B2B/B2C.

---

## 4. Mode HTTP

### 4.1 Comportement par défaut

- Le serveur écoute en **HTTP** sur une adresse et un port configurables.
- **Variables d’environnement :**
  - `MIYUKINIADMIN_HOST` : adresse d’écoute (défaut : `127.0.0.1`).
  - `MIYUKINIADMIN_PORT` : port (défaut : `8181`).

### 4.2 Accès

- URL typique : `http://127.0.0.1:8080/` ou `http://localhost:8080/`.
- En production, restreindre l’accès (pare-feu, VPN, reverse proxy) pour ne pas exposer l’admin sur Internet.

---

## 5. Mode HTTPS (optionnel)

### 5.1 Objectif

- Chiffrement TLS entre le navigateur et MiyukiniAdmin.
- Utile lorsque l’admin est utilisé sur un réseau où le trafic doit être protégé (ex. accès distant sur réseau interne).

### 5.2 Configuration

- **Variables d’environnement :**
  - `MIYUKINIADMIN_HTTPS` : activer HTTPS (`1` ou `true`).
  - `MIYUKINIADMIN_TLS_CERT` : chemin vers le fichier certificat (PEM).
  - `MIYUKINIADMIN_TLS_KEY` : chemin vers le fichier clé privée (PEM).

- Si `MIYUKINIADMIN_HTTPS` est activé et que certificat/clé sont valides, le serveur écoute en **HTTPS** sur le même port (ou sur un port dédié selon implémentation). Sinon, le serveur peut démarrer en HTTP uniquement ou refuser de démarrer (selon politique d’implémentation).

### 5.3 Bonnes pratiques

- Utiliser des certificats émis par une CA interne ou un outil (ex. `mkcert`) pour le développement.
- Ne pas commiter de certificats ou clés dans le dépôt.
- En production, placer certificat et clé dans un emplacement sécurisé, lisible uniquement par le processus MiyukiniAdmin.

### 5.4 Accès HTTPS

- URL typique : `https://127.0.0.1:8443/` ou `https://localhost:8443/` (si port 8443 utilisé pour HTTPS).
- Le navigateur peut afficher un avertissement si le certificat n’est pas signé par une CA reconnue ; attendu en environnement de dev ou interne.

---

## 6. Résumé des variables

| Variable | Défaut | Description |
|----------|--------|-------------|
| `MIYUKINIADMIN_HOST` | `127.0.0.1` | Adresse d’écoute |
| `MIYUKINIADMIN_PORT` | `8181` | Port d’écoute (HTTP ou HTTPS selon config) |
| `MIYUKINIADMIN_HTTPS` | — | `1` ou `true` pour activer HTTPS |
| `MIYUKINIADMIN_TLS_CERT` | — | Chemin vers le certificat PEM |
| `MIYUKINIADMIN_TLS_KEY` | — | Chemin vers la clé privée PEM |

---

## 7. Documents associés

- [MiyukiniAdmin - Index de Navigation](../_index.md)
- [MiyukiniAdmin - Capacités et Référence](../reference/MiyukiniAdmin%20-%20Capacites%20et%20Reference.md)
- [MiyukiniAdmin - Installation & Bootstrap Guide](../foundation/MiyukiniAdmin%20-%20Installation%20&%20Bootstrap%20Guide.md)
- [MiyukiniAdmin - Threat Model Contract](../contracts/security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)

---

**Date de création :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de référence
