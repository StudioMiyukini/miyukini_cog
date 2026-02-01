# Odoo Forum — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Forum dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Forum
- Patterns WriteIntent et Mandates (post, karma, modération)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel, TAMR)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
ForumUI → BondingBrother → PostOperator / ForumOperator / ModerationOperator
                                    → StrongFather (décision)
                                    → KindMother (WriteIntent)
                                    → Master Butler (permissions / karma)
                                    → WorrySentinel (sécurité / modération)
                                    → KarmaOperator (seuils, gains/pertes)
```

### 1.2 Flux Typiques

**Création d’un post (question) :**
1. Intention utilisateur (ForumUI) → BondingBrother
2. PostOperator reçoit la demande
3. StrongFather : décision de créer (autorisation)
4. Master Butler + KarmaOperator : vérification droits (ask questions, seuil karma)
5. WorrySentinel : niveau sécurité contenu
6. KindMother : WriteIntent (création post)
7. KarmaOperator : gain karma (+2 ask question) si publié
8. MiyuNotify : notification followers (si suivi)

**Vote sur un post :**
1. Intention vote (up/down) → PostOperator
2. KarmaOperator : vérification seuil (upvote 5, downvote 50)
3. KindMother : WriteIntent (enregistrement vote)
4. KarmaOperator : gain/perte karma pour l’auteur du post (+5/-2 question, +10/-2 answer)

**Modération (marquer offensif) :**
1. ModerationOperator reçoit l’action (flag confirmé, raison)
2. StrongFather : décision de confirmer offensif
3. KindMother : WriteIntent (état post : masqué, raison)
4. KarmaOperator : -100 karma auteur
5. TAMR : traçabilité intervention humaine

---

## 2. Patterns d'Intégration

### 2.1 Création d’un post (question)

**Pattern :** WriteIntent + Mandate + Karma check

```rust
// Pseudo-code Rust
pub struct CreatePostIntent {
    pub forum_id: Uuid,
    pub title: String,
    pub description: String,
    pub tag_ids: Vec<Uuid>,
    pub post_type: PostType, // Question | Answer | Comment
    pub parent_post_id: Option<Uuid>,
}

