use serde::{Deserialize, Serialize};

/// Debug panel categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebugTab {
    Performance,
    Entities,
    Network,
    Renderer,
    Audio,
}

/// A single debug metric displayed in the overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugMetric {
    pub label: String,
    pub value: String,
    pub warn: bool,
}

/// Debug panel state (dev-only, should not ship in release builds).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugPanel {
    pub visible: bool,
    pub active_tab: DebugTab,
    pub metrics: Vec<DebugMetric>,
    pub log_lines: Vec<String>,
    pub log_capacity: usize,
}

impl Default for DebugPanel {
    fn default() -> Self {
        Self {
            visible: false,
            active_tab: DebugTab::Performance,
            metrics: Vec::new(),
            log_lines: Vec::new(),
            log_capacity: 100,
        }
    }
}

impl DebugPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn set_tab(&mut self, tab: DebugTab) {
        self.active_tab = tab;
    }

    /// Update or insert a metric.
    pub fn set_metric(&mut self, label: impl Into<String>, value: impl Into<String>, warn: bool) {
        let label = label.into();
        if let Some(m) = self.metrics.iter_mut().find(|m| m.label == label) {
            m.value = value.into();
            m.warn = warn;
        } else {
            self.metrics.push(DebugMetric { label, value: value.into(), warn });
        }
    }

    /// Append a log line. Trims oldest if over capacity.
    pub fn log(&mut self, line: impl Into<String>) {
        self.log_lines.push(line.into());
        if self.log_lines.len() > self.log_capacity {
            self.log_lines.remove(0);
        }
    }

    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    pub fn clear_log(&mut self) {
        self.log_lines.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_visibility() {
        let mut panel = DebugPanel::new();
        assert!(!panel.visible);
        panel.toggle();
        assert!(panel.visible);
        panel.toggle();
        assert!(!panel.visible);
    }

    #[test]
    fn metric_update() {
        let mut panel = DebugPanel::new();
        panel.set_metric("FPS", "60", false);
        panel.set_metric("FPS", "59", true);
        assert_eq!(panel.metrics.len(), 1);
        assert_eq!(panel.metrics[0].value, "59");
        assert!(panel.metrics[0].warn);
    }

    #[test]
    fn log_trims_oldest() {
        let mut panel = DebugPanel::new();
        panel.log_capacity = 3;
        for i in 0..5 {
            panel.log(format!("line {i}"));
        }
        assert_eq!(panel.log_lines.len(), 3);
        assert_eq!(panel.log_lines[0], "line 2");
    }
}
