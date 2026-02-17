# Miyukini COG — Audit écarts d’implémentation

**Règle :** Ce document ne contient **que ce qui n’est pas implémenté à 100 %**. Dès qu’un sujet est **100 % implémenté et testé**, supprimer sa section ou sa ligne. Le document ne doit afficher que ce qu’il reste à faire.

**Dernière mise à jour :** 2026-02-16

---

## Comment utiliser ce document


1. **Travailler** sur un sujet de la liste (toolkit, application, intégration).
2. **Implémenter** jusqu’à couvrir 100 % des points listés pour ce sujet.
3. **Tester** (tests unitaires et/ou d’intégration selon le projet).
4. **Supprimer** la section ou la ligne correspondante dans ce fichier.
5. Ne pas ajouter de sujets déjà 100 % faits ; ce document est un **reste-à-faire** uniquement.

---

## 1. Applications

| Application      | Écart | Guide d’implémentation |
|------------------|-------|------------------------|
| **apps/ui-builder** | Doc minimale ou fonctionnalités à compléter selon scope produit | Décider si ui-builder est « doc seule » ou « app à livrer » ; si app : ajouter écrans / build, puis tests ; une fois 100 %, retirer cette ligne. |

---

## 2. Toolkits — retours `Unimplemented` à traiter

Aucun toolkit ne retourne encore `Err(…Error::Unimplemented)` dans le code (vérification 2026-02-16). Les blocs ci-dessous documentent la vérification.

### 2.1 Autres toolkits avec variante `Unimplemented` (erreurs uniquement)

Les crates suivants exposent une variante `Unimplemented` dans leur enum d’erreur et peuvent encore l’utiliser dans certains chemins. À vérifier au cas par cas ; dès que plus aucun retour `Unimplemented` n’existe et que les tests couvrent le sujet, **supprimer la ligne**.

(Vérification 2026-02-16 : aucun retour Unimplemented en code ; MiyuBilling et MiyuLocale corrigés.)

---

## 3. Intégrations services (optionnel)

À ne lister **que** si une intégration est explicitement manquante et bloquante. Dès qu’elle est livrée et testée, supprimer la ligne.

| Intégration | Écart | Guide |
|--------------|--------|--------|
| JayRDV → MiyuBooking (créneaux) | À brancher si besoin | Utiliser `miyubooking` pour créer/lister créneaux depuis JayRDV. |
| JayRDV → MiyuNotify (rappels) | À brancher si besoin | Appeler `miyunotify` pour envoi rappels RDV. |
| MiyukiniSales → MiyuInvoice / JayKonta | À brancher si besoin | Facturation et comptabilité depuis commandes. |

---

## 4. Priorisation suggérée

- **Basse** : Section 1 (apps/ui-builder), Section 3 (intégrations optionnelles).

---

## 5. Références

- **Reference Implementation Guidelines** : `docs/tools/Miyu<Nom>/implementation/`.
- **Contrats gouvernance** : `docs/tools/Miyu<Nom>/contracts/governance/`.
- **Suivi global** : `docs/implementation/Miyukini COG - Suivi Audit et Todo.md` (si présent).
- **MIP/MSCM** : `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`.
