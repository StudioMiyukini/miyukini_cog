//! PurseService — logique metier budget personnel.
//!
//! Responsabilites : dashboard, mouvements, categories, budgets, objectifs.
//! Contrats : CK-OP-01 (dashboard), CK-OP-02 (mouvements), CK-OP-03 (budgets).

use crate::data::{DbError, JayKontaDb};
use crate::domain::purse::{
    BudgetSummary, CreateBudget, CreateGoal, CreateMovement, CreateRecurring,
    Goal, GoalSummary, MovementFilters, MovementsPage, OccasionalBudget,
    PurseDashboardView, Category, RecurringTransaction, RecurringFrequency,
    BudgetForecastView, ForecastEntry,
};
use chrono::{Datelike, Local};
use std::sync::Arc;

/// Service metier Purse.
pub struct PurseService {
    db: Arc<JayKontaDb>,
    /// Identifiant du compte Purse actif.
    account_id: String,
}

impl PurseService {
    /// Cree le service pour un utilisateur donne.
    pub fn new(db: Arc<JayKontaDb>, user_id: &str) -> Result<Self, DbError> {
        let account_id = db.ensure_account("purse", user_id, "Budget personnel")?;
        db.ensure_default_categories(&account_id)?;
        Ok(Self { db, account_id })
    }

    /// Identifiant du compte actif.
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    // ─── Dashboard ───────────────────────────────────────────

    /// Construit la vue dashboard Purse complete (CK-OP-01).
    pub fn dashboard(&self) -> Result<PurseDashboardView, DbError> {
        let month = Local::now().format("%Y-%m").to_string();
        let balance = self.db.solde_purse()?;
        let (month_income, month_expense) = self.db.purse_month_totals(&self.account_id, &month)?;
        let categories_breakdown = self.db.categories_breakdown_purse(&self.account_id, &month)?;

        // Derniers mouvements (5)
        let filters = MovementFilters {
            per_page: 5,
            ..Default::default()
        };
        let (recent_movements, _) = self.db.list_purse_movements(&self.account_id, &filters)?;

        // Objectifs actifs
        let goals = self.db.list_goals(&self.account_id)?;
        let active_goals: Vec<GoalSummary> = goals
            .iter()
            .filter(|g| g.status == crate::domain::purse::GoalStatus::Active)
            .map(|g| GoalSummary {
                id: g.id.clone(),
                name: g.name.clone(),
                current: g.current,
                target: g.target,
                progress_pct: g.progress_pct,
            })
            .collect();

        // Budgets actifs
        let budgets = self.db.list_budgets(&self.account_id)?;
        let active_budgets: Vec<BudgetSummary> = budgets
            .iter()
            .filter(|b| b.status == crate::domain::purse::BudgetStatus::Active)
            .map(|b| BudgetSummary {
                id: b.id.clone(),
                name: b.name.clone(),
                spent: b.spent,
                target: b.target,
                progress_pct: b.progress_pct,
            })
            .collect();

        // Budget mensuel (estimation simple : total revenus du mois)
        let budget_usage_pct = if month_income > 0.0 {
            (month_expense / month_income * 100.0).min(100.0)
        } else {
            0.0
        };
        let budget_remaining = (month_income - month_expense).max(0.0);

        Ok(PurseDashboardView {
            balance,
            month_income,
            month_expense,
            monthly_budget: if month_income > 0.0 { Some(month_income) } else { None },
            budget_usage_pct,
            budget_remaining,
            currency: "EUR".to_string(),
            categories_breakdown,
            recent_movements,
            active_goals,
            active_budgets,
        })
    }

    // ─── Mouvements ──────────────────────────────────────────

