// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

// cspell:ignore cppdocs pipenv pipfile

use anyhow::{Context, Result};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::util::{symlink_file, symlink_files_in_dir};

#[path = "../../api/cpp/cbindgen.rs"]
mod cbindgen;

pub fn generate(show_warnings: bool, experimental: bool) -> Result<(), Box<dyn std::error::Error>> {
    let root = super::root_dir();

    let docs_source_dir = root.join("api/cpp");
    let docs_build_dir = root.join("target/cppdocs");
    let html_static_dir = docs_build_dir.join("_static");

    std::fs::create_dir_all(docs_build_dir.as_path()).context("Error creating docs build dir")?;
    std::fs::create_dir_all(html_static_dir.as_path())
        .context("Error creating _static path for docs build")?;

    symlink_files_in_dir(
        docs_source_dir.join("docs"),
        &docs_build_dir,
        ["..", "..", "api", "cpp", "docs"].iter().collect::<PathBuf>(),
        &[],
    )
    .context("Error creating symlinks from docs source to docs build dir")?;

    symlink_file(
        ["..", "..", "api", "cpp", "README.md"].iter().collect::<PathBuf>(),
        docs_build_dir.join("README.md"),
    )?;

    let generated_headers_dir = docs_build_dir.join("generated_include");
    let enabled_features = cbindgen::EnabledFeatures {
        interpreter: true,
        testing: true,
        backend_qt: true,
        backend_winit: true,
        backend_winit_x11: false,
        backend_winit_wayland: false,
        backend_linuxkms: true,
        backend_linuxkms_noseat: false,
        renderer_femtovg: true,
        renderer_skia: true,
        renderer_skia_opengl: false,
        renderer_skia_vulkan: false,
        renderer_software: true,
        gettext: true,
        accessibility: true,
        system_testing: true,
        freestanding: true,
        experimental,
    };
    cbindgen::gen_all(&root, &generated_headers_dir, enabled_features)?;

    let pip_env = vec![(OsString::from("PIPENV_PIPFILE"), docs_source_dir.join("docs/Pipfile"))];

    println!("Running pipenv install");

    super::run_command("pipenv", &["install"], pip_env.clone())
        .context("Error running pipenv install")?;

    println!("Running sphinx-build");

    let output = super::run_command(
        "pipenv",
        &[
            "run",
            "sphinx-build",
            docs_build_dir.to_str().unwrap(),
            docs_build_dir.join("html").to_str().unwrap(),
        ],
        pip_env,
    )
    .context("Error running pipenv install")?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    if show_warnings {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
