use ratatui::style::Color;

/// Representación compacta de un icono de archivo estilo VS Code (Seti UI).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileIcon {
    pub glyph: &'static str,
    pub color: Color,
}

impl Default for FileIcon {
    fn default() -> Self {
        Self {
            glyph: "≡",
            color: Color::Rgb(109, 128, 134),
        }
    }
}

/// Coincidencia de nombres exactos prioritarios en proyectos de software.
fn match_exact_filename(name: &str) -> Option<FileIcon> {
    match name {
        "package.json" => Some(FileIcon {
            glyph: "{}",
            color: Color::Rgb(203, 203, 65), // Verde lima / Amarillo Seti (#cbcb41)
        }),
        "tsconfig.json" => Some(FileIcon {
            glyph: "TS",
            color: Color::Rgb(81, 154, 186), // Azul TypeScript (#519aba)
        }),
        "jsconfig.json" => Some(FileIcon {
            glyph: "JS",
            color: Color::Rgb(203, 203, 65),
        }),
        ".gitignore" | ".gitattributes" | ".gitmodules" => Some(FileIcon {
            glyph: "⎇",
            color: Color::Rgb(65, 83, 91), // Gris petróleo Git (#41535b)
        }),
        ".npmrc" => Some(FileIcon {
            glyph: "npm",
            color: Color::Rgb(204, 62, 68), // Rojo NPM (#cc3e44)
        }),
        "compose.yaml" | "compose.yml" | "docker-compose.yaml" | "docker-compose.yml" => {
            Some(FileIcon {
                glyph: "🐳",
                color: Color::Rgb(240, 98, 146), // Magenta Docker Compose (#f06292)
            })
        }
        "Dockerfile" | "dockerfile" | ".dockerignore" => Some(FileIcon {
            glyph: "🐳",
            color: Color::Rgb(36, 150, 237), // Azul Docker (#2496ed)
        }),
        ".editorconfig" => Some(FileIcon {
            glyph: "⚙",
            color: Color::Rgb(109, 128, 134), // Gris frío (#6d8086)
        }),
        ".env" | ".env.local" | ".env.development" | ".env.production" => {
            Some(FileIcon {
                glyph: "🔒",
                color: Color::Rgb(243, 139, 168), // Rojo de alerta/seguridad (#f38ba8)
            })
        }
        ".env.example" | ".env.sample" | ".env.template" => {
            Some(FileIcon {
                glyph: "⚙",
                color: Color::Rgb(249, 226, 175), // Oro/Amarillo tenue (#f9e2af)
            })
        }
        ".node-version" | ".nvmrc" => Some(FileIcon {
            glyph: "≡",
            color: Color::Rgb(141, 193, 73), // Verde Node (#8dc149)
        }),
        "Cargo.toml" => Some(FileIcon {
            glyph: "🦀",
            color: Color::Rgb(222, 165, 132), // Naranja Rust (#dea584)
        }),
        "Cargo.lock" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" | "bun.lockb" => {
            Some(FileIcon {
                glyph: "🔒",
                color: Color::Rgb(108, 112, 134), // Candado gris (#6c7086)
            })
        }
        "README" | "README.md" | "README.txt" => Some(FileIcon {
            glyph: "📖",
            color: Color::Rgb(81, 154, 186), // Azul Markdown (#519aba)
        }),
        "LICENSE" | "LICENSE.md" | "LICENCE" => Some(FileIcon {
            glyph: "⚖",
            color: Color::Rgb(203, 203, 65),
        }),
        "Makefile" | "Justfile" => Some(FileIcon {
            glyph: "🛠",
            color: Color::Rgb(109, 128, 134),
        }),
        _ => {
            if name.starts_with("pnpm-") {
                Some(FileIcon {
                    glyph: "!",
                    color: Color::Rgb(177, 128, 215), // Púrpura PNPM (#b180d7)
                })
            } else {
                None
            }
        }
    }
}