    /// Ajoute un mouvement Purse (CK-OP-02).
    /// Performance cible : < 2s, max 3 actions (NFR-PUR-05, NFR-PUR-06).
    pub fn add_movement(&self, cmd: CreateMovement) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        self.db.create_purse_movement(
            &self.account_id,
            &id,
            cmd.amount,
            cmd.category_id.as_deref(),
            cmd.description.as_deref(),
            &cmd.movement_date,
            cmd.budget_id.as_deref(),
        )?;
        Ok(id)
    }

    /// Liste les mouvements Purse avec filtres et pagination (CK-TK-11).
    pub fn list_movements(&self, filters: &MovementFilters) -> Result<MovementsPage, DbError> {
        let per_page = if filters.per_page == 0 { 20 } else { filters.per_page };
        let (items, total_count) = self.db.list_purse_movements(&self.account_id, filters)?;
        let total_pages = (total_count + per_page - 1) / per_page;

        // Calcul totaux revenus/depenses sur les items de la page
        let total_income: f64 = items.iter().filter(|m| m.amount > 0.0).map(|m| m.amount).sum();
        let total_expense: f64 = items.iter().filter(|m| m.amount < 0.0).map(|m| m.amount.abs()).sum();

        Ok(MovementsPage {
            items,
            total_count,
            total_pages,
            current_page: filters.page,
            total_income,
            total_expense,
        })
    }

    // ─── Categories ──────────────────────────────────────────

    /// Liste les categories du compte Purse.
    pub fn list_categories(&self) -> Result<Vec<Category>, DbError> {
        self.db.list_categories(&self.account_id)
    }

    // ─── Budgets occasionnels ────────────────────────────────

    /// Liste les budgets occasionnels (CK-OP-03).
    pub fn list_budgets(&self) -> Result<Vec<OccasionalBudget>, DbError> {
        self.db.list_budgets(&self.account_id)
    }

    /// Cree un budget occasionnel.
    pub fn create_budget(&self, cmd: CreateBudget) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        self.db.create_budget(
            &id,
            &self.account_id,
            &cmd.name,
            cmd.target,
            &cmd.start_date,
            cmd.end_date.as_deref(),
        )?;
        Ok(id)
    }

    /// Cloture un budget.
    pub fn close_budget(&self, budget_id: &str) -> Result<(), DbError> {
        self.db.close_budget(budget_id)
    }

    // ─── Objectifs ───────────────────────────────────────────

    /// Liste les objectifs (CK-TK-61).
    pub fn list_goals(&self) -> Result<Vec<Goal>, DbError> {
        self.db.list_goals(&self.account_id)
    }

    /// Cree un objectif.
    pub fn create_goal(&self, cmd: CreateGoal) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        self.db.create_goal(
            &id,
            &self.account_id,
            &cmd.name,
            cmd.target,
            cmd.goal_type.as_str(),
            cmd.deadline.as_deref(),
        )?;
        Ok(id)
    }

    /// Ajoute un versement vers un objectif.
    pub fn add_goal_contribution(&self, goal_id: &str, amount: f64) -> Result<(), DbError> {
        self.db.update_goal_progress(goal_id, amount)
    }

    // ─── Transactions recurrentes ─────────────────────────────

    /// Liste les transactions recurrentes actives.
    pub fn list_recurring(&self, active_only: bool) -> Result<Vec<RecurringTransaction>, DbError> {
        self.db.list_recurring(&self.account_id, active_only)
    }

    /// Cree une transaction recurrente.
    pub fn create_recurring(&self, cmd: CreateRecurring) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        // Calculer la prochaine echeance depuis start_date
        let next_due = Self::compute_next_due(&cmd.start_date, &cmd.frequency, cmd.day_of_month);
        self.db.create_recurring(
            &id,
            &self.account_id,
            cmd.direction.as_str(),
            cmd.amount,
            cmd.category_id.as_deref(),
            &cmd.description,
            cmd.frequency.as_str(),
            cmd.day_of_month,
            cmd.day_of_week,
            &cmd.start_date,
            cmd.end_date.as_deref(),
            &next_due,
            cmd.auto_create,
            cmd.notes.as_deref(),
        )?;
        Ok(id)
    }

    /// Active ou desactive une transaction recurrente.
    pub fn toggle_recurring(&self, recurring_id: &str, active: bool) -> Result<(), DbError> {
        self.db.toggle_recurring(recurring_id, active)
    }

    /// Supprime une transaction recurrente.
    pub fn delete_recurring(&self, recurring_id: &str) -> Result<(), DbError> {
        self.db.delete_recurring(recurring_id)
    }

    // ─── Previsionnel budget ──────────────────────────────────

    /// Construit la vue previsionnelle sur N mois.
    pub fn forecast(&self, months_ahead: u32) -> Result<BudgetForecastView, DbError> {
        let balance = self.db.solde_purse()?;
        let recurring = self.db.list_recurring(&self.account_id, true)?;
        let (monthly_recurring_income, monthly_recurring_expense) =
            self.db.recurring_monthly_totals(&self.account_id)?;
        let monthly_net = monthly_recurring_income - monthly_recurring_expense;

        // Generer le previsionnel mois par mois
        let today = Local::now().date_naive();
        let mut forecast = Vec::with_capacity(months_ahead as usize);
        let mut cumulative_balance = balance;

        for i in 0..months_ahead {
            let target_date = Self::add_months_naive(today, i + 1);
            let month_str = target_date.format("%Y-%m").to_string();
            let month_label = Self::french_month_label(&month_str);

            // Calculer les recurrents pour ce mois specifique
            let (month_income, month_expense) =
                Self::compute_month_recurring(&recurring, &month_str);

            let net = month_income - month_expense;
            cumulative_balance += net;

            forecast.push(ForecastEntry {
                month: month_str,
                month_label,
                recurring_income: month_income,
                recurring_expense: month_expense,
                recurring_net: net,
                projected_balance: cumulative_balance,
            });
        }

        Ok(BudgetForecastView {
            current_balance: balance,
            currency: "EUR".to_string(),
            monthly_recurring_income,
            monthly_recurring_expense,
            monthly_net,
            forecast,
            active_recurring_count: recurring.len() as u32,
        })
    }

    // ─── Helpers prives ───────────────────────────────────────

    /// Calcule la prochaine echeance a partir d'une date de debut et frequence.
    fn compute_next_due(start: &str, freq: &RecurringFrequency, day_of_month: Option<i32>) -> String {
        use chrono::NaiveDate;
        let today = Local::now().date_naive();
        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d")
            .unwrap_or(today);

        if start_date >= today {
            return start_date.format("%Y-%m-%d").to_string();
        }

        // Avancer par pas de frequence jusqu'a depasser aujourd'hui
        let mut candidate = start_date;
        loop {
            candidate = match freq {
                RecurringFrequency::Weekly => candidate + chrono::Duration::weeks(1),
                RecurringFrequency::Biweekly => candidate + chrono::Duration::weeks(2),
                RecurringFrequency::Monthly => Self::add_months_naive(candidate, 1),
                RecurringFrequency::Quarterly => Self::add_months_naive(candidate, 3),
                RecurringFrequency::Yearly => Self::add_months_naive(candidate, 12),
            };
            if candidate >= today {
                break;
            }
        }

        // Ajuster au jour du mois si specifie (pour monthly/quarterly/yearly)
        if let Some(dom) = day_of_month {
            if matches!(freq, RecurringFrequency::Monthly | RecurringFrequency::Quarterly | RecurringFrequency::Yearly) {
                let year = candidate.year();
                let month = candidate.month();
                let max_day = Self::days_in_month(year, month);
                let day = (dom as u32).min(max_day).max(1);
                if let Some(adjusted) = NaiveDate::from_ymd_opt(year, month, day) {
                    if adjusted >= today {
                        return adjusted.format("%Y-%m-%d").to_string();
                    }
                }
            }
        }

        candidate.format("%Y-%m-%d").to_string()
    }

    /// Ajoute N mois a une NaiveDate (clampe le jour).
    fn add_months_naive(date: chrono::NaiveDate, months: u32) -> chrono::NaiveDate {
        use chrono::NaiveDate;
        let total_months = date.year() as u32 * 12 + date.month() - 1 + months;
        let new_year = (total_months / 12) as i32;
        let new_month = total_months % 12 + 1;
        let max_day = Self::days_in_month(new_year, new_month);
        let day = date.day().min(max_day);
        NaiveDate::from_ymd_opt(new_year, new_month, day).unwrap_or(date)
    }

    /// Nombre de jours dans un mois donne.
    fn days_in_month(year: i32, month: u32) -> u32 {
        use chrono::NaiveDate;
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .map(|d| d.pred_opt().map(|p| p.day()).unwrap_or(28))
        .unwrap_or(28)
    }

    /// Calcule les totaux de recurrents pour un mois donne (YYYY-MM).
    fn compute_month_recurring(recurring: &[RecurringTransaction], month: &str) -> (f64, f64) {
        use chrono::NaiveDate;
        let year: i32 = month[..4].parse().unwrap_or(2026);
        let mon: u32 = month[5..7].parse().unwrap_or(1);
        let month_start = NaiveDate::from_ymd_opt(year, mon, 1);
        let month_end = if mon == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, mon + 1, 1)
        };
        let (Some(m_start), Some(m_end)) = (month_start, month_end) else {
            return (0.0, 0.0);
        };

        let mut income = 0.0;
        let mut expense = 0.0;

        for r in recurring {
            if !r.is_active {
                continue;
            }
            // Verifier que le recurrent est actif pendant ce mois
            let start = NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d")
                .unwrap_or(m_start);
            if start >= m_end {
                continue;
            }
            if let Some(ref end) = r.end_date {
                let end_d = NaiveDate::parse_from_str(end, "%Y-%m-%d")
                    .unwrap_or(m_end);
                if end_d < m_start {
                    continue;
                }
            }

            // Compter le nombre d'occurrences dans ce mois
            let occurrences = Self::occurrences_in_month(r, m_start, m_end);
            let total = r.amount * occurrences as f64;

            match r.direction {
                crate::domain::purse::RecurringDirection::Income => income += total,
                crate::domain::purse::RecurringDirection::Expense => expense += total,
            }
        }

        (income, expense)
    }

    /// Nombre d'occurrences d'un recurrent dans un mois [start, end[.
    fn occurrences_in_month(
        r: &RecurringTransaction,
        _m_start: chrono::NaiveDate,
        _m_end: chrono::NaiveDate,
    ) -> u32 {
        match r.frequency {
            RecurringFrequency::Weekly => 4, // ~4 semaines/mois
            RecurringFrequency::Biweekly => 2,
            RecurringFrequency::Monthly => 1,
            RecurringFrequency::Quarterly => {
                // 1 fois tous les 3 mois
                let month = _m_start.month();
                let start_month = chrono::NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d")
                    .map(|d| d.month())
                    .unwrap_or(1);
                if (month + 12 - start_month) % 3 == 0 { 1 } else { 0 }
            }
            RecurringFrequency::Yearly => {
                let month = _m_start.month();
                let start_month = chrono::NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d")
                    .map(|d| d.month())
                    .unwrap_or(1);
                if month == start_month { 1 } else { 0 }
            }
        }
    }

    /// Libelle francais du mois (ex: "Fev 2026").
    fn french_month_label(month: &str) -> String {
        let mon: u32 = month[5..7].parse().unwrap_or(1);
        let year = &month[..4];
        let name = match mon {
            1 => "Jan",
            2 => "Fev",
            3 => "Mar",
            4 => "Avr",
            5 => "Mai",
            6 => "Juin",
            7 => "Juil",
            8 => "Aout",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "?",
        };
        format!("{name} {year}")
    }
}
