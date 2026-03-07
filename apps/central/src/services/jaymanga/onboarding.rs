//! Onboarding JayManga — phase initiale avec Miou.
//!
//! Flow :
//!   1. Accueil Miou + choix profil (Lecteur / Vendeur)
//!   2. Explication du fonctionnement du service
//!   3. Contrat Lecteur (bouton « Suivant » différé de quelques secondes)
//!   4. Contrat Vendeur (si profil vendeur activé, bouton différé)
//!   5. Fin → accès au service

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;

/// Étape de l'onboarding JayManga.
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
    let p = use_palette();

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

            // Indicateur de progression des étapes
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
                                    p.bg_overlay
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

// ── Étape 1 : Accueil Miou ──────────────────────────────────────────────

#[component]
fn WelcomeStep(
    on_choose_reader: EventHandler<MouseEvent>,
    on_choose_seller: EventHandler<MouseEvent>,
) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 24px; max-width: 600px; text-align: center;",

            // Avatar Miou
            div {
                style: "width: 96px; height: 96px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 48px; box-shadow: 0 4px 20px #FF6B3540;",
                "🐱"
            }

            // Bulle de dialogue Miou
            div {
                style: "background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 16px; padding: 24px 32px; position: relative; max-width: 520px;",

                // Pointe de la bulle
                div {
                    style: "position: absolute; top: -8px; left: 50%; transform: translateX(-50%) rotate(45deg); width: 16px; height: 16px; background: {p.bg_secondary}; border-left: 1px solid {p.border_default}; border-top: 1px solid {p.border_default};",
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou"
                    }
                    p {
                        style: "font-size: 16px; color: {p.text_high}; line-height: 1.6;",
                        "Bienvenue sur JayManga ! 📚"
                    }
                    p {
                        style: "font-size: 14px; color: {p.text_secondary}; line-height: 1.6;",
                        "Je suis Miou, ton guide dans l'univers JayManga. Avant de commencer, dis-moi comment tu veux utiliser le service :"
                    }
                }
            }

            // Choix de profil
            div {
                style: "display: flex; gap: 16px; margin-top: 8px;",

                // Carte Lecteur
                button {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 24px 32px; background: {p.bg_secondary}; border: 2px solid {p.border_default}; border-radius: 12px; cursor: pointer; transition: all 0.2s; min-width: 200px; color: {p.text_primary};",
                    onmouseenter: move |_| {},
                    onclick: move |evt| on_choose_reader.call(evt),

                    span { style: "font-size: 40px;", "📖" }
                    span { style: "font-size: 16px; font-weight: 600; color: {p.text_high};", "Lecteur" }
                    span { style: "font-size: 12px; color: {p.text_muted}; line-height: 1.4;", "Lire, collectionner et suivre ta progression manga" }
                }

                // Carte Vendeur
                button {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 24px 32px; background: {p.bg_secondary}; border: 2px solid {p.border_default}; border-radius: 12px; cursor: pointer; transition: all 0.2s; min-width: 200px; color: {p.text_primary};",
                    onclick: move |evt| on_choose_seller.call(evt),

                    span { style: "font-size: 40px;", "🏪" }
                    span { style: "font-size: 16px; font-weight: 600; color: {p.text_high};", "Lecteur + Vendeur" }
                    span { style: "font-size: 12px; color: {p.text_muted}; line-height: 1.4;", "Publier, vendre et gérer tes œuvres en plus de lire" }
                }
            }

            p {
                style: "font-size: 11px; color: {p.text_muted}; margin-top: 4px;",
                "Tu pourras activer les fonctionnalités vendeur plus tard si tu changes d'avis."
            }
        }
    }
}

// ── Étape 2 : Explication du service ────────────────────────────────────

