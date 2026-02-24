use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::task::{Dependency, Task};

/// A Gantt project containing tasks, dependencies, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Schema version — used to detect old files and migrate defaults. v1 = original, v2 = priority/description/parent, v3 = NaiveDateTime.
    #[serde(default = "default_version")]
    pub version: u32,
    pub name: String,
    pub tasks: Vec<Task>,
    pub dependencies: Vec<Dependency>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

fn default_version() -> u32 {
    1
}

impl Default for Project {
    fn default() -> Self {
        Self {
            version: 3,
            name: "Untitled Project".to_string(),
            tasks: Vec::new(),
            dependencies: Vec::new(),
            created: Utc::now(),
            modified: Utc::now(),
        }
    }
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Touch the modified timestamp.
    pub fn touch(&mut self) {
        self.modified = Utc::now();
    }

    /// Recalculate every parent task's start/end/progress from its children.
    /// Call after any mutation that may change child dates or progress.
    pub fn recalculate_parent_dates(&mut self) {
        // Collect parent IDs that have children.
        let parent_ids: Vec<uuid::Uuid> = self
            .tasks
            .iter()
            .filter_map(|t| t.parent_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for pid in parent_ids {
            let children: Vec<_> = self
                .tasks
                .iter()
                .filter(|t| t.parent_id == Some(pid))
                .cloned()
                .collect();

            if children.is_empty() {
                continue;
            }

            let new_start = children.iter().map(|t| t.start).min().unwrap();
            let new_end   = children.iter().map(|t| t.end).max().unwrap();
            let new_prog  = children.iter().map(|t| t.progress).sum::<f32>()
                / children.len() as f32;

            if let Some(parent) = self.tasks.iter_mut().find(|t| t.id == pid) {
                parent.start    = new_start;
                parent.end      = new_end;
                parent.progress = new_prog;
            }
        }
    }

    /// Re-order tasks so every parent is immediately followed by its children.
    /// Top-level tasks keep their relative order; children keep their relative
    /// order within each group.
    pub fn sort_tasks_grouped(&mut self) {
        let mut result: Vec<super::task::Task> = Vec::with_capacity(self.tasks.len());
        // Separate top-level items preserving their order.
        let top_level: Vec<_> = self
            .tasks
            .iter()
            .filter(|t| t.parent_id.is_none())
            .cloned()
            .collect();

        for parent in top_level {
            let pid = parent.id;
            result.push(parent);
            // Append children in their current relative order.
            let children: Vec<_> = self
                .tasks
                .iter()
                .filter(|t| t.parent_id == Some(pid))
                .cloned()
                .collect();
            result.extend(children);
        }

        // Any orphaned tasks (parent_id set but parent not found) go at the end.
        for t in &self.tasks {
            if!result.iter().any(|r| r.id == t.id) {
                result.push(t.clone());
            }
        }

        self.tasks = result;
    }
}