impl PostOperator {
    pub async fn create_post(
        &self,
        intent: CreatePostIntent,
        mandate: Mandate,
    ) -> Result<Post, PostError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["post.create"])?;

        let karma = self.karma_operator.get_user_karma(mandate.user_id()).await?;
        let rights = self.karma_operator.check_rights(karma, intent.forum_id).await?;

        match intent.post_type {
            PostType::Question if !rights.can_ask_questions => return Err(PostError::InsufficientKarma),
            PostType::Answer if !rights.can_answer => return Err(PostError::InsufficientKarma),
            PostType::Comment if !rights.can_comment => return Err(PostError::InsufficientKarma),
            _ => {}
        }

        let decision = self.strong_father
            .decide(DecisionRequest { action: "create_post", context: &intent })
            .await?;
        if !decision.allowed {
            return Err(PostError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "forum.post",
            operation: WriteOperation::Create,
            data: PostData {
                forum_id: intent.forum_id,
                title: intent.title,
                description: intent.description,
                tag_ids: intent.tag_ids,
                post_type: intent.post_type,
                parent_post_id: intent.parent_post_id,
                author_id: mandate.user_id(),
                state: if rights.ask_without_validation { PostState::Published } else { PostState::Pending },
            },
            security_level: 2,
        };

        let post = self.kind_mother.execute(write_intent).await?;

        if post.state == PostState::Published {
            self.karma_operator.apply_gain(mandate.user_id(), KarmaGain::AskQuestion, post.forum_id).await?;
        }
        Ok(post)
    }
}
```

### 2.2 Marquer une réponse comme « best answer »

**Pattern :** WriteIntent + Mandate + Karma (accept answer)

```rust
pub async fn set_best_answer(
    &self,
    post_id: Uuid,
    answer_post_id: Uuid,
    mandate: Mandate,
) -> Result<(), PostError> {
    mandate.validate_flows(&["post.best_answer"])?;

    let post = self.get_post(post_id).await?;
    let karma = self.karma_operator.get_user_karma(mandate.user_id()).await?;
    let can_accept = post.author_id == mandate.user_id()
        ? self.karma_operator.can_accept_on_own(karma)
        : self.karma_operator.can_accept_on_all(karma);

    if !can_accept {
        return Err(PostError::InsufficientKarma);
    }

    let decision = self.strong_father
        .decide(DecisionRequest { action: "set_best_answer", context: &(post_id, answer_post_id) })
        .await?;
    if !decision.allowed {
        return Err(PostError::DecisionDenied);
    }

    let write_intent = WriteIntent {
        entity_type: "forum.post",
        operation: WriteOperation::Update,
        data: PostUpdateData { best_answer_id: Some(answer_post_id), ..default() },
        security_level: 2,
    };
    self.kind_mother.execute(write_intent).await?;

    self.karma_operator.apply_gain(answer_author_id, KarmaGain::AnswerAccepted, post.forum_id).await?;
    self.karma_operator.apply_gain(mandate.user_id(), KarmaGain::AcceptingAnswer, post.forum_id).await?;
    Ok(())
}
```

### 2.3 Modération : confirmer un post comme offensif

**Pattern :** WriteIntent + Mandate modérateur + Karma -100 + TAMR

```rust
impl ModerationOperator {
    pub async fn mark_as_offensive(
        &self,
        post_id: Uuid,
        reason_id: Uuid,
        mandate: Mandate,
    ) -> Result<(), ModerationError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["moderation.flag_offensive"])?;

        if !self.master_butler.can_moderate(mandate.user_id()).await? {
            return Err(ModerationError::NoModerationRight);
        }

        let decision = self.strong_father
            .decide(DecisionRequest { action: "mark_offensive", context: &(post_id, reason_id) })
            .await?;
        if !decision.allowed {
            return Err(ModerationError::DecisionDenied);
        }

        let post = self.post_operator.get_post(post_id).await?;
        let write_intent = WriteIntent {
            entity_type: "forum.post",
            operation: WriteOperation::Update,
            data: PostUpdateData {
                state: PostState::FlaggedOffensive,
                close_reason_id: Some(reason_id),
                hidden_from_non_moderators: true,
                ..default()
            },
            security_level: 2,
        };
        self.kind_mother.execute(write_intent).await?;

        self.karma_operator.apply_loss(post.author_id, KarmaLoss::AnswerFlagged, post.forum_id).await?;

        self.tamr.record_human_intervention(HumanIntervention {
            action: "mark_offensive",
            operator: self.id(),
            resource: post_id,
            reason_id,
        }).await?;

        Ok(())
    }
}
```

---

## 3. Gouvernance par Core

| Action | StrongFather | KindMother | Master Butler | WorrySentinel | TAMR | KarmaOperator |
|--------|--------------|------------|---------------|---------------|------|---------------|
| Créer post | Décision | WriteIntent | Permissions | Niveau sécurité | — | Seuils, gains |
| Voter | — | WriteIntent (vote) | — | — | — | Seuils, gains/pertes |
| Best answer | Décision | WriteIntent | — | — | — | Seuils, gains |
| Valider post | Décision | WriteIntent (état) | Droit modérer | — | Enregistrement | — |
| Marquer offensif | Décision | WriteIntent (état) | Droit modérer | Contenu | Enregistrement | -100 |
| Fermer question | Décision | WriteIntent (état) | Droit close | — | Enregistrement | -100 si motif |

---

## 4. Recommandations

- **Mandats** : Toujours valider `validate_operators` et `validate_flows` en entrée ; révoquer en cas de violation (spam, abus).
- **Karma** : Centraliser gains/pertes dans KarmaOperator ; un seul appel par action (éviter doublons).
- **Modération** : Toute action de modération doit passer par TAMR pour traçabilité (Maintenance explicable).
- **Façade publique** : Lecture forum/public sans Mandat via Façade Publique Gouvernée ; écriture et modération toujours sous Mandat.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