#[component]
fn ExplanationStep(is_seller: bool, on_next: EventHandler<MouseEvent>) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 24px; max-width: 640px;",

            // Avatar Miou
            div {
                style: "width: 72px; height: 72px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 36px;",
                "🐱"
            }

            // Bulle explication
            div {
                style: "background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 16px; padding: 24px 32px; width: 100%;",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou — Comment fonctionne JayManga"
                    }

                    p {
                        style: "font-size: 14px; color: {p.text_secondary}; line-height: 1.7;",
                        "JayManga est un service intégré à ton COG. Voici ce que tu peux faire :"
                    }

                    // Fonctionnalités lecteur
                    div {
                        style: "display: flex; flex-direction: column; gap: 10px; padding-left: 8px;",

                        FeatureItem { icon: "📚", text: "Bibliothèque — Retrouve tes favoris, achats et téléchargements, même hors-ligne." }
                        FeatureItem { icon: "📖", text: "Liseuse native — Lis tes mangas directement dans Central avec navigation clavier, zoom et suivi de progression." }
                        FeatureItem { icon: "⭐", text: "Progression — Gagne de l'XP, débloque des badges et maintiens ta streak de lecture quotidienne." }
                        FeatureItem { icon: "🌐", text: "Découverte — Explore les catalogues des COGs JayManga via le réseau Miyukini Webway." }
                    }

                    if is_seller {
                        div {
                            style: "border-top: 1px solid {p.border_default}; padding-top: 12px; margin-top: 4px;",

                            p {
                                style: "font-size: 13px; color: #FF6B35; font-weight: 500; margin-bottom: 8px;",
                                "🏪 Fonctionnalités vendeur"
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 10px; padding-left: 8px;",

                                FeatureItem { icon: "📝", text: "Catalogue — Crée et gère ton catalogue d'œuvres avec un éditeur complet multi-étapes." }
                                FeatureItem { icon: "💰", text: "Ventes — Suis tes revenus, transactions et licences avec export CSV/PDF." }
                                FeatureItem { icon: "🔗", text: "Portail — Rends ton catalogue accessible aux autres COGs du réseau." }
                            }
                        }
                    }
                }
            }

            button {
                style: "padding: 12px 32px; background: #FF6B35; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500; transition: all 0.2s;",
                onclick: move |evt| on_next.call(evt),
                "Compris, continuons →"
            }
        }
    }
}

#[component]
fn FeatureItem(icon: &'static str, text: &'static str) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; gap: 10px; align-items: flex-start;",
            span { style: "font-size: 16px; flex-shrink: 0;", "{icon}" }
            p { style: "font-size: 13px; color: {p.text_secondary}; line-height: 1.5;", "{text}" }
        }
    }
}

// ── Étape 3 : Contrat Lecteur ───────────────────────────────────────────

/// Délai en secondes avant que le bouton « Suivant » soit disponible.
const BUTTON_DELAY_SECS: u64 = 4;

