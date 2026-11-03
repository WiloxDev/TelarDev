//! Typography catalog, Nerd Font definitions, and terminal font configuration helpers.

/// Represents a curated modern monospace font profile optimized for coding and minimalist aesthetics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontProfile {
    pub id: &'static str,
    pub display_name: &'static str,
    pub nerd_font_family: &'static str,
    pub style_tags: &'static [&'static str],
    pub license: &'static str,
    pub download_url: &'static str,
}

/// Curated catalog of 12 modern, minimalist, thin/fine-capable monospace fonts
/// strictly verified with official Nerd Font patching to protect Seti UI and TUI file icons.
pub const FONT_CATALOG: &[FontProfile] = &[
    FontProfile {
        id: "geist-mono",
        display_name: "Geist Mono",
        nerd_font_family: "GeistMono Nerd Font",
        style_tags: &["swiss", "minimalist", "geometric", "clean"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/GeistMono.tar.xz",
    },
    FontProfile {
        id: "iosevka",
        display_name: "Iosevka",
        nerd_font_family: "Iosevka Nerd Font",
        style_tags: &["condensed", "ultra-slender", "space-efficient"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/Iosevka.tar.xz",
    },
    FontProfile {
        id: "jetbrains-mono",
        display_name: "JetBrains Mono",
        nerd_font_family: "JetBrainsMono Nerd Font",
        style_tags: &["modern", "high-legibility", "8deg-cut"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz",
    },
    FontProfile {
        id: "commit-mono",
        display_name: "Commit Mono",
        nerd_font_family: "CommitMono Nerd Font",
        style_tags: &["neutral", "swiss", "anonymous", "clean"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/CommitMono.tar.xz",
    },
    FontProfile {
        id: "cascadia-code",
        display_name: "Cascadia Code",
        nerd_font_family: "CaskaydiaCove Nerd Font",
        style_tags: &["humanist", "balanced", "friendly"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/CascadiaCode.tar.xz",
    },
    FontProfile {
        id: "monaspace-argon",
        display_name: "Monaspace Argon",
        nerd_font_family: "MonaspaceArgon Nerd Font",
        style_tags: &["texture-healing", "humanist", "innovative"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/Monaspace.tar.xz",
    },
    FontProfile {
        id: "monaspace-krypton",
        display_name: "Monaspace Krypton",
        nerd_font_family: "MonaspaceKrypton Nerd Font",
        style_tags: &["mechanical", "technical", "futuristic"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/Monaspace.tar.xz",
    },
    FontProfile {
        id: "victor-mono",
        display_name: "Victor Mono",
        nerd_font_family: "VictorMono Nerd Font",
        style_tags: &["slender", "cursive-italics", "distinctive"],
        license: "MIT",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/VictorMono.tar.xz",
    },
    FontProfile {
        id: "ibm-plex-mono",
        display_name: "IBM Plex Mono",
        nerd_font_family: "IBMPlexMono Nerd Font",
        style_tags: &["industrial", "engineered", "neo-grotesque"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/IBMPlexMono.tar.xz",
    },
    FontProfile {
        id: "intel-one-mono",
        display_name: "Intel One Mono",
        nerd_font_family: "IntelOneMono Nerd Font",
        style_tags: &["ergonomic", "low-fatigue", "readable"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/IntelOneMono.tar.xz",
    },
    FontProfile {
        id: "fira-code",
        display_name: "Fira Code",
        nerd_font_family: "FiraCode Nerd Font",
        style_tags: &["technical-ligatures", "classic", "crisp"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/FiraCode.tar.xz",
    },
    FontProfile {
        id: "source-code-pro",
        display_name: "Source Code Pro",
        nerd_font_family: "SauceCodePro Nerd Font",
        style_tags: &["adobe", "wide-aperture", "compact-clear"],
        license: "SIL OFL 1.1",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/SourceCodePro.tar.xz",
    },
];

/// Known host terminal emulators that may be hosting Herdr/Telar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostTerminalKind {
    Ghostty,
    Kitty,
    WezTerm,
    Alacritty,
    WindowsTerminal,
    Generic,
}

impl HostTerminalKind {
    /// Human-friendly name of the terminal emulator.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Ghostty => "Ghostty",
            Self::Kitty => "Kitty",
            Self::WezTerm => "WezTerm",
            Self::Alacritty => "Alacritty",
            Self::WindowsTerminal => "Windows Terminal",
            Self::Generic => "Terminal",
        }
    }

    /// Primary configuration file path or location indicator.
    pub fn config_location(&self) -> &'static str {
        match self {
            Self::Ghostty => "~/.config/ghostty/config",
            Self::Kitty => "~/.config/kitty/kitty.conf",
            Self::WezTerm => "~/.config/wezterm/wezterm.lua",
            Self::Alacritty => "~/.config/alacritty/alacritty.toml",
            Self::WindowsTerminal => "settings.json (profiles.defaults.font.face)",
            Self::Generic => "terminal config",
        }
    }
}

/// Detects the host terminal emulator based on environment variables.
pub fn detect_host_terminal_from_env() -> HostTerminalKind {
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default().to_lowercase();
    let term = std::env::var("TERM").unwrap_or_default().to_lowercase();

    if std::env::var("GHOSTTY_RESOURCES_DIR").is_ok()
        || term_program.contains("ghostty")
        || term.contains("ghostty")
    {
        return HostTerminalKind::Ghostty;
    }
    if std::env::var("KITTY_PID").is_ok() || term_program.contains("kitty") || term.contains("kitty") {
        return HostTerminalKind::Kitty;
    }
    if std::env::var("WEZTERM_PANE").is_ok()
        || term_program.contains("wezterm")
        || term.contains("wezterm")
    {
        return HostTerminalKind::WezTerm;
    }
    if std::env::var("ALACRITTY_LOG").is_ok()
        || std::env::var("ALACRITTY_WINDOW_ID").is_ok()
        || term_program.contains("alacritty")
        || term.contains("alacritty")
    {
        return HostTerminalKind::Alacritty;
    }
    if std::env::var("WT_SESSION").is_ok() || std::env::var("WT_PROFILE_ID").is_ok() {
        return HostTerminalKind::WindowsTerminal;
    }

    HostTerminalKind::Generic
}

/// Generates the exact configuration snippet for the target terminal to use the selected font.
pub fn format_terminal_directive(terminal: HostTerminalKind, font: &FontProfile) -> String {
    let family = font.nerd_font_family;
    match terminal {
        HostTerminalKind::Ghostty => format!("font-family = \"{family}\""),
        HostTerminalKind::Kitty => format!("font_family {family}"),
        HostTerminalKind::WezTerm => format!("config.font = wezterm.font(\"{family}\")"),
        HostTerminalKind::Alacritty => {
            format!("[font.normal]\nfamily = \"{family}\"")
        }
        HostTerminalKind::WindowsTerminal => {
            format!("\"font\": {{\n  \"face\": \"{family}\"\n}}")
        }
        HostTerminalKind::Generic => format!("font-family: \"{family}\""),
    }
}

/// Finds a font profile by its ID in the catalog.
// Allowed as public domain helper for font profile lookup by identifier.
#[allow(dead_code)]
pub fn find_font_by_id(id: &str) -> Option<&'static FontProfile> {
    FONT_CATALOG.iter().find(|font| font.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_catalog_has_at_least_twelve_fonts() {
        assert!(FONT_CATALOG.len() >= 12, "catalog must contain at least 12 fonts");
    }

    #[test]
    fn font_profiles_have_valid_identifiers_and_nerd_font_families() {
        for font in FONT_CATALOG {
            assert!(!font.id.is_empty(), "id must not be empty");
            assert!(!font.display_name.is_empty(), "display_name must not be empty");
            assert!(
                font.nerd_font_family.ends_with("Nerd Font"),
                "font '{}' family '{}' must end with 'Nerd Font' to protect Seti UI icons",
                font.id,
                font.nerd_font_family
            );
            assert!(!font.style_tags.is_empty(), "font '{}' must have style tags", font.id);
            assert!(
                font.download_url.starts_with("https://"),
                "download_url must be https: {}",
                font.download_url
            );
        }
    }

    #[test]
    fn format_terminal_directive_matches_emulator_syntaxes() {
        let font = &FONT_CATALOG[0]; // Geist Mono
        let family = font.nerd_font_family;

        let ghostty = format_terminal_directive(HostTerminalKind::Ghostty, font);
        assert_eq!(ghostty, format!("font-family = \"{family}\""));

        let kitty = format_terminal_directive(HostTerminalKind::Kitty, font);
        assert_eq!(kitty, format!("font_family {family}"));

        let wezterm = format_terminal_directive(HostTerminalKind::WezTerm, font);
        assert_eq!(wezterm, format!("config.font = wezterm.font(\"{family}\")"));

        let alacritty = format_terminal_directive(HostTerminalKind::Alacritty, font);
        assert!(alacritty.contains("[font.normal]"));
        assert!(alacritty.contains(&format!("family = \"{family}\"")));

        let wt = format_terminal_directive(HostTerminalKind::WindowsTerminal, font);
        assert!(wt.contains("\"font\""));
        assert!(wt.contains(&format!("\"face\": \"{family}\"")));
    }

    #[test]
    fn find_font_by_id_retrieves_matching_profile() {
        let geist = find_font_by_id("geist-mono").expect("geist-mono must exist");
        assert_eq!(geist.display_name, "Geist Mono");
        assert_eq!(geist.nerd_font_family, "GeistMono Nerd Font");

        assert!(find_font_by_id("non-existent-font").is_none());
    }
}
