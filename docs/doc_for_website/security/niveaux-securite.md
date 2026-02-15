# Niveaux de Sécurité

## Les 5 Niveaux (0 à 4)

Les données et ressources sont classées en **5 niveaux de sécurité**. Chaque niveau impose des contraintes de stockage, d'accès et de transmission.

| Niveau | Nom | Description | Exemples |
|--------|-----|-------------|----------|
| **0** | Public | Accessible sans authentification | Préférences UI, contenu public |
| **1** | Standard | Authentification requise | Profil utilisateur, préférences |
| **2** | Sensible | Chiffrement + accès restreint | Documents métier, factures |
| **3** | Critique | Accès minimal, audit strict | Données bancaires, secrets |
| **4** | Maximum | Protection maximale | Clés, credentials |

## Application

- **KindMother** : Chiffrement au repos selon le niveau (SQLCipher pour 2–4).
- **TAMR** : Autorisation selon le niveau et le rôle.
- **BorderGuard** : Contrôle des flux selon le niveau.
- **Services** : Chaque service déclare le niveau de ses données.

## Références

Security - Liste des Mesures ; Documentation des services (niveaux par service).