/// Coincidencia exhaustiva por extensión (sin asignación dinámica en heap).
fn match_extension(ext: &str) -> FileIcon {
    match ext {
        // Documentos y Suites Ofimáticas
        "pdf" => FileIcon {
            glyph: "▤",
            color: Color::Rgb(204, 62, 68), // Rojo Adobe (#cc3e44)
        },
        "doc" | "docx" | "odt" | "rtf" => FileIcon {
            glyph: "DOC",
            color: Color::Rgb(81, 154, 186), // Azul Word (#519aba)
        },
        "xls" | "xlsx" | "csv" | "tsv" | "ods" => FileIcon {
            glyph: "XLS",
            color: Color::Rgb(141, 193, 73), // Verde Excel (#8dc149)
        },
        "ppt" | "pptx" | "odp" | "keynote" => FileIcon {
            glyph: "PPT",
            color: Color::Rgb(210, 71, 38), // Naranja PowerPoint (#d24726)
        },
        "md" | "markdown" | "rst" => FileIcon {
            glyph: "M↓",
            color: Color::Rgb(81, 154, 186), // Azul Markdown
        },
        "txt" | "log" => FileIcon {
            glyph: "≡",
            color: Color::Rgb(166, 173, 200), // Texto neutro tenue
        },

        // Web y Frontend
        "html" | "htm" | "xhtml" => FileIcon {
            glyph: "<>",
            color: Color::Rgb(227, 79, 38), // Naranja HTML5 (#e34f26)
        },
        "css" => FileIcon {
            glyph: "#",
            color: Color::Rgb(81, 154, 186), // Azul CSS (#519aba)
        },
        "scss" | "sass" | "less" => FileIcon {
            glyph: "§",
            color: Color::Rgb(245, 83, 133), // Rosa Sass (#f55385)
        },
        "js" | "mjs" | "cjs" => FileIcon {
            glyph: "JS",
            color: Color::Rgb(203, 203, 65), // Amarillo JS (#cbcb41)
        },
        "ts" | "mts" | "cts" => FileIcon {
            glyph: "TS",
            color: Color::Rgb(81, 154, 186), // Azul TS (#519aba)
        },
        "jsx" => FileIcon {
            glyph: "JSX",
            color: Color::Rgb(97, 218, 251), // Cyan React (#61dafb)
        },
        "tsx" => FileIcon {
            glyph: "TSX",
            color: Color::Rgb(97, 218, 251), // Cyan React TSX
        },
        "vue" => FileIcon {
            glyph: "V",
            color: Color::Rgb(66, 184, 131), // Verde Vue (#42b883)
        },
        "svelte" => FileIcon {
            glyph: "S",
            color: Color::Rgb(255, 62, 0), // Naranja Svelte (#ff3e00)
        },
        "astro" => FileIcon {
            glyph: "A",
            color: Color::Rgb(255, 93, 1), // Naranja Astro
        },

        // Backend y Scripts
        "rs" => FileIcon {
            glyph: "RS",
            color: Color::Rgb(222, 165, 132), // Naranja Rust (#dea584)
        },
        "py" | "pyw" | "ipynb" => FileIcon {
            glyph: "PY",
            color: Color::Rgb(81, 154, 186), // Azul Python (#519aba)
        },
        "go" => FileIcon {
            glyph: "GO",
            color: Color::Rgb(0, 173, 216), // Cyan Go (#00add8)
        },
        "ps1" | "psm1" | "psd1" => FileIcon {
            glyph: ">_",
            color: Color::Rgb(78, 201, 176), // Cyan PowerShell (#4ec9b0)
        },
        "sh" | "bash" | "zsh" | "fish" => FileIcon {
            glyph: "$",
            color: Color::Rgb(137, 224, 81), // Verde Bash (#89e051)
        },
        "c" | "h" => FileIcon {
            glyph: "C",
            color: Color::Rgb(81, 154, 186),
        },
        "cpp" | "cc" | "cxx" | "hpp" => FileIcon {
            glyph: "C++",
            color: Color::Rgb(81, 154, 186),
        },
        "cs" => FileIcon {
            glyph: "C#",
            color: Color::Rgb(23, 134, 0), // Verde C# (#178600)
        },
        "java" | "jar" | "class" => FileIcon {
            glyph: "JV",
            color: Color::Rgb(204, 62, 68),
        },
        "kt" | "kts" => FileIcon {
            glyph: "KT",
            color: Color::Rgb(127, 82, 255),
        },
        "php" => FileIcon {
            glyph: "PHP",
            color: Color::Rgb(81, 154, 186),
        },
        "rb" | "gem" => FileIcon {
            glyph: "RB",
            color: Color::Rgb(204, 62, 68),
        },
        "lua" => FileIcon {
            glyph: "☾",
            color: Color::Rgb(81, 154, 186),
        },
        "zig" => FileIcon {
            glyph: "Z",
            color: Color::Rgb(236, 145, 92),
        },
        "dart" => FileIcon {
            glyph: "D",
            color: Color::Rgb(0, 180, 171),
        },

        // Configuración y Datos Estructurados
        "json" | "jsonc" | "json5" => FileIcon {
            glyph: "{}",
            color: Color::Rgb(203, 203, 65), // Amarillo JSON (#cbcb41)
        },
        "yaml" | "yml" => FileIcon {
            glyph: "Y",
            color: Color::Rgb(224, 95, 101), // Rojo/Rosa YAML (#e05f65)
        },
        "toml" => FileIcon {
            glyph: "T",
            color: Color::Rgb(156, 66, 33), // Naranja/Marrón TOML (#9c4221)
        },
        "xml" => FileIcon {
            glyph: "<>",
            color: Color::Rgb(227, 79, 38),
        },
        "sql" | "pgsql" | "mysql" => FileIcon {
            glyph: "🗄",
            color: Color::Rgb(227, 140, 0), // Ámbar SQL (#e38c00)
        },
        "db" | "sqlite" | "sqlite3" => FileIcon {
            glyph: "🗃",
            color: Color::Rgb(0, 59, 87),
        },
        "graphql" | "gql" => FileIcon {
            glyph: "◈",
            color: Color::Rgb(225, 0, 152), // Rosa GraphQL
        },
        "prisma" => FileIcon {
            glyph: "▲",
            color: Color::Rgb(45, 55, 72),
        },

        // Multimedia y Assets
        "png" | "jpg" | "jpeg" | "webp" | "gif" | "ico" | "bmp" | "svg" | "avif" => {
            FileIcon {
                glyph: "🖼",
                color: Color::Rgb(160, 116, 196), // Púrpura imagen (#a074c4)
            }
        }
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" => FileIcon {
            glyph: "♪",
            color: Color::Rgb(0, 175, 255), // Cyan audio (#00afff)
        },
        "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" => FileIcon {
            glyph: "▶",
            color: Color::Rgb(253, 151, 31), // Naranja video (#fd971f)
        },
        "ttf" | "otf" | "woff" | "woff2" => FileIcon {
            glyph: "F",
            color: Color::Rgb(204, 62, 68),
        },
        "zip" | "tar" | "gz" | "7z" | "rar" | "bz2" | "xz" => FileIcon {
            glyph: "📦",
            color: Color::Rgb(236, 165, 23), // Amarillo archivo
        },
        "pem" | "key" | "pub" | "crt" | "cer" | "pfx" => FileIcon {
            glyph: "🔐",
            color: Color::Rgb(249, 226, 175), // Claves de seguridad
        },

        // Fallback genérico para archivos de texto o código no catalogados
        _ => FileIcon {
            glyph: "≡",
            color: Color::Rgb(109, 128, 134),
        },
    }
}

