use super::task::{Dependency, Task};

const MAX_HISTORY: usize = 50;

/// A snapshot of the mutable project data (tasks + dependencies).
#[derive(Clone)]
pub struct ProjectSnapshot {
    pub tasks: Vec<Task>,
    pub dependencies: Vec<Dependency>,
}

/// Undo/redo stack for project mutations.
pub struct UndoHistory {
    past: Vec<ProjectSnapshot>,
    future: Vec<ProjectSnapshot>,
}

impl UndoHistory {
    pub fn new() -> Self {
        Self {
            past: Vec::new(),
            future: Vec::new(),
        }
    }

    /// Push a snapshot of the current state before a mutation is applied.
    pub fn push(&mut self, tasks: &[Task], dependencies: &[Dependency]) {
        if self.past.len() >= MAX_HISTORY {
            self.past.remove(0);
        }
        self.past.push(ProjectSnapshot {
            tasks: tasks.to_vec(),
            dependencies: dependencies.to_vec(),
        });
        // Any new action clears the redo stack.
        self.future.clear();
    }

    /// Undo: returns the previous snapshot (state to restore), saving the current state for redo.
    pub fn undo(
        &mut self,
        current_tasks: &[Task],
        current_deps: &[Dependency],
    ) -> Option<ProjectSnapshot> {
        let snapshot = self.past.pop()?;
        self.future.push(ProjectSnapshot {
            tasks: current_tasks.to_vec(),
            dependencies: current_deps.to_vec(),
        });
        Some(snapshot)
    }

    /// Redo: returns the next snapshot, saving current state back to undo stack.
    pub fn redo(
        &mut self,
        current_tasks: &[Task],
        current_deps: &[Dependency],
    ) -> Option<ProjectSnapshot> {
        let snapshot = self.future.pop()?;
        self.past.push(ProjectSnapshot {
            tasks: current_tasks.to_vec(),
            dependencies: current_deps.to_vec(),
        });
        Some(snapshot)
    }

    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    pub fn clear(&mut self) {
        self.past.clear();
        self.future.clear();
    }
}
