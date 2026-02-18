use chrono::NaiveDate;
use egui::Color32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskPriority {
    #[default]
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl TaskPriority {
    pub fn label(self) -> &'static str {
        match self {
            TaskPriority::None => "—",
            TaskPriority::Low => "Low",
            TaskPriority::Medium => "Medium",
            TaskPriority::High => "High",
            TaskPriority::Critical => "Critical",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            TaskPriority::None     => "",
            TaskPriority::Low      => egui_phosphor::regular::ARROW_DOWN,
            TaskPriority::Medium   => egui_phosphor::regular::EQUALS,
            TaskPriority::High     => egui_phosphor::regular::ARROW_UP,
            TaskPriority::Critical => egui_phosphor::regular::WARNING,
        }
    }

    pub fn all() -> &'static [TaskPriority] {
        &[
            TaskPriority::None,
            TaskPriority::Low,
            TaskPriority::Medium,
            TaskPriority::High,
            TaskPriority::Critical,
        ]
    }
}

/// Represents the type of dependency between two tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DependencyKind {
    #[default]
    FinishToStart,
    StartToStart,
    FinishToFinish,
    StartToFinish,
}

impl DependencyKind {
    pub fn short_label(self) -> &'static str {
        match self {
            DependencyKind::FinishToStart => "FS",
            DependencyKind::StartToStart => "SS",
            DependencyKind::FinishToFinish => "FF",
            DependencyKind::StartToFinish => "SF",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            DependencyKind::FinishToStart  => "Finish-to-Start (FS): successor can't start until this task finishes",
            DependencyKind::StartToStart   => "Start-to-Start (SS): successor can't start until this task starts",
            DependencyKind::FinishToFinish => "Finish-to-Finish (FF): successor can't finish until this task finishes",
            DependencyKind::StartToFinish  => "Start-to-Finish (SF): successor can't finish until this task starts",
        }
    }

    #[allow(dead_code)]
    pub fn all() -> &'static [DependencyKind] {
        &[
            DependencyKind::FinishToStart,
            DependencyKind::StartToStart,
            DependencyKind::FinishToFinish,
            DependencyKind::StartToFinish,
        ]
    }
}

/// A dependency link between two tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub from_task: Uuid,
    pub to_task: Uuid,
    #[serde(default)]
    pub kind: DependencyKind,
}

/// A single task or milestone in the Gantt chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
    /// Progress from 0.0 (not started) to 1.0 (complete).
    pub progress: f32,
    /// Optional group/category name (legacy, kept for compat).
    #[serde(default)]
    pub group: Option<String>,
    /// Parent task id for hierarchy/phases.
    #[serde(default)]
    pub parent_id: Option<Uuid>,
    /// Whether this parent task's children are collapsed.
    #[serde(default)]
    pub collapsed: bool,
    /// Priority level.
    #[serde(default)]
    pub priority: TaskPriority,
    /// Optional description / notes.
    #[serde(default)]
    pub description: String,
    /// Display color for the task bar (stored as RGBA).
    #[serde(with = "color_serde")]
    pub color: Color32,
    /// If true, this is a milestone (rendered as a diamond, zero-duration).
    pub is_milestone: bool,
}

impl Task {
    /// Create a new task with sensible defaults.
    pub fn new(name: impl Into<String>, start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            start,
            end,
            progress: 0.0,
            group: None,
            parent_id: None,
            collapsed: false,
            priority: TaskPriority::None,
            description: String::new(),
            color: Color32::from_rgb(70, 130, 180), // Steel blue
            is_milestone: false,
        }
    }

    /// Create a new milestone.
    pub fn new_milestone(name: impl Into<String>, date: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            start: date,
            end: date,
            progress: 0.0,
            group: None,
            parent_id: None,
            collapsed: false,
            priority: TaskPriority::None,
            description: String::new(),
            color: Color32::from_rgb(255, 165, 0), // Orange
            is_milestone: true,
        }
    }

    /// Returns true if this task has any children in the given task list.
    pub fn has_children(&self, tasks: &[Task]) -> bool {
        tasks.iter().any(|t| t.parent_id == Some(self.id))
    }

    /// Returns the IDs of all direct children of this task.
    pub fn children_ids<'a>(&self, tasks: &'a [Task]) -> Vec<&'a Task> {
        tasks.iter().filter(|t| t.parent_id == Some(self.id)).collect()
    }
}

/// Serde helper for `Color32`.
mod color_serde {
    use egui::Color32;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rgba = [color.r(), color.g(), color.b(), color.a()];
        rgba.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rgba: [u8; 4] = Deserialize::deserialize(deserializer)?;
        Ok(Color32::from_rgba_premultiplied(
            rgba[0], rgba[1], rgba[2], rgba[3],
        ))
    }
}
