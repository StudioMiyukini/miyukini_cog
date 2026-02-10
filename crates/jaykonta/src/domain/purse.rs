//! Types de domaine Purse (budget personnel).
//!
//! Couvre les entites : compte Purse, mouvements, categories,
//! budgets occasionnels, objectifs et alertes.

use serde::{Deserialize, Serialize};

// ─── Compte Purse ────────────────────────────────────────────

/// Compte Purse (budget personnel).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurseAccount {
    /// Identifiant unique.
    pub id: String,
    /// Identifiant utilisateur proprietaire.
    pub user_id: String,
    /// Libelle du compte.
    pub label: String,
    /// Devise principale.
    pub currency: String,
    /// Solde net (calcule : somme algebrique des mouvements).
    pub balance: f64,
    /// Revenus du mois en cours.
    pub month_income: f64,
    /// Depenses du mois en cours.
    pub month_expense: f64,
}

// ─── Categorie ───────────────────────────────────────────────

/// Categorie de mouvement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// Identifiant unique.
    pub id: String,
    /// Nom lisible.
    pub name: String,
    /// Icone optionnelle.
    pub icon: Option<String>,
    /// Couleur hex optionnelle.
    pub color: Option<String>,
    /// Categorie parente (hierarchie).
    pub parent_id: Option<String>,
    /// Vrai si categorie de revenu.
    pub is_income: bool,
    /// Ordre d'affichage.
    pub sort_order: i32,
}

/// Categories Purse par defaut.
pub const DEFAULT_PURSE_CATEGORIES: &[(&str, &str, bool)] = &[
    ("Alimentation", "🍕", false),
    ("Transport", "🚌", false),
    ("Logement", "🏠", false),
    ("Sante", "🏥", false),
    ("Loisirs", "🎮", false),
    ("Vetements", "👕", false),
    ("Technologie", "📱", false),
    ("Epargne", "💰", false),
    ("Revenus", "💵", true),
    ("Divers", "📦", false),
];

// ─── Mouvement ───────────────────────────────────────────────

/// Mouvement financier (depense ou revenu).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    /// Identifiant unique.
    pub id: String,
    /// Montant signe (positif=revenu, negatif=depense).
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Categorie associee.
    pub category: Option<CategoryRef>,
    /// Description libre.
    pub description: Option<String>,
    /// Date du mouvement (ISO 8601).
    pub movement_date: String,
    /// Budget occasionnel lie.
    pub budget: Option<BudgetRef>,
    /// Service source (manual, jayfestival, jayrdv...).
    pub source_service: Option<String>,
    /// Reference contexte metier.
    pub context_ref: Option<String>,
    /// Horodatage creation.
    pub created_at: String,
}

/// Reference legere vers une categorie.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryRef {
    /// Identifiant categorie.
    pub id: String,
    /// Nom lisible.
    pub name: String,
}

/// Reference legere vers un budget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetRef {
    /// Identifiant budget.
    pub id: String,
    /// Nom du budget.
    pub name: String,
}

/// Formulaire de creation de mouvement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMovement {
    /// Montant signe.
    pub amount: f64,
    /// Categorie cible.
    pub category_id: Option<String>,
    /// Description libre.
    pub description: Option<String>,
    /// Date du mouvement.
    pub movement_date: String,
    /// Budget occasionnel cible.
    pub budget_id: Option<String>,
}

// ─── Budget occasionnel ──────────────────────────────────────

/// Budget occasionnel (Noel, vacances, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccasionalBudget {
    /// Identifiant unique.
    pub id: String,
    /// Nom du budget.
    pub name: String,
    /// Montant cible.
    pub target: f64,
    /// Cumul depenses.
    pub spent: f64,
    /// Restant (calcule : target - spent).
    pub remaining: f64,
    /// Progression (calcule : spent / target * 100).
    pub progress_pct: f64,
    /// Devise.
    pub currency: String,
    /// Date de debut.
    pub start_date: String,
    /// Date de fin optionnelle.
    pub end_date: Option<String>,
    /// Statut du budget.
    pub status: BudgetStatus,
    /// Nombre de mouvements affectes.
    pub movements_count: u32,
}

