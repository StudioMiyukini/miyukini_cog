//! Onboarding JayManga — phase initiale avec Miou.
//!
//! Flow :
//!   1. Accueil Miou + choix profil (Lecteur / Vendeur)
//!   2. Explication du fonctionnement du service
//!   3. Contrat Lecteur (bouton "Suivant" differe de quelques secondes)
//!   4. Contrat Vendeur (si profil vendeur active, bouton differe)
//!   5. Fin -> acces au service

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Etape de l'onboarding JayManga.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnboardingStep {
    #[default]
    Welcome,
    Explanation,
    ReaderContract,
    SellerContract,
}

/// Composant principal de l'onboarding.
#[component]
pub fn Onboarding(on_complete: EventHandler<bool>) -> Element {
    let c = use_palette();

    let mut step = use_signal(|| OnboardingStep::Welcome);
    let mut wants_seller = use_signal(|| false);
    let mut reader_contract_accepted = use_signal(|| false);
    let mut seller_contract_accepted = use_signal(|| false);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; padding: 40px; overflow-y: auto;",

            match *step.read() {
                OnboardingStep::Welcome => rsx! {
                    WelcomeStep {
                        on_choose_reader: move |_| {
                            wants_seller.set(false);
                            step.set(OnboardingStep::Explanation);
                        },
                        on_choose_seller: move |_| {
                            wants_seller.set(true);
                            step.set(OnboardingStep::Explanation);
                        },
                    }
                },
                OnboardingStep::Explanation => rsx! {
                    ExplanationStep {
                        is_seller: *wants_seller.read(),
                        on_next: move |_| {
                            step.set(OnboardingStep::ReaderContract);
                        },
                    }
                },
                OnboardingStep::ReaderContract => rsx! {
                    ReaderContractStep {
                        on_accept: move |_| {
                            reader_contract_accepted.set(true);
                            if *wants_seller.read() {
                                step.set(OnboardingStep::SellerContract);
                            } else {
                                on_complete.call(false);
                            }
                        },
                    }
                },
                OnboardingStep::SellerContract => rsx! {
                    SellerContractStep {
                        on_accept: move |_| {
                            seller_contract_accepted.set(true);
                            on_complete.call(true);
                        },
                    }
                },
            }

            // Indicateur de progression des etapes
            div {
                style: "display: flex; gap: 8px; margin-top: 32px;",

                {
                    let current = *step.read();
                    let is_seller = *wants_seller.read();
                    let total = if is_seller { 4 } else { 3 };
                    let current_idx = match current {
                        OnboardingStep::Welcome => 0,
                        OnboardingStep::Explanation => 1,
                        OnboardingStep::ReaderContract => 2,
                        OnboardingStep::SellerContract => 3,
                    };

                    rsx! {
                        for i in 0..total {
                            {
                                let is_current = i == current_idx;
                                let is_done = i < current_idx;
                                let bg = if is_current {
                                    "#FF6B35"
                                } else if is_done {
                                    "#FF6B3580"
                                } else {
                                    c.bg_hover
                                };
                                {
                                    let w = if is_current { 24 } else { 8 };
                                    rsx! {
                                        div {
                                            style: "width: {w}px; height: 8px; border-radius: 4px; background: {bg}; transition: all 0.3s;",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// -- Etape 1 : Accueil Miou --

#[component]
fn WelcomeStep(
    on_choose_reader: EventHandler<MouseEvent>,
    on_choose_seller: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 24px; max-width: 600px; text-align: center;",

            // Avatar Miou
            div {
                style: "width: 96px; height: 96px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 48px; box-shadow: 0 4px 20px #FF6B3540;",
                "\u{1F431}"
            }

            // Bulle de dialogue Miou
            div {
                style: "background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 16px; padding: 24px 32px; position: relative; max-width: 520px;",

                // Pointe de la bulle
                div {
                    style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%) rotate(45deg); width: 16px; height: 16px; background: {c.bg_secondary}; border-left: 1px solid {c.border}; border-top: 1px solid {c.border};",
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou"
                    }
                    p {
                        style: "font-size: 16px; color: {c.text_white}; line-height: 1.6;",
                        "Bienvenue sur JayManga ! \u{1F4DA}"
                    }
                    p {
                        style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.6;",
                        "Je suis Miou, ton guide dans l'univers JayManga. Avant de commencer, dis-moi comment tu veux utiliser le service :"
                    }
                }
            }

            // Choix de profil
            div {
                style: "display: flex; gap: 16px; margin-top: 8px;",

                // Carte Lecteur
                button {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 24px 32px; background: {c.bg_secondary}; border: 2px solid {c.border}; border-radius: 12px; cursor: pointer; transition: all 0.2s; min-width: 200px; color: {c.text_primary};",
                    onmouseenter: move |_| {},
                    onclick: move |evt| on_choose_reader.call(evt),

                    span { style: "font-size: 40px;", "\u{1F4D6}" }
                    span { style: "font-size: 16px; font-weight: 600; color: {c.text_white};", "Lecteur" }
                    span { style: "font-size: 12px; color: {c.text_muted}; line-height: 1.4;", "Lire, collectionner et suivre ta progression manga" }
                }

                // Carte Vendeur
                button {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 24px 32px; background: {c.bg_secondary}; border: 2px solid {c.border}; border-radius: 12px; cursor: pointer; transition: all 0.2s; min-width: 200px; color: {c.text_primary};",
                    onclick: move |evt| on_choose_seller.call(evt),

                    span { style: "font-size: 40px;", "\u{1F3EA}" }
                    span { style: "font-size: 16px; font-weight: 600; color: {c.text_white};", "Lecteur + Vendeur" }
                    span { style: "font-size: 12px; color: {c.text_muted}; line-height: 1.4;", "Publier, vendre et gerer tes oeuvres en plus de lire" }
                }
            }

            p {
                style: "font-size: 11px; color: {c.text_muted}; margin-top: 4px;",
                "Tu pourras activer les fonctionnalites vendeur plus tard si tu changes d'avis."
            }
        }
    }
}

// -- Etape 2 : Explication du service --

#[component]
fn ExplanationStep(is_seller: bool, on_next: EventHandler<MouseEvent>) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 24px; max-width: 640px;",

            // Avatar Miou
            div {
                style: "width: 72px; height: 72px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 36px;",
                "\u{1F431}"
            }

            // Bulle explication
            div {
                style: "background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 16px; padding: 24px 32px; width: 100%;",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou \u{2014} Comment fonctionne JayManga"
                    }

                    p {
                        style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.7;",
                        "JayManga est un service integre a ton COG. Voici ce que tu peux faire :"
                    }

                    // Fonctionnalites lecteur
                    div {
                        style: "display: flex; flex-direction: column; gap: 10px; padding-left: 8px;",

                        FeatureItem { icon: "\u{1F4DA}", text: "Bibliotheque \u{2014} Retrouve tes favoris, achats et telechargements, meme hors-ligne." }
                        FeatureItem { icon: "\u{1F4D6}", text: "Liseuse native \u{2014} Lis tes mangas directement dans Central avec navigation clavier, zoom et suivi de progression." }
                        FeatureItem { icon: "\u{2B50}", text: "Progression \u{2014} Gagne de l'XP, debloque des badges et maintiens ta streak de lecture quotidienne." }
                        FeatureItem { icon: "\u{1F310}", text: "Decouverte \u{2014} Explore les catalogues des COGs JayManga via le reseau Miyukini Webway." }
                    }

                    if is_seller {
                        div {
                            style: "border-top: 1px solid {c.border}; padding-top: 12px; margin-top: 4px;",

                            p {
                                style: "font-size: 13px; color: #FF6B35; font-weight: 500; margin-bottom: 8px;",
                                "\u{1F3EA} Fonctionnalites vendeur"
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 10px; padding-left: 8px;",

                                FeatureItem { icon: "\u{1F4DD}", text: "Catalogue \u{2014} Cree et gere ton catalogue d'oeuvres avec un editeur complet multi-etapes." }
                                FeatureItem { icon: "\u{1F4B0}", text: "Ventes \u{2014} Suis tes revenus, transactions et licences avec export CSV/PDF." }
                                FeatureItem { icon: "\u{1F517}", text: "Portail \u{2014} Rends ton catalogue accessible aux autres COGs du reseau." }
                            }
                        }
                    }
                }
            }

            button {
                style: "padding: 12px 32px; background: #FF6B35; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500; transition: all 0.2s;",
                onclick: move |evt| on_next.call(evt),
                "Compris, continuons \u{2192}"
            }
        }
    }
}

