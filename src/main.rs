use cargo::{
    core::{shell::Shell, Workspace},
    ops::load_pkg_lockfile,
    util::{command_prelude::*, important_paths},
};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    if let Err(e) = run(&mut config) {
        cargo::exit_with_error(e, &mut *config.shell())
    }
}

fn run(config: &mut Config) -> CliResult {
    let root = important_paths::find_root_manifest_for_wd(config.cwd())?;
    let ws = Workspace::new(&root, config)?;
    let lockfile = if let Some(lockfile) = load_pkg_lockfile(&ws)? {
        lockfile
    } else {
        return Err(anyhow::Error::msg("Cargo.lock is required").into());
    };
    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    for dep in lockfile.iter() {
        deps.entry(dep.name().to_string())
            .or_default()
            .insert(dep.version().to_string());
    }
    for (name, versions) in deps.iter().filter(|(_, v)| v.len() > 1) {
        println!(
            "Found duplicate: {}. Required versions: [{}].",
            name,
            versions
                .iter()
                .fold("".to_string(), |acc, version| if acc == "" {
                    version.to_string()
                } else {
                    format!("{}, {}", acc, version)
                })
        );
    }
    Ok(())
}
