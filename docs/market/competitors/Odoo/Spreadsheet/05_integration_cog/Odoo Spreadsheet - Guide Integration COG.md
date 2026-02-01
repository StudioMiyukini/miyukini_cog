# Odoo Spreadsheet — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Spreadsheet dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création classeur, source de données, formules, partage)
- Gestion des gouvernances

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
SpreadsheetUI → BondingBrother → SpreadsheetOperator → StrongFather (décision)
                                                       → KindMother (WriteIntent)
                                                       → Master Butler (permissions)
                                                       → WorrySentinel (sécurité)
                ↓
                SpreadsheetDataSourceOperator → Opérateurs métier (vues)
                                              → KindMother (lecture)
                                              → WorrySentinel (niveau sécurité)
```

### 1.2 Flux Typique

1. **Intention utilisateur** → SpreadsheetUI
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather (si création/modification/partage)
4. **Vérification permissions** → Master Butler
5. **Vérification sécurité** → WorrySentinel
6. **Persistance** → KindMother (WriteIntent pour classeur, sources, templates, versions)
7. **Rafraîchissement données** → SpreadsheetDataSourceOperator → KindMother (lecture) + Opérateurs métier

---

## 2. Patterns d'Intégration

### 2.1 Création d'un Classeur

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateSpreadsheetIntent {
    pub name: String,
    pub folder_id: Option<Uuid>,
    pub locale: LocaleId,
    pub from_template_id: Option<Uuid>,
}

impl SpreadsheetOperator {
    pub async fn create_spreadsheet(
        &self,
        intent: CreateSpreadsheetIntent,
        mandate: Mandate,
    ) -> Result<Spreadsheet, SpreadsheetError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["spreadsheet.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_spreadsheet",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(SpreadsheetError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "spreadsheet.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(SpreadsheetError::PermissionDenied);
        }

        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;
        if security_level > mandate.max_security_level {
            return Err(SpreadsheetError::SecurityLevelExceeded);
        }

        let content = if let Some(template_id) = intent.from_template_id {
            self.template_operator.get_content(template_id).await?
        } else {
            SpreadsheetContent::default()
        };

        let write_intent = WriteIntent {
            entity_type: "spreadsheet.spreadsheet",
            operation: WriteOperation::Create,
            data: SpreadsheetData {
                name: intent.name,
                folder_id: intent.folder_id,
                locale: intent.locale,
                content,
                company_id: self.get_company_id().await?,
            },
            security_level,
        };

        let spreadsheet = self.kind_mother
            .persist(write_intent)
            .await?;
        Ok(spreadsheet)
    }
}
```

### 2.2 Insertion d'une Source de Données (Liste)

**Pattern :** Mandate + exposition vue par l'Opérateur métier

```rust
pub struct InsertListDataSourceIntent {
    pub spreadsheet_id: Uuid,
    pub sheet_name: String,
    pub source_operator_id: OperatorId,  // ex. MiyuSales
    pub view_name: String,              // ex. "sale.order.list"
    pub domain: Vec<DomainTerm>,
    pub sort: Vec<SortSpec>,
    pub columns: Vec<String>,
    pub row_count: u32,
}

impl SpreadsheetDataSourceOperator {
    pub async fn insert_list(
        &self,
        intent: InsertListDataSourceIntent,
        mandate: Mandate,
    ) -> Result<DataSource, SpreadsheetError> {
        mandate.validate_flows(&["datasource.create"])?;
        mandate.validate_operators(&[intent.source_operator_id])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "insert_list_datasource",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(SpreadsheetError::DecisionDenied);
        }

        // Vérifier que l'Opérateur métier expose bien cette vue
        let view_contract = self.bonding_brother
            .request_view(intent.source_operator_id, &intent.view_name, &mandate)
            .await?;
        if !view_contract.allows_list {
            return Err(SpreadsheetError::ViewNotAvailable);
        }

        let data_source = DataSource {
            id: DataSourceId::new(),
            spreadsheet_id: intent.spreadsheet_id,
            type_: DataSourceType::List,
            list_id: self.next_list_id(intent.spreadsheet_id).await?,
            model: view_contract.model.clone(),
            domain: intent.domain,
            sort: intent.sort,
            columns: intent.columns,
            row_count: intent.row_count,
        };

        let write_intent = WriteIntent {
            entity_type: "spreadsheet.datasource",
            operation: WriteOperation::Create,
            data: data_source.clone(),
            security_level: mandate.max_security_level,
        };
        self.kind_mother.persist(write_intent).await?;
        Ok(data_source)
    }
}
```