#[component]
fn ReaderContractStep(on_accept: EventHandler<MouseEvent>) -> Element {
    let p = use_palette();
    let mut button_enabled = use_signal(|| false);
    let mut remaining_secs = use_signal(|| BUTTON_DELAY_SECS as i32);
    let mut checkbox_accepted = use_signal(|| false);

    // Timer pour activer le bouton après BUTTON_DELAY_SECS secondes
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
    let cb_border_color = if accepted { "#FF6B35" } else { p.border_default };
    let cb_bg_color = if accepted { "#FF6B35" } else { "transparent" };
    let btn_bg = if can_proceed { "#FF6B35" } else { p.bg_overlay };
    let btn_color = if can_proceed { "white" } else { p.text_muted };
    let btn_border = if can_proceed { "none".to_string() } else { format!("1px solid {}", p.border_default) };
    let btn_cursor = if can_proceed { "pointer" } else { "not-allowed" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 680px; width: 100%;",

            // En-tête
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "width: 48px; height: 48px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                    "🐱"
                }
                div {
                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou"
                    }
                    p {
                        style: "font-size: 15px; color: {p.text_high}; margin-top: 4px;",
                        "Pour utiliser JayManga, tu dois accepter les conditions d'utilisation suivantes. Prends le temps de les lire."
                    }
                }
            }

            // Corps du contrat
            div {
                style: "background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 12px; padding: 24px; max-height: 400px; overflow-y: auto;",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    h3 {
                        style: "font-size: 16px; color: {p.text_high}; font-weight: 600; text-align: center; padding-bottom: 12px; border-bottom: 1px solid {p.border_default};",
                        "📜 Conditions d'utilisation — Lecteur JayManga"
                    }

                    ContractSection {
                        number: "1",
                        title: "Usage personnel",
                        text: "L'utilisateur s'engage à utiliser le service JayManga exclusivement dans un cadre personnel et non commercial. Il est interdit de modifier, altérer, contourner ou détourner le fonctionnement du service, de ses mécanismes de protection ou de ses systèmes de licence.",
                    }
                    ContractSection {
                        number: "2",
                        title: "Propriété intellectuelle des œuvres",
                        text: "Toutes les œuvres accessibles via JayManga demeurent la propriété exclusive de leurs ayants droit respectifs. L'utilisateur s'engage à ne pas reproduire, modifier, redistribuer, partager publiquement ou exploiter commercialement les œuvres téléchargées, en tout ou partie, par quelque moyen que ce soit.",
                    }
                    ContractSection {
                        number: "3",
                        title: "Licence de lecture",
                        text: "L'achat ou le téléchargement d'une œuvre confère à l'utilisateur une licence de lecture personnelle, non cessible et non exclusive. Cette licence peut être révoquée en cas de violation des présentes conditions.",
                    }
                    ContractSection {
                        number: "4",
                        title: "Disponibilité du service",
                        text: "Miyukini COG n'assure pas la disponibilité permanente des œuvres ni du service. Le catalogue, les contenus et les fonctionnalités peuvent être modifiés, suspendus ou supprimés à tout moment sans préavis. L'accès aux œuvres dépend de la disponibilité des COGs vendeurs sur le réseau.",
                    }
                    ContractSection {
                        number: "5",
                        title: "Données et confidentialité",
                        text: "Les données de lecture, de progression et de préférences sont stockées localement sur le COG de l'utilisateur (LOI-3 : souveraineté des données). Aucune donnée personnelle n'est transmise à des tiers sans consentement explicite. Les données de synchronisation inter-COG sont limitées au strict nécessaire.",
                    }
                    ContractSection {
                        number: "6",
                        title: "Comportement et contenu",
                        text: "L'utilisateur s'engage à ne pas utiliser le service à des fins illicites, diffamatoires ou portant atteinte aux droits d'autrui. Miyukini COG se réserve le droit de limiter ou suspendre l'accès au service en cas de comportement abusif.",
                    }
                    ContractSection {
                        number: "7",
                        title: "Limitation de responsabilité",
                        text: "Miyukini COG est fourni « en l'état ». En aucun cas Miyukini ne pourra être tenu responsable des dommages directs ou indirects résultant de l'utilisation du service, y compris la perte de données, l'interruption de service ou l'indisponibilité des contenus.",
                    }
                    ContractSection {
                        number: "8",
                        title: "Modification des conditions",
                        text: "Les présentes conditions peuvent être modifiées à tout moment. L'utilisateur sera informé des modifications lors de sa prochaine connexion. La poursuite de l'utilisation du service vaut acceptation des nouvelles conditions.",
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
                        span { style: "color: white; font-size: 12px; font-weight: 700;", "✓" }
                    }
                }
                span {
                    style: "font-size: 13px; color: {p.text_secondary}; cursor: pointer;",
                    onclick: move |_| {
                        let val = *checkbox_accepted.read();
                        checkbox_accepted.set(!val);
                    },
                    "J'ai lu et j'accepte les conditions d'utilisation de JayManga"
                }
            }

            // Bouton avec délai
            button {
                style: "padding: 12px 32px; background: {btn_bg}; color: {btn_color}; border: {btn_border}; border-radius: 8px; cursor: {btn_cursor}; font-size: 14px; font-weight: 500; transition: all 0.3s; align-self: center;",
                disabled: !can_proceed,
                onclick: move |evt| {
                    if can_proceed {
                        on_accept.call(evt);
                    }
                },
                if !enabled {
                    "Lecture en cours… ({remaining}s)"
                } else if !accepted {
                    "Veuillez accepter les conditions"
                } else {
                    "J'accepte et je continue →"
                }
            }
        }
    }
}