/// Formulaire de creation de budget occasionnel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBudget {
    /// Nom du budget.
    pub name: String,
    /// Montant cible.
    pub target: f64,
    /// Date de debut.
    pub start_date: String,
    /// Date de fin optionnelle.
    pub end_date: Option<String>,
}

/// Statut d'un budget occasionnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetStatus {
    /// En cours.
    Active,
    /// Clos (termine).
    Closed,
    /// Annule.
    Cancelled,
}

impl BudgetStatus {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "closed" => Self::Closed,
            "cancelled" => Self::Cancelled,
            _ => Self::Active,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Closed => "closed",
            Self::Cancelled => "cancelled",
        }
    }
}

// ─── Objectif ────────────────────────────────────────────────

/// Objectif d'epargne ou de limite de depense.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Identifiant unique.
    pub id: String,
    /// Nom de l'objectif.
    pub name: String,
    /// Montant cible.
    pub target: f64,
    /// Montant actuel atteint.
    pub current: f64,
    /// Progression (calcule : current / target * 100).
    pub progress_pct: f64,
    /// Devise.
    pub currency: String,
    /// Type d'objectif.
    pub goal_type: GoalType,
    /// Echeance optionnelle.
    pub deadline: Option<String>,
    /// Statut de l'objectif.
    pub status: GoalStatus,
    /// Montant mensuel requis pour atteindre l'objectif (calcule si echeance).
    pub monthly_required: Option<f64>,
    /// Indique si l'objectif est en bonne voie (calcule).
    pub is_on_track: bool,
}

/// Formulaire de creation d'objectif.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGoal {
    /// Nom de l'objectif.
    pub name: String,
    /// Montant cible.
    pub target: f64,
    /// Type d'objectif.
    pub goal_type: GoalType,
    /// Echeance optionnelle.
    pub deadline: Option<String>,
}

/// Type d'objectif.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalType {
    /// Epargne.
    Savings,
    /// Limite de depense.
    SpendingLimit,
}

impl GoalType {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "spending_limit" => Self::SpendingLimit,
            _ => Self::Savings,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Savings => "savings",
            Self::SpendingLimit => "spending_limit",
        }
    }
}

/// Statut d'un objectif.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    /// En cours.
    Active,
    /// Atteint.
    Reached,
    /// Echoue.
    Failed,
    /// Annule.
    Cancelled,
}

impl GoalStatus {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "reached" => Self::Reached,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => Self::Active,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Reached => "reached",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

// ─── Transaction recurrente ─────────────────────────────────

/// Transaction recurrente (depense ou revenu periodique).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecurringTransaction {
    /// Identifiant unique.
    pub id: String,
    /// Direction : depense ou revenu.
    pub direction: RecurringDirection,
    /// Montant (toujours positif, le signe est deduit de direction).
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Categorie associee.
    pub category: Option<CategoryRef>,
    /// Description (ex: "Loyer", "Salaire", "Netflix").
    pub description: String,
    /// Frequence de recurrence.
    pub frequency: RecurringFrequency,
    /// Jour du mois (1-31) pour monthly/quarterly/yearly.
    pub day_of_month: Option<i32>,
    /// Jour de la semaine (1=lundi, 7=dimanche) pour weekly/biweekly.
    pub day_of_week: Option<i32>,
    /// Date de debut.
    pub start_date: String,
    /// Date de fin optionnelle (null = illimite).
    pub end_date: Option<String>,
    /// Prochaine echeance calculee.
    pub next_due_date: String,
    /// Actif ou suspendu.
    pub is_active: bool,
    /// Creer automatiquement le mouvement a l'echeance.
    pub auto_create: bool,
    /// Notes libres.
    pub notes: Option<String>,
    /// Horodatage creation.
    pub created_at: String,
}

/// Direction d'une transaction recurrente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurringDirection {
    /// Depense recurrente.
    Expense,
    /// Revenu recurrent.
    Income,
}