### 2.3 Résolution de Formule ODOO.LIST

**Pattern :** Lecture gouvernée + Mandate

```rust
pub async fn resolve_odoo_list(
    &self,
    spreadsheet_id: Uuid,
    list_id: u32,
    index: u32,
    field_name: &str,
    mandate: Mandate,
) -> Result<Value, SpreadsheetError> {
    let datasource = self.get_list_datasource(spreadsheet_id, list_id).await?;
    let security_level = self.worry_sentinel
        .get_security_level_for_model(&datasource.model)
        .await?;
    if security_level > mandate.max_security_level {
        return Err(SpreadsheetError::SecurityLevelExceeded);
    }

    let read_intent = ReadIntent {
        entity_type: datasource.model.clone(),
        domain: datasource.domain_with_global_filters(spreadsheet_id).await?,
        sort: datasource.sort.clone(),
        fields: vec![field_name.to_string()],
        limit: 1,
        offset: index.saturating_sub(1),
        security_level,
    };
    let rows = self.kind_mother.query(read_intent).await?;
    let row = rows.into_iter().next();
    Ok(row.and_then(|r| r.get(field_name).cloned()).unwrap_or(Value::Null))
}
```

### 2.4 Partage (Viewer / Editor)

**Pattern :** StrongFather + Documents (droits)

```rust
pub struct ShareSpreadsheetIntent {
    pub spreadsheet_id: Uuid,
    pub partner_ids: Vec<PartnerId>,
    pub access: AccessLevel,  // Viewer | Editor
    pub freeze_and_share: bool,
}

impl SpreadsheetOperator {
    pub async fn share(
        &self,
        intent: ShareSpreadsheetIntent,
        mandate: Mandate,
    ) -> Result<(), SpreadsheetError> {
        mandate.validate_flows(&["spreadsheet.share"])?;
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "share_spreadsheet",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(SpreadsheetError::DecisionDenied);
        }
        self.documents_operator
            .grant_access(
                intent.spreadsheet_id,
                intent.partner_ids,
                intent.access,
                intent.freeze_and_share,
                mandate,
            )
            .await?;
        Ok(())
    }
}
```

### 2.5 Restauration d'une Version

**Pattern :** WriteIntent (remplacement contenu) + StrongFather

```rust
pub async fn restore_version(
    &self,
    spreadsheet_id: Uuid,
    version_id: Uuid,
    mandate: Mandate,
) -> Result<(), SpreadsheetError> {
    mandate.validate_flows(&["version.restore"])?;
    let decision = self.strong_father
        .decide(DecisionRequest {
            action: "restore_spreadsheet_version",
            context: &RestoreVersionContext { spreadsheet_id, version_id },
        })
        .await?;
    if !decision.allowed {
        return Err(SpreadsheetError::DecisionDenied);
    }
    let snapshot = self.version_operator.get_snapshot(version_id).await?;
    let write_intent = WriteIntent {
        entity_type: "spreadsheet.spreadsheet",
        operation: WriteOperation::Update,
        data: UpdateSpreadsheetContent {
            spreadsheet_id,
            content: snapshot.content,
        },
        security_level: mandate.max_security_level,
    };
    self.kind_mother.persist(write_intent).await?;
    Ok(())
}
```

---

## 3. Gestion des Gouvernances

### 3.1 Rafraîchissement Global

- Pour chaque source du classeur : vérifier Mandate + niveau sécurité (WorrySentinel) ; puis KindMother (lecture) ou Opérateur métier (vue).
- Si une source échoue (permission, modèle supprimé) : signaler l’erreur pour cette source, continuer les autres si possible.

### 3.2 Export .xlsx

- Lecture du contenu (KindMother) + résolution des formules « métier » (ODOO.LIST, PIVOT, fonctions Odoo-like) en valeurs.
- Pas d’écriture métier ; export = fichier binaire généré côté serveur avec Mandat « spreadsheet.export_xlsx ».

### 3.3 Filtres Globaux

- Stockés dans le classeur (KindMother) ; à chaque rafraîchissement, le domain effectif de chaque source = domain ∪ critères des filtres globaux.
- Pas de décision StrongFather par filtre ; décision déjà couverte par le Mandat d’édition du classeur.

---

**Document créé le :** 2026-02-01
