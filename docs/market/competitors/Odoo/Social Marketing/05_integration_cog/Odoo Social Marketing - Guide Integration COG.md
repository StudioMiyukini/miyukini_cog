# Odoo Social Marketing — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Social Marketing dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Social Marketing
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (connexion compte, création post, planification, création lead depuis commentaire)
- Gestion des gouvernances

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
SocialMarketingUI → BondingBrother → SocialPostOperator / SocialAccountOperator / SocialCampaignOperator / SocialLeadOperator / SocialVisitorOperator
                                              → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
                                              → Ever Buddy (cycle de vie post / compte)
```

### 1.2 Flux Typiques

**Connexion d’un compte social :**
1. Intention utilisateur (Add Stream, plateforme) → SocialMarketingUI
2. Traduction intention → BondingBrother
3. Vérification permissions → Master Butler
4. Décision de connexion → StrongFather
5. Validation sécurité (stockage token) → WorrySentinel
6. Persistance compte + token (WriteIntent) → KindMother
7. Création stream associé (optionnel) → SocialStreamOperator → KindMother

**Création et publication d’un post :**
1. Intention (New Post, message, comptes, When) → SocialMarketingUI
2. BondingBrother → SocialPostOperator
3. Vérification permissions → Master Butler
4. Décision (publier maintenant ou planifier) → StrongFather
5. WriteIntent (création post draft ou scheduled) → KindMother
6. Si Send Now : appel API publication (gouverné) → puis mise à jour état published via KindMother
7. Si Schedule later : état scheduled ; job planifié déclenche publication à la date → StrongFather + API → KindMother (published_date, state)

**Création d’un lead depuis un commentaire :**
1. Intention (Create Lead sur commentaire) → SocialMarketingUI
2. BondingBrother → SocialLeadOperator
3. Vérification permissions CRM → Master Butler
4. Décision de création lead → StrongFather
5. WriteIntent (création crm.lead avec contexte commentaire/post/compte) → KindMother (délégation CRM)
6. Lien traçable commentaire → lead

**Envoi Email / SMS à un visiteur :**
1. Intention (Email ou SMS visiteur) → SocialMarketingUI
2. BondingBrother → SocialVisitorOperator
3. Permissions → Master Butler
4. Décision d’envoi → StrongFather
5. Vérification consentement / RGPD → WorrySentinel
6. Envoi via MiyuMail / MiyuSMS (Mandat)

---

## 2. Patterns d'Intégration

### 2.1 Connexion d’un compte social

**Pattern :** WriteIntent + Mandate + WorrySentinel (tokens)

```rust
// Pseudo-code Rust
pub struct ConnectAccountIntent {
    pub media_type: MediaType,  // facebook | instagram | linkedin | twitter | youtube | push
    pub company_id: Uuid,
    pub oauth_callback_params: OAuthCallbackParams,
}

impl SocialAccountOperator {
    pub async fn connect_account(
        &self,
        intent: ConnectAccountIntent,
        mandate: Mandate,
    ) -> Result<SocialAccountId, GovernanceError> {
        // 1. Vérification Mandat (StrongFather, KindMother, WorrySentinel)
        mandate.require_cores([StrongFather, KindMother, WorrySentinel])?;
        
        // 2. Décision StrongFather
        self.strong_father.authorize_connect(intent.media_type(), intent.company_id()).await?;
        
        // 3. Sécurité tokens (WorrySentinel)
        self.worry_sentinel.validate_token_storage(intent.media_type()).await?;
        
        // 4. Échange OAuth déjà fait côté UI ; ici on reçoit les params
        let token = self.exchange_oauth(intent.oauth_callback_params).await?;
        
        // 5. WriteIntent KindMother
        let write = WriteIntent::create_social_account {
            media_type: intent.media_type,
            company_id: intent.company_id,
            name: token.account_name,
            handle: token.handle,
            token_encrypted: self.worry_sentinel.encrypt(token.access_token),
        };
        let account_id = self.kind_mother.persist(write).await?;
        
        Ok(account_id)
    }
}
```

### 2.2 Création et planification d’un post

**Pattern :** WriteIntent (draft/scheduled) + Mandate

```rust
// Pseudo-code Rust
pub struct CreatePostIntent {
    pub message: String,
    pub account_ids: Vec<Uuid>,
    pub image_ids: Option<Vec<Uuid>>,
    pub campaign_id: Option<Uuid>,
    pub when: PostSchedule,  // SendNow | Scheduled(datetime)
    pub push_options: Option<PushNotificationOptions>,
}

