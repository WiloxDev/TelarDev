#[derive(Clone, Copy)]
pub(crate) enum ConfigEdit<'a> {
    Theme(&'a str),
    StatusIndicators(super::StatusIndicatorStyle),
    Sound(bool),
    ToastDelivery(super::ToastDelivery),
    Font(&'a str),
    DockState(bool),
    DockGit(bool),
    DockEngram(bool),
    DockContext(bool),
    DockCpamc(bool),
    DockTools(bool),
    DockMcp(bool),
    DockGitFlow(bool),
    CpamcExperience(super::CpamcExperienceConfig),
    DockVisible(bool),
}

impl ConfigEdit<'_> {
    pub(crate) fn description(self) -> &'static str {
        match self {
            Self::Theme(_) => "theme",
            Self::StatusIndicators(_) => "status indicators",
            Self::Sound(_) => "sound setting",
            Self::ToastDelivery(_) => "toast setting",
            Self::Font(_) => "font preference",
            Self::DockState(_)
            | Self::DockGit(_)
            | Self::DockEngram(_)
            | Self::DockContext(_)
            | Self::DockCpamc(_)
            | Self::DockTools(_)
            | Self::DockMcp(_)
            | Self::DockGitFlow(_)
            | Self::CpamcExperience(_)
            | Self::DockVisible(_) => "sidebar setting",
        }
    }

    pub(crate) fn apply(self, content: &str) -> String {
        match self {
            Self::Theme(name) => {
                let content =
                    super::upsert_section_value(content, "theme", "name", &format!("\"{name}\""));
                super::upsert_section_bool(&content, "theme", "auto_switch", false)
            }
            Self::StatusIndicators(style) => super::upsert_section_value(
                content,
                "ui",
                "status_indicators",
                &format!("\"{}\"", style.as_str()),
            ),
            Self::Sound(enabled) => {
                super::upsert_section_bool(content, "ui.sound", "enabled", enabled)
            }
            Self::ToastDelivery(delivery) => {
                let value = match delivery {
                    super::ToastDelivery::Off => "\"off\"",
                    super::ToastDelivery::Herdr => "\"herdr\"",
                    super::ToastDelivery::Terminal => "\"terminal\"",
                    super::ToastDelivery::System => "\"system\"",
                };
                let content = super::upsert_section_value(content, "ui.toast", "delivery", value);
                super::remove_section_key(&content, "ui.toast", "enabled")
            }
            Self::Font(font_id) => {
                super::upsert_section_value(content, "ui", "font", &format!("\"{font_id}\""))
            }
            Self::DockState(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_state", enabled),
            Self::DockGit(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_git", enabled),
            Self::DockEngram(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_engram", enabled),
            Self::DockContext(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_context", enabled),
            Self::DockCpamc(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_cpamc", enabled),
            Self::DockTools(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_tools", enabled),
            Self::DockMcp(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_mcp", enabled),
            Self::DockGitFlow(enabled) => super::upsert_section_bool(content, "ui", "right_dock_show_git_flow", enabled),
            Self::CpamcExperience(exp) => super::upsert_section_value(content, "ui", "cpamc_experience", &format!("\"{}\"", exp.as_str())),
            Self::DockVisible(enabled) => super::upsert_section_bool(content, "ui", "right_dock_enabled", enabled),
        }
    }
}

pub(crate) fn update_file_at(
    path: &std::path::Path,
    description: &str,
    update: impl FnOnce(&str) -> Result<String, String>,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create config directory: {error}"))?;
    }
    let content = match super::io::read_optional_config(path) {
        Ok(Some(content)) => content,
        Ok(None) => String::new(),
        Err(error) => {
            return Err(format!(
                "failed to read config before saving {description}: {error}"
            ));
        }
    };
    let updated = update(&content)?;
    std::fs::write(path, updated).map_err(|error| format!("failed to save {description}: {error}"))
}

pub(crate) fn write_edit(edit: ConfigEdit<'_>) -> Result<(), String> {
    update_file_at(&super::config_path(), edit.description(), |content| {
        Ok(edit.apply(content))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dock_sections_default_enabled_and_persist_independently() {
        let defaults = crate::config::Config::default();
        assert!(defaults.ui.right_dock_enabled);
        assert!(defaults.ui.right_dock_show_state);
        assert!(defaults.ui.right_dock_show_git);
        assert!(defaults.ui.right_dock_show_engram);
        assert!(defaults.ui.right_dock_show_context);
        assert!(defaults.ui.right_dock_show_cpamc);
        assert!(defaults.ui.right_dock_show_tools);
        assert!(defaults.ui.right_dock_show_mcp);
        assert_eq!(defaults.ui.cpamc_experience, crate::config::CpamcExperienceConfig::AutoSwitcher);
        let text = ConfigEdit::DockState(false).apply("[ui]\nright_dock_show_git = true\n");
        let text = ConfigEdit::DockGit(false).apply(&text);
        let text = ConfigEdit::DockEngram(false).apply(&text);
        let text = ConfigEdit::DockContext(false).apply(&text);
        let text = ConfigEdit::DockCpamc(false).apply(&text);
        let text = ConfigEdit::DockTools(false).apply(&text);
        let text = ConfigEdit::DockMcp(false).apply(&text);
        let text = ConfigEdit::CpamcExperience(crate::config::CpamcExperienceConfig::Totopo27).apply(&text);
        let text = ConfigEdit::DockVisible(false).apply(&text);
        let parsed: crate::config::Config = toml::from_str(&text).unwrap();
        assert!(!parsed.ui.right_dock_enabled);
        assert!(!parsed.ui.right_dock_show_state);
        assert!(!parsed.ui.right_dock_show_git);
        assert!(!parsed.ui.right_dock_show_engram);
        assert!(!parsed.ui.right_dock_show_context);
        assert!(!parsed.ui.right_dock_show_cpamc);
        assert!(!parsed.ui.right_dock_show_tools);
        assert!(!parsed.ui.right_dock_show_mcp);
        assert_eq!(parsed.ui.cpamc_experience, crate::config::CpamcExperienceConfig::Totopo27);
    }

    #[test]
    fn update_file_at_does_not_move_a_leading_bom_into_the_file() {
        let dir = std::env::temp_dir().join(format!("herdr-config-bom-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        std::fs::write(
            &path,
            b"\xEF\xBB\xBF[terminal]\ndefault_shell = \"pwsh.exe\"\n",
        )
        .unwrap();

        update_file_at(&path, "onboarding setting", |content| {
            Ok(crate::config::upsert_top_level_bool(
                content,
                "onboarding",
                false,
            ))
        })
        .unwrap();

        let written = std::fs::read_to_string(&path).unwrap();
        let _ = std::fs::remove_dir_all(dir);

        assert!(
            !written.contains('\u{feff}'),
            "unexpected BOM in {written:?}"
        );
        assert!(
            toml::from_str::<toml::Value>(&written).is_ok(),
            "written config is not valid TOML: {written:?}"
        );
    }
}
