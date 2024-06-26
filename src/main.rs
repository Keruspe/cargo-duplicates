use cargo::{
    core::{shell::Shell, PackageId, Resolve, Workspace},
    ops::load_pkg_lockfile,
    util::{command_prelude::*, important_paths},
};
use std::{
    collections::{BTreeSet, HashMap},
    io::Write,
};
use tabwriter::TabWriter;

fn main() {
    let mut gctx = match GlobalContext::default() {
        Ok(gctx) => gctx,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    if let Err(e) = run(&mut gctx) {
        cargo::exit_with_error(e.into(), &mut *gctx.shell())
    }
}

fn run(gctx: &mut GlobalContext) -> anyhow::Result<()> {
    let root = important_paths::find_root_manifest_for_wd(gctx.cwd())?;
    let ws = Workspace::new(&root, gctx)?;
    let lockfile = if let Some(lockfile) = load_pkg_lockfile(&ws)? {
        lockfile
    } else {
        anyhow::bail!("Cargo.lock is required");
    };
    let mut duplicates: HashMap<String, BTreeSet<_>> = HashMap::new();
    for dep in lockfile.iter() {
        duplicates
            .entry(dep.name().to_string())
            .or_default()
            .insert(dep);
    }
    let mut duplicates = duplicates
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .collect::<Vec<_>>();
    duplicates.sort_by_key(|d| d.0);
    if duplicates.is_empty() {
        println!("No duplicate dependencies, yay!");
    } else {
        let mut writer = TabWriter::new(Vec::new());
        writeln!(&mut writer, "Package\tVersions")?;
        writeln!(&mut writer, "-------\t--------")?;
        for (name, deps) in &duplicates {
            writeln!(
                &mut writer,
                "{}\t{}",
                name,
                deps.iter()
                    .fold("".to_string(), |acc, dep| if acc.is_empty() {
                        dep.version().to_string()
                    } else {
                        format!("{}\t{}", dep.version(), acc)
                    })
            )?;
        }
        writer.flush()?;
        print!("{}", String::from_utf8(writer.into_inner()?)?);
        for (_, deps) in &duplicates {
            println!();
            for dep in deps.iter().rev() {
                println!("{} {}:", dep.name(), dep.version());
                for c in find_dependents_of(&lockfile, *dep) {
                    println!(
                        "{} {} {}",
                        c.iter()
                            .fold("- Because of".to_string(), |acc, dep| format!(
                                "{} {} {} =>",
                                acc,
                                dep.name(),
                                dep.version()
                            )),
                        dep.name(),
                        dep.version()
                    );
                }
            }
        }
    }
    Ok(())
}

fn find_dependents_of(lockfile: &Resolve, dep: PackageId) -> Vec<Vec<PackageId>> {
    let mut res = Vec::new();
    for d in lockfile.iter() {
        if lockfile.deps(d).into_iter().filter(|d| d.0 == dep).count() != 0 {
            let mut r = find_dependents_of(lockfile, d)
                .iter()
                .map(|dd| {
                    let mut dd = dd.clone();
                    dd.push(d);
                    dd
                })
                .collect::<Vec<_>>();
            if r.is_empty() {
                res.push(vec![d]);
            } else {
                res.append(&mut r);
            }
        }
    }
    res
}
