# Dépôt README public

## 📍 Dépôt public séparé

Le README de ce projet est également disponible dans un **dépôt public séparé** pour permettre la découverte du projet sans exposer le code source.

**URL du dépôt public** : `https://github.com/StudioMiyukini/miyukini-core-system-public`

**Emplacement local** : `c:\Users\miyuk\Documents\Cursor\miyukini-core-system-readme`

---

## 🔄 Synchronisation

### Méthode rapide (script)

```powershell
cd "c:\Users\miyuk\Documents\Cursor\miyukini-core-system-readme"
.\sync-readme.ps1
```

### Méthode manuelle

```powershell
cd "c:\Users\miyuk\Documents\Cursor\miyukini-core-system-readme"
Copy-Item "..\miyukini_kernel\README.md" . -Force
git add README.md
git commit -m "Update README from main repository"
git push
```

---

## 📚 Documentation complète

Pour plus de détails sur l'accès et la synchronisation, voir :
- **Dans le dépôt README public** : `ACCES_ET_SYNCHRONISATION.md`
- **Script de synchronisation** : `sync-readme.ps1`

---

**Note** : Ce dépôt principal reste **privé**. Seul le README est rendu public via le dépôt séparé.
