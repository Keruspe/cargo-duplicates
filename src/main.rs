use cargo::{
    core::{shell::Shell, Workspace},
    ops::load_pkg_lockfile,
    util::{command_prelude::*, important_paths},
};
use std::{
    collections::{BTreeSet, HashMap},
    io::Write,
};
use tabwriter::TabWriter;

fn main() {
    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    if let Err(e) = run(&mut config) {
        cargo::exit_with_error(e.into(), &mut *config.shell())
    }
}

fn run(config: &mut Config) -> anyhow::Result<()> {
    let root = important_paths::find_root_manifest_for_wd(config.cwd())?;
    let ws = Workspace::new(&root, config)?;
    let lockfile = if let Some(lockfile) = load_pkg_lockfile(&ws)? {
        lockfile
    } else {
        anyhow::bail!("Cargo.lock is required");
    };
    let mut duplicates: HashMap<String, BTreeSet<_>> = HashMap::new();
    for dep in lockfile.iter() {
        duplicates.entry(dep.name().to_string())
            .or_default()
            .insert(dep);
    }
    duplicates.retain(|_, v| v.len() > 1);
    if duplicates.is_empty() {
        println!("No duplicate dependencies, yay!");
    } else {
        let mut writer = TabWriter::new(Vec::new());
        writeln!(&mut writer, "Package\tVersions")?;
        writeln!(&mut writer, "-------\t--------")?;
        for (name, deps) in duplicates {
            writeln!(
                &mut writer,
                "{}\t{}",
                name,
                deps
                    .iter()
                    .fold("".to_string(), |acc, dep| if acc == "" {
                        dep.version().to_string()
                    } else {
                        format!("{}\t{}", dep.version(), acc)
                    })
            )?;
        }
        writer.flush()?;
        print!("{}", String::from_utf8(writer.into_inner()?)?);
    }
    Ok(())
}