/// Resuelve el icono y su color asignado de forma pura, en tiempo constante O(1)
/// y sin asignaciones dinámicas en el Heap.
pub fn resolve_file_icon(name: &str) -> FileIcon {
    if let Some(icon) = match_exact_filename(name) {
        return icon;
    }

    if super::telemetry::is_sensitive_file(name) {
        return FileIcon {
            glyph: "🔒",
            color: Color::Rgb(243, 139, 168), // Rojo de alerta/seguridad (#f38ba8)
        };
    }

    if let Some(dot_pos) = name.rfind('.') {
        if dot_pos + 1 < name.len() {
            let ext = &name[dot_pos + 1..];
            let mut ext_buf = [0u8; 16];
            if ext.len() <= 16 {
                for (i, b) in ext.bytes().enumerate() {
                    ext_buf[i] = b.to_ascii_lowercase();
                }
                if let Ok(lower_ext) = std::str::from_utf8(&ext_buf[..ext.len()]) {
                    return match_extension(lower_ext);
                }
            }
        }
    }

    FileIcon::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_filenames_match_priority() {
        let pkg = resolve_file_icon("package.json");
        assert_eq!(pkg.glyph, "{}");
        assert_eq!(pkg.color, Color::Rgb(203, 203, 65));

        let tsconfig = resolve_file_icon("tsconfig.json");
        assert_eq!(tsconfig.glyph, "TS");
        assert_eq!(tsconfig.color, Color::Rgb(81, 154, 186));

        let gitignore = resolve_file_icon(".gitignore");
        assert_eq!(gitignore.glyph, "⎇");

        let pnpm = resolve_file_icon("pnpm-lock.yaml");
        assert_eq!(pnpm.glyph, "🔒");

        let pnpm_ws = resolve_file_icon("pnpm-workspace.yaml");
        assert_eq!(pnpm_ws.glyph, "!");
    }

    #[test]
    fn extension_matches_office_and_web() {
        let pdf = resolve_file_icon("documento.pdf");
        assert_eq!(pdf.glyph, "▤");
        assert_eq!(pdf.color, Color::Rgb(204, 62, 68));

        let xlsx = resolve_file_icon("reporte.XLSX"); // Prueba case-insensitivity
        assert_eq!(xlsx.glyph, "XLS");
        assert_eq!(xlsx.color, Color::Rgb(141, 193, 73));

        let doc = resolve_file_icon("tesis.docx");
        assert_eq!(doc.glyph, "DOC");

        let html = resolve_file_icon("Task-Manager-Portable.html");
        assert_eq!(html.glyph, "<>");
        assert_eq!(html.color, Color::Rgb(227, 79, 38));

        let ps1 = resolve_file_icon("SIO_M1C7B2B0_R4_CORRECTED.ps1");
        assert_eq!(ps1.glyph, ">_");
        assert_eq!(ps1.color, Color::Rgb(78, 201, 176));
    }

    #[test]
    fn unknown_extensions_fallback_gracefully() {
        let fallback = resolve_file_icon("archivo.xyz123");
        assert_eq!(fallback.glyph, "≡");
        assert_eq!(fallback.color, Color::Rgb(109, 128, 134));

        let no_ext = resolve_file_icon("sin_extension");
        assert_eq!(no_ext.glyph, "≡");
    }

    #[test]
    fn sensitive_files_resolve_lock_icon() {
        let env_file = resolve_file_icon(".env");
        assert_eq!(env_file.glyph, "🔒");
        assert_eq!(env_file.color, Color::Rgb(243, 139, 168));

        let env_prod = resolve_file_icon(".env.production");
        assert_eq!(env_prod.glyph, "🔒");
        assert_eq!(env_prod.color, Color::Rgb(243, 139, 168));

        let env_example = resolve_file_icon(".env.example");
        assert_eq!(env_example.glyph, "⚙");

        let creds = resolve_file_icon("credentials.json");
        assert_eq!(creds.glyph, "🔒");
        assert_eq!(creds.color, Color::Rgb(243, 139, 168));

        let privkey = resolve_file_icon("server.key");
        assert_eq!(privkey.glyph, "🔒");
        assert_eq!(privkey.color, Color::Rgb(243, 139, 168));
    }
}