impl SocialPostOperator {
    pub async fn create_post(
        &self,
        intent: CreatePostIntent,
        mandate: Mandate,
    ) -> Result<PostId, GovernanceError> {
        mandate.require_cores([StrongFather, KindMother])?;
        
        self.strong_father.authorize_post(intent.account_ids(), intent.when()).await?;
        
        let state = match intent.when {
            PostSchedule::SendNow => PostState::Draft,  // sera publié juste après
            PostSchedule::Scheduled(_) => PostState::Scheduled,
        };
        
        let write = WriteIntent::create_social_post {
            message: intent.message,
            account_ids: intent.account_ids,
            image_ids: intent.image_ids,
            campaign_id: intent.campaign_id,
            state,
            scheduled_date: intent.when.scheduled_date(),
            push_options: intent.push_options,
        };
        let post_id = self.kind_mother.persist(write).await?;
        
        if matches!(intent.when, PostSchedule::SendNow) {
            self.publish_post_now(post_id, mandate).await?;
        }
        
        Ok(post_id)
    }
}
```

### 2.3 Création d’un lead depuis un commentaire

**Pattern :** WriteIntent (lead) + Mandate (CRM)

```rust
// Pseudo-code Rust
pub struct CreateLeadFromCommentIntent {
    pub post_id: Uuid,
    pub comment_id: String,  // id plateforme
    pub comment_body: String,
    pub author_handle: Option<String>,
    pub link_option: LeadLinkOption,  // NewCustomer | LinkExisting(partner_id) | NoLink
}

impl SocialLeadOperator {
    pub async fn create_lead_from_comment(
        &self,
        intent: CreateLeadFromCommentIntent,
        mandate: Mandate,
    ) -> Result<LeadId, GovernanceError> {
        mandate.require_cores([StrongFather, KindMother, MasterButler])?;
        
        self.master_butler.require_permission("crm.lead.create").await?;
        self.strong_father.authorize_create_lead_from_social().await?;
        
        let lead_data = LeadData {
            name: intent.comment_body.truncate(80),
            description: intent.comment_body,
            source: LeadSource::SocialComment {
                post_id: intent.post_id,
                comment_id: intent.comment_id,
                author_handle: intent.author_handle,
            },
            partner_id: intent.link_option.partner_id(),
        };
        
        let write = WriteIntent::create_crm_lead(lead_data);
        let lead_id = self.kind_mother.persist(write).await?;  // délégation CRM
        
        Ok(lead_id)
    }
}
```

### 2.4 Ajout d’un contenu à une campagne

**Pattern :** StrongFather + Opérateur contenu

```rust
// Pseudo-code Rust
pub enum CampaignContentType {
    SocialPost(CreatePostIntent),
    Mailing(MailingIntent),
    Sms(SmsIntent),
    PushNotification(PushIntent),
}

impl SocialCampaignOperator {
    pub async fn add_content(
        &self,
        campaign_id: Uuid,
        content: CampaignContentType,
        mandate: Mandate,
    ) -> Result<ContentId, GovernanceError> {
        mandate.require_cores([StrongFather, KindMother])?;
        self.strong_father.authorize_campaign_content(campaign_id, &content).await?;
        
        let content_id = match content {
            CampaignContentType::SocialPost(i) => self.social_post.create_post(i, mandate).await?,
            CampaignContentType::Mailing(i) => self.miyu_mail.create_mailing(i, mandate).await?,
            CampaignContentType::Sms(i) => self.miyu_sms.send_sms(i, mandate).await?,
            CampaignContentType::PushNotification(i) => self.social_post.create_post(i.into(), mandate).await?,
        };
        
        let write = WriteIntent::link_campaign_content { campaign_id, content_id, content_type };
        self.kind_mother.persist(write).await?;
        
        Ok(content_id)
    }
}
```

---

## 3. Gestion des Erreurs et Rollback

- **OAuth refusé ou token révoqué :** WorrySentinel signale ; pas de persistance token ; message utilisateur « Connexion échouée ».
- **Publication API échouée :** État post → `failed` ; WriteIntent update state ; log ou notification ; pas de rollback des autres comptes si publication partielle (décision métier : retry ou manuel).
- **Création lead échouée :** Rollback KindMother si erreur après validation StrongFather ; message « Impossible de créer le lead ».
- **Multi-company / quota API :** WorrySentinel ou Border Guard peut refuser la connexion d’un nouveau compte si limite atteinte ; message explicite.

---

## 4. Intégration avec Kits Existants

- **MiyuCRM** : SocialLeadOperator appelle KindMother avec WriteIntent create_crm_lead ; le module CRM enregistre le lead.
- **MiyuMail / MiyuSMS** : SocialCampaignOperator et SocialVisitorOperator demandent envoi via BondingBrother + Mandat.
- **MiyuWeb** : Visiteurs (website.visitor) et paramètres push lus par SocialVisitorOperator et SocialPostOperator ; pas d’écriture directe sur Website.
- **UTM** : Campagnes et posts portent campaign_id (utm.campaign) ; Sales / Invoicing / CRM filtrent par UTM pour smart buttons.

---

## 5. Tests d'Intégration COG

- **Connexion compte** : Mock OAuth → StrongFather + KindMother + WorrySentinel invoqués ; compte créé et token chiffré.
- **Création post** : Draft puis Scheduled ; vérification WriteIntent et état ; mock API pour Send Now → état published.
- **Create Lead from comment** : Intent avec commentaire → StrongFather + KindMother ; lead créé en base avec source SocialComment.
- **Campagne + contenu** : Ajout post puis mailing (si MiyuMail) ; vérification lien campaign_id et onglets.

---

**Document** : Odoo Social Marketing — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