#[component]
fn FeatureItem(icon: &'static str, text: &'static str) -> Element {
    let c = use_palette();
    rsx! {
        div {
            style: "display: flex; gap: 10px; align-items: flex-start;",
            span { style: "font-size: 16px; flex-shrink: 0;", "{icon}" }
            p { style: "font-size: 13px; color: {c.text_secondary}; line-height: 1.5;", "{text}" }
        }
    }
}

// -- Etape 3 : Contrat Lecteur --

/// Delai en secondes avant que le bouton "Suivant" soit disponible.
const BUTTON_DELAY_SECS: u64 = 4;

#[component]
fn ReaderContractStep(on_accept: EventHandler<MouseEvent>) -> Element {
    let c = use_palette();
    let mut button_enabled = use_signal(|| false);
    let mut remaining_secs = use_signal(|| BUTTON_DELAY_SECS as i32);
    let mut checkbox_accepted = use_signal(|| false);

    // Timer pour activer le bouton apres BUTTON_DELAY_SECS secondes
    let _ = use_resource(move || async move {
        for i in (0..BUTTON_DELAY_SECS).rev() {
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            remaining_secs.set(i as i32);
        }
        button_enabled.set(true);
    });

    let can_proceed = *button_enabled.read() && *checkbox_accepted.read();
    let remaining = *remaining_secs.read();
    let accepted = *checkbox_accepted.read();
    let enabled = *button_enabled.read();
    let cb_border_color = if accepted { "#FF6B35" } else { c.border };
    let cb_bg_color = if accepted { "#FF6B35" } else { "transparent" };
    let btn_bg = if can_proceed { "#FF6B35" } else { c.bg_hover };
    let btn_color = if can_proceed { "white" } else { c.text_muted };
    let btn_border = if can_proceed { "none".to_string() } else { format!("1px solid {}", c.border) };
    let btn_cursor = if can_proceed { "pointer" } else { "not-allowed" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 680px; width: 100%;",

            // En-tete
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "width: 48px; height: 48px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                    "\u{1F431}"
                }
                div {
                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou"
                    }
                    p {
                        style: "font-size: 15px; color: {c.text_white}; margin-top: 4px;",
                        "Pour utiliser JayManga, tu dois accepter les conditions d'utilisation suivantes. Prends le temps de les lire."
                    }
                }
            }

            // Corps du contrat
            div {
                style: "background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 12px; padding: 24px; max-height: 400px; overflow-y: auto;",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; font-weight: 600; text-align: center; padding-bottom: 12px; border-bottom: 1px solid {c.border};",
                        "\u{1F4DC} Conditions d'utilisation \u{2014} Lecteur JayManga"
                    }

                    ContractSection {
                        number: "1",
                        title: "Usage personnel",
                        text: "L'utilisateur s'engage a utiliser le service JayManga exclusivement dans un cadre personnel et non commercial. Il est interdit de modifier, alterer, contourner ou detourner le fonctionnement du service, de ses mecanismes de protection ou de ses systemes de licence.",
                    }
                    ContractSection {
                        number: "2",
                        title: "Propriete intellectuelle des oeuvres",
                        text: "Toutes les oeuvres accessibles via JayManga demeurent la propriete exclusive de leurs ayants droit respectifs. L'utilisateur s'engage a ne pas reproduire, modifier, redistribuer, partager publiquement ou exploiter commercialement les oeuvres telechargees, en tout ou partie, par quelque moyen que ce soit.",
                    }
                    ContractSection {
                        number: "3",
                        title: "Licence de lecture",
                        text: "L'achat ou le telechargement d'une oeuvre confere a l'utilisateur une licence de lecture personnelle, non cessible et non exclusive. Cette licence peut etre revoquee en cas de violation des presentes conditions.",
                    }
                    ContractSection {
                        number: "4",
                        title: "Disponibilite du service",
                        text: "Miyukini COG n'assure pas la disponibilite permanente des oeuvres ni du service. Le catalogue, les contenus et les fonctionnalites peuvent etre modifies, suspendus ou supprimes a tout moment sans preavis. L'acces aux oeuvres depend de la disponibilite des COGs vendeurs sur le reseau.",
                    }
                    ContractSection {
                        number: "5",
                        title: "Donnees et confidentialite",
                        text: "Les donnees de lecture, de progression et de preferences sont stockees localement sur le COG de l'utilisateur (LOI-3 : souverainete des donnees). Aucune donnee personnelle n'est transmise a des tiers sans consentement explicite. Les donnees de synchronisation inter-COG sont limitees au strict necessaire.",
                    }
                    ContractSection {
                        number: "6",
                        title: "Comportement et contenu",
                        text: "L'utilisateur s'engage a ne pas utiliser le service a des fins illicites, diffamatoires ou portant atteinte aux droits d'autrui. Miyukini COG se reserve le droit de limiter ou suspendre l'acces au service en cas de comportement abusif.",
                    }
                    ContractSection {
                        number: "7",
                        title: "Limitation de responsabilite",
                        text: "Miyukini COG est fourni \"en l'etat\". En aucun cas Miyukini ne pourra etre tenu responsable des dommages directs ou indirects resultant de l'utilisation du service, y compris la perte de donnees, l'interruption de service ou l'indisponibilite des contenus.",
                    }
                    ContractSection {
                        number: "8",
                        title: "Modification des conditions",
                        text: "Les presentes conditions peuvent etre modifiees a tout moment. L'utilisateur sera informe des modifications lors de sa prochaine connexion. La poursuite de l'utilisation du service vaut acceptation des nouvelles conditions.",
                    }
                }
            }

            // Checkbox d'acceptation
            div {
                style: "display: flex; align-items: center; gap: 10px; padding: 8px 0;",

                div {
                    style: "width: 18px; height: 18px; border: 2px solid {cb_border_color}; border-radius: 4px; background: {cb_bg_color}; cursor: pointer; display: flex; align-items: center; justify-content: center; flex-shrink: 0; transition: all 0.2s;",
                    onclick: move |_| {
                        let val = *checkbox_accepted.read();
                        checkbox_accepted.set(!val);
                    },
                    if accepted {
                        span { style: "color: white; font-size: 12px; font-weight: 700;", "\u{2713}" }
                    }
                }
                span {
                    style: "font-size: 13px; color: {c.text_secondary}; cursor: pointer;",
                    onclick: move |_| {
                        let val = *checkbox_accepted.read();
                        checkbox_accepted.set(!val);
                    },
                    "J'ai lu et j'accepte les conditions d'utilisation de JayManga"
                }
            }

            // Bouton avec delai
            button {
                style: "padding: 12px 32px; background: {btn_bg}; color: {btn_color}; border: {btn_border}; border-radius: 8px; cursor: {btn_cursor}; font-size: 14px; font-weight: 500; transition: all 0.3s; align-self: center;",
                disabled: !can_proceed,
                onclick: move |evt| {
                    if can_proceed {
                        on_accept.call(evt);
                    }
                },
                if !enabled {
                    "Lecture en cours\u{2026} ({remaining}s)"
                } else if !accepted {
                    "Veuillez accepter les conditions"
                } else {
                    "J'accepte et je continue \u{2192}"
                }
            }
        }
    }
}