impl RecurringDirection {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "income" => Self::Income,
            _ => Self::Expense,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Expense => "expense",
            Self::Income => "income",
        }
    }

    /// Libelle lisible.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Expense => "Depense",
            Self::Income => "Revenu",
        }
    }
}

/// Frequence de recurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurringFrequency {
    /// Hebdomadaire.
    Weekly,
    /// Bi-hebdomadaire (tous les 15 jours).
    Biweekly,
    /// Mensuel.
    Monthly,
    /// Trimestriel.
    Quarterly,
    /// Annuel.
    Yearly,
}

impl RecurringFrequency {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "weekly" => Self::Weekly,
            "biweekly" => Self::Biweekly,
            "quarterly" => Self::Quarterly,
            "yearly" => Self::Yearly,
            _ => Self::Monthly,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Weekly => "weekly",
            Self::Biweekly => "biweekly",
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::Yearly => "yearly",
        }
    }

    /// Libelle lisible.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Weekly => "Hebdomadaire",
            Self::Biweekly => "Bi-hebdomadaire",
            Self::Monthly => "Mensuel",
            Self::Quarterly => "Trimestriel",
            Self::Yearly => "Annuel",
        }
    }

    /// Toutes les frequences disponibles.
    pub const ALL: &'static [RecurringFrequency] = &[
        Self::Weekly,
        Self::Biweekly,
        Self::Monthly,
        Self::Quarterly,
        Self::Yearly,
    ];
}

/// Formulaire de creation de transaction recurrente.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecurring {
    /// Direction (depense / revenu).
    pub direction: RecurringDirection,
    /// Montant (positif).
    pub amount: f64,
    /// Categorie cible.
    pub category_id: Option<String>,
    /// Description.
    pub description: String,
    /// Frequence.
    pub frequency: RecurringFrequency,
    /// Jour du mois (pour monthly/quarterly/yearly).
    pub day_of_month: Option<i32>,
    /// Jour de la semaine (pour weekly/biweekly).
    pub day_of_week: Option<i32>,
    /// Date de debut.
    pub start_date: String,
    /// Date de fin optionnelle.
    pub end_date: Option<String>,
    /// Creer automatiquement les mouvements.
    pub auto_create: bool,
    /// Notes.
    pub notes: Option<String>,
}

// ─── Previsionnel budget ────────────────────────────────────

/// Ligne du previsionnel mensuel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastEntry {
    /// Mois (YYYY-MM).
    pub month: String,
    /// Libelle lisible du mois.
    pub month_label: String,
    /// Total revenus recurrents prevus.
    pub recurring_income: f64,
    /// Total depenses recurrentes prevues.
    pub recurring_expense: f64,
    /// Solde net recurrent (income - expense).
    pub recurring_net: f64,
    /// Solde cumule projete depuis le solde actuel.
    pub projected_balance: f64,
}

/// Vue complete du previsionnel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetForecastView {
    /// Solde actuel reel.
    pub current_balance: f64,
    /// Devise.
    pub currency: String,
    /// Total revenus recurrents mensuels estimes.
    pub monthly_recurring_income: f64,
    /// Total depenses recurrentes mensuelles estimees.
    pub monthly_recurring_expense: f64,
    /// Solde mensuel net recurrent.
    pub monthly_net: f64,
    /// Previsionnel mois par mois (6 ou 12 mois).
    pub forecast: Vec<ForecastEntry>,
    /// Nombre de transactions recurrentes actives.
    pub active_recurring_count: u32,
}

// ─── Alerte ──────────────────────────────────────────────────

/// Configuration d'alerte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Identifiant unique.
    pub id: String,
    /// Type d'alerte.
    pub alert_type: AlertType,
    /// Categorie cible (pour seuil categorie).
    pub category_id: Option<String>,
    /// Objectif cible (pour alerte objectif).
    pub goal_id: Option<String>,
    /// Budget cible (pour alerte budget).
    pub budget_id: Option<String>,
    /// Seuil en EUR.
    pub threshold: Option<f64>,
    /// Frequence d'evaluation.
    pub frequency: AlertFrequency,
    /// Actif ou non.
    pub is_active: bool,
    /// Derniere activation.
    pub last_triggered: Option<String>,
}

