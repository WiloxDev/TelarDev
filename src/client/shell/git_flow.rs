//! Modelo de dominio puro para el flujo visual continuo de Git y GitHub (Git Flow Timeline).
//! Diseñado sin dependencias de I/O, sin async y sin mutaciones durante el render.

/// Operaciones y conceptos visuales del ciclo de vida de Git y GitHub.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitFlowStep {
    /// 01 COMMIT: Agrupa archivos modificados en una instantánea en el historial.
    Commit {
        hash: String,
        branch: String,
        message: String,
        files: Vec<String>,
        upstream: String,
    },
    /// 02 BRANCH: Línea de desarrollo paralela que se bifurca desde la rama base.
    Branch {
        name: String,
        ahead: usize,
        behind: usize,
    },
    /// 06 STASH: Cambios locales pausados temporalmente en la pila.
    Stash {
        index: usize,
        file_hint: String,
    },
    /// 07 TAG: Etiqueta inamovible anclada sobre un commit o rama específica.
    Tag {
        name: String,
        target_ref: String,
    },
    /// 04 REBASE: Re-aplicación de commits de la rama secundaria sobre la principal.
    Rebase {
        in_progress: bool,
        step: usize,
        total: usize,
        commit: String,
        has_conflict: bool,
    },
    /// 05 CHERRY-PICK: Copia de un commit específico insertándose en la rama actual.
    CherryPick {
        commit_hash: String,
        note: String,
    },
    /// 03 MERGE: Fusión de la rama secundaria de vuelta a la principal con commit de unión.
    Merge {
        branch: String,
        target: String,
        is_merge_commit: bool,
    },
    /// 08 FORK: Clonación remota y sincronización entre repositorio autor y propio.
    Fork {
        upstream_repo: String,
        my_repo: String,
        ahead_commits: usize,
    },
    /// 09 PULL REQUEST: Solicitud de revisión y balance de líneas en GitHub.
    PullRequest {
        number: u32,
        title: String,
        delta: (usize, usize),
        approved: bool,
    },
    /// 10 STACKED PRs: Pila secuencial de PRs divididos y encadenados.
    StackedPrs {
        prs: Vec<u32>,
        current_pr: u32,
    },
}

// Public helpers for inspecting step metadata during live background event ingestion.
#[allow(dead_code)]
impl GitFlowStep {
    /// Número de paso canónico de dos dígitos (según infografía x3.jpg / q111.png).
    pub fn step_number(&self) -> &'static str {
        match self {
            Self::Commit { .. } => "01",
            Self::Branch { .. } => "02",
            Self::Merge { .. } => "03",
            Self::Rebase { .. } => "04",
            Self::CherryPick { .. } => "05",
            Self::Stash { .. } => "06",
            Self::Tag { .. } => "07",
            Self::Fork { .. } => "08",
            Self::PullRequest { .. } => "09",
            Self::StackedPrs { .. } => "10",
        }
    }

    /// Nombre de la operación en mayúsculas para la cabecera del nodo.
    pub fn step_name(&self) -> &'static str {
        match self {
            Self::Commit { .. } => "COMMIT",
            Self::Branch { .. } => "BRANCH",
            Self::Merge { .. } => "MERGE",
            Self::Rebase { .. } => "REBASE",
            Self::CherryPick { .. } => "CHERRY-PICK",
            Self::Stash { .. } => "STASH",
            Self::Tag { .. } => "TAG",
            Self::Fork { .. } => "FORK",
            Self::PullRequest { .. } => "PULL REQUEST",
            Self::StackedPrs { .. } => "STACKED PRs",
        }
    }

    /// Indica si el paso representa una operación en curso o requiere atención inmediata.
    pub fn is_active_or_alert(&self) -> bool {
        match self {
            Self::Rebase { in_progress, has_conflict, .. } => *in_progress || *has_conflict,
            Self::Merge { is_merge_commit, .. } => !*is_merge_commit,
            Self::PullRequest { approved, .. } => !*approved,
            _ => false,
        }
    }
}

/// Estado inmutable del canvas visual de Git Flow preparado para renderizado puro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitFlowCanvasSnapshot {
    /// Lista ordenada cronológicamente de arriba (TOP: antiguo) a abajo (BOTTOM: actual).
    pub steps: Vec<GitFlowStep>,
    /// Desplazamiento vertical en filas dentro del viewport.
    pub scroll_offset: usize,
    /// Mensaje de alerta o conflicto activo (si lo hubiere).
    pub active_alert: Option<String>,
}

impl Default for GitFlowCanvasSnapshot {
    fn default() -> Self {
        Self::sample_timeline()
    }
}

