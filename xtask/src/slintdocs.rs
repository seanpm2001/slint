// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

// cspell:ignore slintdocs pipenv pipfile

use anyhow::{Context, Result};
use std::ffi::OsString;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use xshell::{cmd, Shell};

use crate::util::{symlink_files_in_dir, to_kebab_case};

pub fn generate(show_warnings: bool) -> Result<(), Box<dyn std::error::Error>> {
    generate_enum_docs()?;
    generate_builtin_struct_docs()?;

    let root = super::root_dir();

    let docs_source_dir = root.join("docs/reference");
    let docs_build_dir = root.join("target/slintdocs");
    let html_static_dir = docs_build_dir.join("_static");

    std::fs::create_dir_all(docs_build_dir.as_path()).context("Error creating docs build dir")?;
    std::fs::create_dir_all(html_static_dir.as_path())
        .context("Error creating _static path for docs build")?;

    symlink_files_in_dir(
        &docs_source_dir,
        &docs_build_dir,
        ["..", "..", "docs", "reference"].iter().collect::<PathBuf>(),
        &[std::ffi::OsStr::new("_static")]
    )
    .context(format!("Error creating symlinks from docs source {docs_source_dir:?} to docs build dir {docs_build_dir:?}"))?;

    symlink_files_in_dir(
        &docs_source_dir.join("_static"),
        &html_static_dir,
        ["..", r"..", "..", "docs", "reference", "_static"].iter().collect::<PathBuf>(),
        &[]
    )
    .context(format!("Error creating symlinks from docs source {docs_source_dir:?} to docs build dir {docs_build_dir:?}"))?;

    {
        let sh = Shell::new()?;
        let _p = sh.push_dir(root.join("docs/editor"));
        let pnpm_check_output = cmd!(sh, "which pnpm").ignore_stdout().ignore_stderr().run();

        if pnpm_check_output.is_err() {
            eprintln!("Warning: 'pnpm' is not installed. Please install 'npm install -g pnpm' to proceed.");
            return Err(anyhow::anyhow!("'pnpm' is not installed.").into());
        }
        cmd!(sh, "pnpm install --frozen-lockfile --ignore-scripts").run()?;
        cmd!(sh, "pnpm build").run()?;
    }

    let pip_env = vec![(OsString::from("PIPENV_PIPFILE"), docs_source_dir.join("Pipfile"))];

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

pub fn generate_enum_docs() -> Result<(), Box<dyn std::error::Error>> {
    let mut enums: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();

    macro_rules! gen_enums {
        ($( $(#[doc = $enum_doc:literal])* $(#[non_exhaustive])? enum $Name:ident { $( $(#[doc = $value_doc:literal])* $Value:ident,)* })*) => {
            $(
                let mut entry = format!("## `{}`\n\n", stringify!($Name));
                $(entry += &format!("{}\n", $enum_doc);)*
                entry += "\n";
                $(
                    let mut has_val = false;
                    entry += &format!("* **`{}`**:", to_kebab_case(stringify!($Value)));
                    $(
                        if has_val {
                            entry += "\n   ";
                        }
                        entry += &format!("{}", $value_doc);
                        has_val = true;
                    )*
                    entry += "\n";
                )*
                entry += "\n";
                enums.insert(stringify!($Name).to_string(), entry);
            )*
        }
    }

    #[allow(unused)] // for 'has_val'
    {
        i_slint_common::for_each_enums!(gen_enums);
    }

    let root = super::root_dir();

    let path = root.join("docs/reference/src/language/builtins/enums.md");
    let mut file =
        BufWriter::new(std::fs::File::create(&path).context(format!("error creating {path:?}"))?);

    file.write_all(
        br#"<!-- Generated with `cargo xtask slintdocs` from internal/commons/enums.rs -->
# Builtin Enumerations

"#,
    )?;

    for (_, v) in enums {
        // BTreeMap<i64, String>
        write!(file, "{v}")?;
    }

    Ok(())
}

pub fn generate_builtin_struct_docs() -> Result<(), Box<dyn std::error::Error>> {
    // `Point` should be in the documentation, but it's not inside of `for_each_builtin_structs`,
    // so we manually create its entry first.
    let mut structs: std::collections::BTreeMap<String, String> =
        std::collections::BTreeMap::from([(
            "Point".to_string(),
            "## `Point`\n
This structure represents a point with x and y coordinate\n
### Fields\n
- **`x`** (_length_)
- **`y`** (_length_)\n\n"
                .to_string(),
        )]);
    macro_rules! map_type {
        (i32) => {
            stringify!(int)
        };
        (f32) => {
            stringify!(float)
        };
        (SharedString) => {
            stringify!(string)
        };
        (Coord) => {
            stringify!(length)
        };
        ($pub_type:ident) => {
            stringify!($pub_type)
        };
    }
    macro_rules! gen_structs {
        ($(
            $(#[doc = $struct_doc:literal])*
            $(#[non_exhaustive])?
            $(#[derive(Copy, Eq)])?
            struct $Name:ident {
                @name = $inner_name:literal
                export {
                    $( $(#[doc = $pub_doc:literal])* $pub_field:ident : $pub_type:ident, )*
                }
                private {
                    $( $(#[doc = $pri_doc:literal])* $pri_field:ident : $pri_type:ty, )*
                }
            }
        )*) => {
            $(
                let mut entry = format!("## `{}`\n\n", stringify!($Name));
                $(entry += &format!("{}\n", $struct_doc);)*
                entry += "\n### Fields\n\n";
                $(
                    entry += &format!("- **`{}`** (_{}_):", to_kebab_case(stringify!($pub_field)), map_type!($pub_type));
                    $(
                        entry += &format!("{}", $pub_doc);
                    )*
                    entry += "\n";
                )*
                entry += "\n";
                structs.insert(stringify!($Name).to_string(), entry);
            )*
        }
    }

    i_slint_common::for_each_builtin_structs!(gen_structs);

    let root = super::root_dir();

    let path = root.join("docs/reference/src/language/builtins/structs.md");
    let mut file =
        BufWriter::new(std::fs::File::create(&path).context(format!("error creating {path:?}"))?);

    file.write_all(
        br#"<!-- Generated with `cargo xtask slintdocs` from internal/common/builtin_structs.rs -->
# Builtin Structures

"#,
    )?;

    // `StateInfo` should not be in the documentation, so remove it before writing file
    structs.remove("StateInfo");
    for (_, v) in structs {
        write!(file, "{v}")?;
    }

    Ok(())
}
