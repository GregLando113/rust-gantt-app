use crate::model::task::TaskPriority;
use crate::ui::theme;
use egui::{RichText, Ui};

/// Active filter state used to decide which tasks are visible.
#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct FilterState {
    pub search: String,
    pub priority: Option<TaskPriority>,
    pub only_overdue: bool,
    pub only_in_progress: bool,
}

impl FilterState {
    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        !self.search.is_empty()
            || self.priority.is_some()
            || self.only_overdue
            || self.only_in_progress
    }
}

/// Render the filter / search bar.
/// Returns true if the filter state changed.
pub fn show_filter_bar(
    search_query: &mut String,
    filter_priority: &mut Option<TaskPriority>,
    ui: &mut Ui,
) -> bool {
    let mut changed = false;

    // Capture available width BEFORE entering the horizontal layout to avoid
    // a feedback loop where widget sizes change available_width each frame.
    let avail = ui.available_width();
    let combo_w = 100.0;
    let clear_w = 18.0;
    let spacing = ui.spacing().item_spacing.x * 2.0 + 6.0;
    let has_filter = !search_query.is_empty() || filter_priority.is_some();
    let search_w = (avail - combo_w - spacing - if has_filter { clear_w + 4.0 } else { 0.0 })
        .max(40.0);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.visuals_mut().extreme_bg_color = theme::bg_field();

        // Search box — fixed width derived from panel width snapshot above
        let search_resp = ui.add_sized(
            [search_w, 22.0],
            egui::TextEdit::singleline(search_query)
                .hint_text("🔍 Search…")
                .font(egui::FontId::proportional(11.0))
                .text_color(theme::text_secondary()),
        );
        if search_resp.changed() {
            changed = true;
        }

        // Priority filter combo — fixed width
        let pri_label = match filter_priority {
            None => "Priority".to_string(),
            Some(p) => format!("{} {}", p.icon(), p.label()),
        };
        egui::ComboBox::from_id_salt("filter_priority_combo")
            .selected_text(RichText::new(&pri_label).size(11.0))
            .width(combo_w)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(filter_priority.is_none(), "— All —")
                    .clicked()
                {
                    *filter_priority = None;
                    changed = true;
                }
                for p in TaskPriority::all() {
                    let lbl = format!("{} {}", p.icon(), p.label());
                    if ui
                        .selectable_label(*filter_priority == Some(*p), &lbl)
                        .clicked()
                    {
                        *filter_priority = Some(*p);
                        changed = true;
                    }
                }
            });

        // Clear button — only visible when a filter is active
        if has_filter {
            if ui
                .add(
                    egui::Button::new(RichText::new(egui_phosphor::regular::X).size(10.0).color(theme::text_dim()))
                        .frame(false),
                )
                .on_hover_text("Clear filters")
                .clicked()
            {
                search_query.clear();
                *filter_priority = None;
                changed = true;
            }
        }
    });

    changed
}

/// Returns true if a task matches the current search/filter.
pub fn task_matches(
    name: &str,
    description: &str,
    priority: TaskPriority,
    search: &str,
    filter_priority: Option<TaskPriority>,
) -> bool {
    // Priority filter
    if let Some(fp) = filter_priority {
        if priority != fp {
            return false;
        }
    }

    // Text search (case-insensitive)
    if !search.is_empty() {
        let query = search.to_lowercase();
        if !name.to_lowercase().contains(&query) && !description.to_lowercase().contains(&query) {
            return false;
        }
    }

    true
}