// -- Etape 4 : Contrat Vendeur --

#[component]
fn SellerContractStep(on_accept: EventHandler<MouseEvent>) -> Element {
    let c = use_palette();
    let mut button_enabled = use_signal(|| false);
    let mut remaining_secs = use_signal(|| BUTTON_DELAY_SECS as i32);
    let mut checkbox_accepted = use_signal(|| false);

    // Timer pour activer le bouton apres BUTTON_DELAY_SECS secondes
    let _ = use_resource(move || async move {
        for i in (0..BUTTON_DELAY_SECS).rev() {
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            remaining_secs.set(i as i32);
        }
        button_enabled.set(true);
    });

    let can_proceed = *button_enabled.read() && *checkbox_accepted.read();
    let remaining = *remaining_secs.read();
    let accepted = *checkbox_accepted.read();
    let enabled = *button_enabled.read();
    let cb_border_color = if accepted { "#FF6B35" } else { c.border };
    let cb_bg_color = if accepted { "#FF6B35" } else { "transparent" };
    let btn_bg = if can_proceed { "#FF6B35" } else { c.bg_hover };
    let btn_color = if can_proceed { "white" } else { c.text_muted };
    let btn_border = if can_proceed { "none".to_string() } else { format!("1px solid {}", c.border) };
    let btn_cursor = if can_proceed { "pointer" } else { "not-allowed" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 680px; width: 100%;",

            // En-tete
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "width: 48px; height: 48px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                    "\u{1F431}"
                }
                div {
                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou"
                    }
                    p {
                        style: "font-size: 15px; color: {c.text_white}; margin-top: 4px;",
                        "Tu as choisi d'activer les fonctionnalites vendeur. Voici les engagements supplementaires lies a cette activite."
                    }
                }
            }

            // Corps du contrat vendeur
            div {
                style: "background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 12px; padding: 24px; max-height: 400px; overflow-y: auto;",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; font-weight: 600; text-align: center; padding-bottom: 12px; border-bottom: 1px solid {c.border};",
                        "\u{1F4DC} Conditions d'utilisation \u{2014} Vendeur JayManga"
                    }

                    ContractSection {
                        number: "1",
                        title: "Responsabilite editoriale",
                        text: "Le vendeur est seul et entierement responsable des oeuvres qu'il publie sur JayManga. Il garantit detenir les droits necessaires a la publication, la reproduction et la distribution des contenus mis en ligne. Miyukini COG ne controle pas et ne valide pas les contenus publies.",
                    }
                    ContractSection {
                        number: "2",
                        title: "Distribution et commerce",
                        text: "Le vendeur est seul responsable de la mise en vente, de la tarification, de la distribution et de la promotion de ses oeuvres. Il assume l'integralite des obligations legales liees a l'activite de vente en ligne, y compris les obligations fiscales, declaratives et reglementaires applicables dans sa juridiction.",
                    }
                    ContractSection {
                        number: "3",
                        title: "Mise a disposition et gestion du catalogue",
                        text: "Le vendeur est responsable de la mise a disposition de ses oeuvres, de la qualite des fichiers publies, de l'exactitude des metadonnees et de la gestion de ses chapitres et pages. Il veille a ce que son catalogue soit conforme aux lois en vigueur.",
                    }
                    ContractSection {
                        number: "4",
                        title: "Service apres-vente",
                        text: "Le vendeur assure seul le service apres-vente aupres de ses acheteurs : remboursements, reclamations, questions relatives aux licences. Miyukini COG n'intervient pas dans la relation commerciale entre le vendeur et ses acheteurs et ne peut etre tenu responsable de litiges entre ces parties.",
                    }
                    ContractSection {
                        number: "5",
                        title: "Contenus illicites et moderation",
                        text: "Le vendeur s'engage a ne publier aucun contenu illicite, contrefait, diffamatoire, pornographique impliquant des mineurs, incitant a la haine ou a la violence, ou portant atteinte aux droits de tiers. Miyukini COG se reserve le droit de retirer tout contenu signale ou manifestement illicite, sans preavis ni indemnite.",
                    }
                    ContractSection {
                        number: "6",
                        title: "Propriete intellectuelle",
                        text: "Le vendeur conserve l'integralite de ses droits de propriete intellectuelle sur ses oeuvres. La publication sur JayManga confere uniquement une licence technique de diffusion limitee au fonctionnement du service. Cette licence prend fin des le retrait de l'oeuvre par le vendeur.",
                    }
                    ContractSection {
                        number: "7",
                        title: "Disponibilite et interruption",
                        text: "Miyukini COG ne garantit pas la disponibilite permanente du service de vente. Le vendeur reconnait que le service peut etre interrompu, modifie ou arrete a tout moment. Miyukini COG ne pourra etre tenu responsable des pertes de revenus liees a une interruption du service.",
                    }
                    ContractSection {
                        number: "8",
                        title: "Limitation de responsabilite",
                        text: "Miyukini COG agit exclusivement en tant qu'hebergeur technique. La responsabilite de Miyukini COG ne pourra etre engagee au titre des contenus publies par les vendeurs, des transactions effectuees ou des litiges commerciaux. Le vendeur garantit Miyukini COG contre toute reclamation de tiers liee a ses contenus.",
                    }
                    ContractSection {
                        number: "9",
                        title: "Resiliation",
                        text: "Miyukini COG se reserve le droit de suspendre ou resilier le compte vendeur en cas de violation des presentes conditions, sans preavis ni indemnite. Le vendeur peut cesser son activite de vente a tout moment en desactivant les fonctionnalites vendeur depuis les parametres du service.",
                    }
                }
            }

            // Checkbox d'acceptation
            div {
                style: "display: flex; align-items: center; gap: 10px; padding: 8px 0;",

                div {
                    style: "width: 18px; height: 18px; border: 2px solid {cb_border_color}; border-radius: 4px; background: {cb_bg_color}; cursor: pointer; display: flex; align-items: center; justify-content: center; flex-shrink: 0; transition: all 0.2s;",
                    onclick: move |_| {
                        let val = *checkbox_accepted.read();
                        checkbox_accepted.set(!val);
                    },
                    if accepted {
                        span { style: "color: white; font-size: 12px; font-weight: 700;", "\u{2713}" }
                    }
                }
                span {
                    style: "font-size: 13px; color: {c.text_secondary}; cursor: pointer;",
                    onclick: move |_| {
                        let val = *checkbox_accepted.read();
                        checkbox_accepted.set(!val);
                    },
                    "J'ai lu et j'accepte les conditions vendeur de JayManga"
                }
            }

            // Bouton avec delai
            button {
                style: "padding: 12px 32px; background: {btn_bg}; color: {btn_color}; border: {btn_border}; border-radius: 8px; cursor: {btn_cursor}; font-size: 14px; font-weight: 500; transition: all 0.3s; align-self: center;",
                disabled: !can_proceed,
                onclick: move |evt| {
                    if can_proceed {
                        on_accept.call(evt);
                    }
                },
                if !enabled {
                    "Lecture en cours\u{2026} ({remaining}s)"
                } else if !accepted {
                    "Veuillez accepter les conditions"
                } else {
                    "J'accepte et je commence \u{2192}"
                }
            }
        }
    }
}

// -- Composant reutilisable : section de contrat --

#[component]
fn ContractSection(
    number: &'static str,
    title: &'static str,
    text: &'static str,
) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; gap: 12px; align-items: flex-start;",

            // Numero
            div {
                style: "width: 28px; height: 28px; border-radius: 50%; background: #FF6B3520; color: #FF6B35; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 600; flex-shrink: 0;",
                "{number}"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 4px; flex: 1;",

                p {
                    style: "font-size: 13px; color: {c.text_white}; font-weight: 600;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted}; line-height: 1.6;",
                    "{text}"
                }
            }
        }
    }
}
