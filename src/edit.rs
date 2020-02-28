/*
 * This file is part of espanso.
 *
 * Copyright (C) 2020 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::config::ConfigSet;
use std::path::Path;

pub fn open_editor(config: &ConfigSet, file_path: &Path) -> bool {
    use std::process::Command;

    // Check if another editor is defined in the environment variables
    let editor_var = std::env::var_os("EDITOR");
    let visual_var = std::env::var_os("VISUAL");

    // Prioritize the editors specified by the environment variable, otherwise use the config
    let editor : String = if let Some(editor_var) = editor_var {
        editor_var.to_string_lossy().to_string()
    }else if let Some(visual_var) = visual_var {
        visual_var.to_string_lossy().to_string()
    }else{
        config.default.editor.clone()
    };

    // Start the editor and wait for its termination
    let status = Command::new(editor)
        .arg(file_path)
        .spawn();

    if let Ok(mut child) = status {
        // Wait for the user to edit the configuration
        let result = child.wait();

        if let Ok(exit_status) = result {
            exit_status.success()
        }else{
            false
        }
    }else{
        println!("Error: could not start editor at: {}", config.default.editor);
        false
    }
}