impl GitFlowCanvasSnapshot {
    /// Crea un snapshot vacío listo para poblar desde el repositorio.
    // Constructor used when initializing fresh repository snapshots in background worker.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            steps: Vec::new(),
            scroll_offset: 0,
            active_alert: None,
        }
    }

    /// Genera la línea de tiempo de ejemplo idéntica a la infografía x3.jpg para pruebas y preview.
    pub fn sample_timeline() -> Self {
        Self {
            steps: vec![
                GitFlowStep::Commit {
                    hash: "5f074e5".into(),
                    branch: "login".into(),
                    message: "feat: login con google".into(),
                    files: vec!["D".into(), "D".into()],
                    upstream: "origin/main".into(),
                },
                GitFlowStep::Branch {
                    name: "login".into(),
                    ahead: 2,
                    behind: 0,
                },
                GitFlowStep::Stash {
                    index: 0,
                    file_hint: "wip-login.rs".into(),
                },
                GitFlowStep::Tag {
                    name: "v1.0.0".into(),
                    target_ref: "main".into(),
                },
                GitFlowStep::Rebase {
                    in_progress: false,
                    step: 2,
                    total: 2,
                    commit: "A', B'".into(),
                    has_conflict: false,
                },
                GitFlowStep::CherryPick {
                    commit_hash: "a3f9c1e".into(),
                    note: "copia commit hacia abajo".into(),
                },
                GitFlowStep::Merge {
                    branch: "login".into(),
                    target: "main".into(),
                    is_merge_commit: true,
                },
                GitFlowStep::Fork {
                    upstream_repo: "autor/repo".into(),
                    my_repo: "tú/repo".into(),
                    ahead_commits: 1,
                },
                GitFlowStep::PullRequest {
                    number: 42,
                    title: "Arregla login".into(),
                    delta: (12, 3),
                    approved: true,
                },
                GitFlowStep::StackedPrs {
                    prs: vec![41, 42, 43],
                    current_pr: 42,
                },
            ],
            scroll_offset: 0,
            active_alert: None,
        }
    }

    /// Desplaza el viewport hacia abajo (hacia eventos más recientes).
    pub fn scroll_down(&mut self, max_rows: usize) {
        if self.scroll_offset < max_rows {
            self.scroll_offset = self.scroll_offset.saturating_add(1);
        }
    }

    /// Desplaza el viewport hacia arriba (hacia eventos más antiguos).
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Restablece el scroll al inicio (TOP).
    // Navigation helper invoked on keybinding 'g' in git flow inspector view.
    #[allow(dead_code)]
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_flow_steps_canonical_metadata() {
        let commit = GitFlowStep::Commit {
            hash: "5f074e5".into(),
            branch: "login".into(),
            message: "test".into(),
            files: vec!["A".into()],
            upstream: "origin/main".into(),
        };
        assert_eq!(commit.step_number(), "01");
        assert_eq!(commit.step_name(), "COMMIT");
        assert!(!commit.is_active_or_alert());

        let branch = GitFlowStep::Branch {
            name: "login".into(),
            ahead: 2,
            behind: 0,
        };
        assert_eq!(branch.step_number(), "02");
        assert_eq!(branch.step_name(), "BRANCH");

        let rebase_conflict = GitFlowStep::Rebase {
            in_progress: true,
            step: 1,
            total: 3,
            commit: "deadbeef".into(),
            has_conflict: true,
        };
        assert_eq!(rebase_conflict.step_number(), "04");
        assert_eq!(rebase_conflict.step_name(), "REBASE");
        assert!(rebase_conflict.is_active_or_alert());

        let pr = GitFlowStep::PullRequest {
            number: 42,
            title: "fix".into(),
            delta: (10, 2),
            approved: true,
        };
        assert_eq!(pr.step_number(), "09");
        assert_eq!(pr.step_name(), "PULL REQUEST");
    }

    #[test]
    fn test_sample_timeline_contains_all_ten_concepts() {
        let sample = GitFlowCanvasSnapshot::sample_timeline();
        assert_eq!(sample.steps.len(), 10);

        let numbers: Vec<&'static str> = sample.steps.iter().map(|s| s.step_number()).collect();
        assert_eq!(
            numbers,
            vec!["01", "02", "06", "07", "04", "05", "03", "08", "09", "10"]
        );
    }

    #[test]
    fn test_scroll_navigation_bounds() {
        let mut snapshot = GitFlowCanvasSnapshot::sample_timeline();
        assert_eq!(snapshot.scroll_offset, 0);

        snapshot.scroll_down(5);
        assert_eq!(snapshot.scroll_offset, 1);
        snapshot.scroll_down(5);
        assert_eq!(snapshot.scroll_offset, 2);

        snapshot.scroll_up();
        assert_eq!(snapshot.scroll_offset, 1);
        snapshot.scroll_to_top();
        assert_eq!(snapshot.scroll_offset, 0);
        snapshot.scroll_up();
        assert_eq!(snapshot.scroll_offset, 0);
    }
}