// ── Étape 4 : Contrat Vendeur ───────────────────────────────────────────

#[component]
fn SellerContractStep(on_accept: EventHandler<MouseEvent>) -> Element {
    let p = use_palette();
    let mut button_enabled = use_signal(|| false);
    let mut remaining_secs = use_signal(|| BUTTON_DELAY_SECS as i32);
    let mut checkbox_accepted = use_signal(|| false);

    // Timer pour activer le bouton après BUTTON_DELAY_SECS secondes
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
    let cb_border_color = if accepted { "#FF6B35" } else { p.border_default };
    let cb_bg_color = if accepted { "#FF6B35" } else { "transparent" };
    let btn_bg = if can_proceed { "#FF6B35" } else { p.bg_overlay };
    let btn_color = if can_proceed { "white" } else { p.text_muted };
    let btn_border = if can_proceed { "none".to_string() } else { format!("1px solid {}", p.border_default) };
    let btn_cursor = if can_proceed { "pointer" } else { "not-allowed" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 680px; width: 100%;",

            // En-tête
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "width: 48px; height: 48px; border-radius: 50%; background: linear-gradient(135deg, #FF6B35, #FFD700); display: flex; align-items: center; justify-content: center; font-size: 24px; flex-shrink: 0;",
                    "🐱"
                }
                div {
                    p {
                        style: "font-size: 11px; color: #FF6B35; font-weight: 600; letter-spacing: 1px; text-transform: uppercase;",
                        "Miou"
                    }
                    p {
                        style: "font-size: 15px; color: {p.text_high}; margin-top: 4px;",
                        "Tu as choisi d'activer les fonctionnalités vendeur. Voici les engagements supplémentaires liés à cette activité."
                    }
                }
            }

            // Corps du contrat vendeur
            div {
                style: "background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 12px; padding: 24px; max-height: 400px; overflow-y: auto;",

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    h3 {
                        style: "font-size: 16px; color: {p.text_high}; font-weight: 600; text-align: center; padding-bottom: 12px; border-bottom: 1px solid {p.border_default};",
                        "📜 Conditions d'utilisation — Vendeur JayManga"
                    }

                    ContractSection {
                        number: "1",
                        title: "Responsabilité éditoriale",
                        text: "Le vendeur est seul et entièrement responsable des œuvres qu'il publie sur JayManga. Il garantit détenir les droits nécessaires à la publication, la reproduction et la distribution des contenus mis en ligne. Miyukini COG ne contrôle pas et ne valide pas les contenus publiés.",
                    }
                    ContractSection {
                        number: "2",
                        title: "Distribution et commerce",
                        text: "Le vendeur est seul responsable de la mise en vente, de la tarification, de la distribution et de la promotion de ses œuvres. Il assume l'intégralité des obligations légales liées à l'activité de vente en ligne, y compris les obligations fiscales, déclaratives et réglementaires applicables dans sa juridiction.",
                    }
                    ContractSection {
                        number: "3",
                        title: "Mise à disposition et gestion du catalogue",
                        text: "Le vendeur est responsable de la mise à disposition de ses œuvres, de la qualité des fichiers publiés, de l'exactitude des métadonnées et de la gestion de ses chapitres et pages. Il veille à ce que son catalogue soit conforme aux lois en vigueur.",
                    }
                    ContractSection {
                        number: "4",
                        title: "Service après-vente",
                        text: "Le vendeur assure seul le service après-vente auprès de ses acheteurs : remboursements, réclamations, questions relatives aux licences. Miyukini COG n'intervient pas dans la relation commerciale entre le vendeur et ses acheteurs et ne peut être tenu responsable de litiges entre ces parties.",
                    }
                    ContractSection {
                        number: "5",
                        title: "Contenus illicites et modération",
                        text: "Le vendeur s'engage à ne publier aucun contenu illicite, contrefait, diffamatoire, pornographique impliquant des mineurs, incitant à la haine ou à la violence, ou portant atteinte aux droits de tiers. Miyukini COG se réserve le droit de retirer tout contenu signalé ou manifestement illicite, sans préavis ni indemnité.",
                    }
                    ContractSection {
                        number: "6",
                        title: "Propriété intellectuelle",
                        text: "Le vendeur conserve l'intégralité de ses droits de propriété intellectuelle sur ses œuvres. La publication sur JayManga confère uniquement une licence technique de diffusion limitée au fonctionnement du service. Cette licence prend fin dès le retrait de l'œuvre par le vendeur.",
                    }
                    ContractSection {
                        number: "7",
                        title: "Disponibilité et interruption",
                        text: "Miyukini COG ne garantit pas la disponibilité permanente du service de vente. Le vendeur reconnaît que le service peut être interrompu, modifié ou arrêté à tout moment. Miyukini COG ne pourra être tenu responsable des pertes de revenus liées à une interruption du service.",
                    }
                    ContractSection {
                        number: "8",
                        title: "Limitation de responsabilité",
                        text: "Miyukini COG agit exclusivement en tant qu'hébergeur technique. La responsabilité de Miyukini COG ne pourra être engagée au titre des contenus publiés par les vendeurs, des transactions effectuées ou des litiges commerciaux. Le vendeur garantit Miyukini COG contre toute réclamation de tiers liée à ses contenus.",
                    }
                    ContractSection {
                        number: "9",
                        title: "Résiliation",
                        text: "Miyukini COG se réserve le droit de suspendre ou résilier le compte vendeur en cas de violation des présentes conditions, sans préavis ni indemnité. Le vendeur peut cesser son activité de vente à tout moment en désactivant les fonctionnalités vendeur depuis les paramètres du service.",
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
                        span { style: "color: white; font-size: 12px; font-weight: 700;", "✓" }
                    }
                }
                span {
                    style: "font-size: 13px; color: {p.text_secondary}; cursor: pointer;",
                    onclick: move |_| {
                        let val = *checkbox_accepted.read();
                        checkbox_accepted.set(!val);
                    },
                    "J'ai lu et j'accepte les conditions vendeur de JayManga"
                }
            }

            // Bouton avec délai
            button {
                style: "padding: 12px 32px; background: {btn_bg}; color: {btn_color}; border: {btn_border}; border-radius: 8px; cursor: {btn_cursor}; font-size: 14px; font-weight: 500; transition: all 0.3s; align-self: center;",
                disabled: !can_proceed,
                onclick: move |evt| {
                    if can_proceed {
                        on_accept.call(evt);
                    }
                },
                if !enabled {
                    "Lecture en cours… ({remaining}s)"
                } else if !accepted {
                    "Veuillez accepter les conditions"
                } else {
                    "J'accepte et je commence →"
                }
            }
        }
    }
}

// ── Composant réutilisable : section de contrat ─────────────────────────

#[component]
fn ContractSection(
    number: &'static str,
    title: &'static str,
    text: &'static str,
) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; gap: 12px; align-items: flex-start;",

            // Numéro
            div {
                style: "width: 28px; height: 28px; border-radius: 50%; background: #FF6B3520; color: #FF6B35; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 600; flex-shrink: 0;",
                "{number}"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 4px; flex: 1;",

                p {
                    style: "font-size: 13px; color: {p.text_high}; font-weight: 600;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {p.text_muted}; line-height: 1.6;",
                    "{text}"
                }
            }
        }
    }
}
