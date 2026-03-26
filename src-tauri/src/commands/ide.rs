use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct DetectedApp {
    pub id: String,
    pub name: String,
    pub app_type: String, // "ide" or "terminal"
    pub command: String,
}

struct AppEntry {
    id: &'static str,
    name: &'static str,
    app_type: &'static str,
    command: &'static str,
}

const APP_REGISTRY: &[AppEntry] = &[
    // IDEs / Editors
    AppEntry {
        id: "vscode",
        name: "VS Code",
        app_type: "ide",
        command: "code",
    },
    AppEntry {
        id: "cursor",
        name: "Cursor",
        app_type: "ide",
        command: "cursor",
    },
    AppEntry {
        id: "zed",
        name: "Zed",
        app_type: "ide",
        command: "zed",
    },
    AppEntry {
        id: "sublime",
        name: "Sublime Text",
        app_type: "ide",
        command: "subl",
    },
    AppEntry {
        id: "idea",
        name: "IntelliJ IDEA",
        app_type: "ide",
        command: "idea",
    },
    AppEntry {
        id: "webstorm",
        name: "WebStorm",
        app_type: "ide",
        command: "webstorm",
    },
    AppEntry {
        id: "fleet",
        name: "Fleet",
        app_type: "ide",
        command: "fleet",
    },
    AppEntry {
        id: "clion",
        name: "CLion",
        app_type: "ide",
        command: "clion",
    },
    AppEntry {
        id: "goland",
        name: "GoLand",
        app_type: "ide",
        command: "goland",
    },
    AppEntry {
        id: "pycharm",
        name: "PyCharm",
        app_type: "ide",
        command: "pycharm",
    },
    AppEntry {
        id: "rustrover",
        name: "RustRover",
        app_type: "ide",
        command: "rustrover",
    },
    // Terminals
    #[cfg(target_os = "linux")]
    AppEntry {
        id: "gnome-terminal",
        name: "GNOME Terminal",
        app_type: "terminal",
        command: "gnome-terminal",
    },
    #[cfg(target_os = "linux")]
    AppEntry {
        id: "konsole",
        name: "Konsole",
        app_type: "terminal",
        command: "konsole",
    },
    AppEntry {
        id: "alacritty",
        name: "Alacritty",
        app_type: "terminal",
        command: "alacritty",
    },
    AppEntry {
        id: "kitty",
        name: "Kitty",
        app_type: "terminal",
        command: "kitty",
    },
    AppEntry {
        id: "wezterm",
        name: "WezTerm",
        app_type: "terminal",
        command: "wezterm",
    },
    #[cfg(target_os = "linux")]
    AppEntry {
        id: "xterm",
        name: "xterm",
        app_type: "terminal",
        command: "xterm",
    },
    #[cfg(target_os = "windows")]
    AppEntry {
        id: "wt",
        name: "Windows Terminal",
        app_type: "terminal",
        command: "wt",
    },
];

#[tauri::command]
pub fn detect_available_apps() -> Result<Vec<DetectedApp>, String> {
    let mut detected = Vec::new();

    for entry in APP_REGISTRY {
        if which::which(entry.command).is_ok() {
            detected.push(DetectedApp {
                id: entry.id.to_string(),
                name: entry.name.to_string(),
                app_type: entry.app_type.to_string(),
                command: entry.command.to_string(),
            });
        }
    }

    Ok(detected)
}

#[tauri::command]
pub fn open_in_app(app_id: String, path: String) -> Result<(), String> {
    let path_ref = std::path::Path::new(&path);
    if !path_ref.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let entry = APP_REGISTRY
        .iter()
        .find(|e| e.id == app_id)
        .ok_or_else(|| format!("Unknown app: {}", app_id))?;

    // All IDEs accept the directory as a plain argument
    Command::new(entry.command)
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", entry.name, e))?;

    Ok(())
}

#[tauri::command]
pub fn open_terminal(
    terminal_id: String,
    path: String,
    command: Option<String>,
) -> Result<(), String> {
    let path_ref = std::path::Path::new(&path);
    if !path_ref.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let entry = APP_REGISTRY
        .iter()
        .find(|e| e.id == terminal_id && e.app_type == "terminal")
        .ok_or_else(|| format!("Unknown terminal: {}", terminal_id))?;

    let mut cmd = Command::new(entry.command);

    // Build a shell snippet that cd's into the working directory,
    // runs the requested command (if any), then drops into an interactive shell.
    let shell_script = match &command {
        Some(c) => format!("cd '{}' && {} ; exec bash", path, c),
        None => format!("cd '{}' && exec bash", path),
    };

    match entry.id {
        "gnome-terminal" => {
            cmd.arg("--").arg("bash").arg("-c").arg(&shell_script);
        }
        "konsole" => {
            cmd.arg("--workdir")
                .arg(&path)
                .arg("-e")
                .arg("bash")
                .arg("-c")
                .arg(&shell_script);
        }
        "alacritty" => {
            cmd.arg("--working-directory")
                .arg(&path)
                .arg("-e")
                .arg("bash")
                .arg("-c")
                .arg(&shell_script);
        }
        "kitty" => {
            cmd.arg("--directory")
                .arg(&path)
                .arg("bash")
                .arg("-c")
                .arg(&shell_script);
        }
        "wezterm" => {
            cmd.arg("start")
                .arg("--cwd")
                .arg(&path)
                .arg("--")
                .arg("bash")
                .arg("-c")
                .arg(&shell_script);
        }
        "xterm" => {
            cmd.arg("-e").arg("bash").arg("-c").arg(&shell_script);
        }
        "wt" => {
            cmd.arg("-d").arg(&path);
            if let Some(c) = &command {
                cmd.arg("cmd").arg("/k").arg(c);
            }
        }
        _ => {
            return Err(format!(
                "No launch configuration for terminal: {}",
                entry.id
            ));
        }
    }

    cmd.spawn()
        .map_err(|e| format!("Failed to launch {}: {}", entry.name, e))?;

    Ok(())
}