/// Type d'alerte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    /// Seuil par categorie.
    CategoryThreshold,
    /// Avertissement objectif.
    GoalWarning,
    /// Avertissement budget.
    BudgetWarning,
}

impl AlertType {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "goal_warning" => Self::GoalWarning,
            "budget_warning" => Self::BudgetWarning,
            _ => Self::CategoryThreshold,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CategoryThreshold => "category_threshold",
            Self::GoalWarning => "goal_warning",
            Self::BudgetWarning => "budget_warning",
        }
    }
}

/// Frequence d'evaluation d'alerte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertFrequency {
    /// Quotidienne.
    Daily,
    /// Hebdomadaire.
    Weekly,
    /// Mensuelle.
    Monthly,
}

impl AlertFrequency {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "daily" => Self::Daily,
            "monthly" => Self::Monthly,
            _ => Self::Weekly,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
        }
    }
}

// ─── Vue Dashboard Purse ─────────────────────────────────────

/// Modele de vue du dashboard Purse.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurseDashboardView {
    /// Solde actuel.
    pub balance: f64,
    /// Revenus du mois.
    pub month_income: f64,
    /// Depenses du mois.
    pub month_expense: f64,
    /// Budget mensuel defini (si configure).
    pub monthly_budget: Option<f64>,
    /// Pourcentage budget utilise.
    pub budget_usage_pct: f64,
    /// Budget restant.
    pub budget_remaining: f64,
    /// Devise.
    pub currency: String,
    /// Repartition par categorie ce mois.
    pub categories_breakdown: Vec<CategoryBreakdown>,
    /// Derniers mouvements (5).
    pub recent_movements: Vec<Movement>,
    /// Resume des objectifs actifs.
    pub active_goals: Vec<GoalSummary>,
    /// Resume des budgets occasionnels actifs.
    pub active_budgets: Vec<BudgetSummary>,
}

/// Repartition par categorie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    /// Nom de la categorie.
    pub category_name: String,
    /// Montant total.
    pub amount: f64,
    /// Pourcentage du total.
    pub percentage: f64,
}

/// Resume d'objectif pour le dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSummary {
    /// Identifiant.
    pub id: String,
    /// Nom.
    pub name: String,
    /// Actuel.
    pub current: f64,
    /// Cible.
    pub target: f64,
    /// Progression.
    pub progress_pct: f64,
}

/// Resume de budget pour le dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetSummary {
    /// Identifiant.
    pub id: String,
    /// Nom.
    pub name: String,
    /// Depense.
    pub spent: f64,
    /// Cible.
    pub target: f64,
    /// Progression.
    pub progress_pct: f64,
}

// ─── Filtres ─────────────────────────────────────────────────

/// Filtres pour la liste de mouvements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MovementFilters {
    /// Periode (YYYY-MM).
    pub month: Option<String>,
    /// Categorie cible.
    pub category_id: Option<String>,
    /// Type : depense, revenu, ou tous.
    pub movement_type: Option<MovementType>,
    /// Budget cible.
    pub budget_id: Option<String>,
    /// Page courante (0-indexed).
    pub page: u32,
    /// Nombre par page.
    pub per_page: u32,
}

/// Type de mouvement pour filtrage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MovementType {
    /// Depense (montant negatif).
    Expense,
    /// Revenu (montant positif).
    Income,
}

/// Resultat pagine de mouvements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementsPage {
    /// Mouvements de la page.
    pub items: Vec<Movement>,
    /// Nombre total de mouvements (tous filtres).
    pub total_count: u32,
    /// Nombre de pages.
    pub total_pages: u32,
    /// Page courante.
    pub current_page: u32,
    /// Total revenus (filtres appliques).
    pub total_income: f64,
    /// Total depenses (filtres appliques).
    pub total_expense: f64,
}
