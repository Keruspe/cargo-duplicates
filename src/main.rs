use cargo::{
    core::{dependency::DepKind, shell::Shell},
    ops::{
        Packages, load_pkg_lockfile,
        tree::{self, TreeOptions},
    },
    util::command_prelude::*,
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::Write,
};
use tabwriter::TabWriter;

fn main() {
     let mut gctx = GlobalContext::default().unwrap_or_else(|e| {
        let mut eval = Shell::new();
        cargo::exit_with_error(e.into(), &mut eval)
    });

    if let Err(e) = run(&mut gctx) {
        cargo::exit_with_error(e.into(), &mut gctx.shell())
    }
}

fn cli() -> Command {
    subcommand("duplicates")
        .about("A cargo subcommand for displaying when different versions of a same dependency are pulled in")
        .arg_features()
        .arg_manifest_path()
        .arg_lockfile_path()
}

fn run(gctx: &mut GlobalContext) -> anyhow::Result<()> {
    let args = cli().try_get_matches()?;
    let ws = args.workspace(gctx)?;
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
        return Ok(());
    }

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
    println!("{}", String::from_utf8(writer.into_inner()?)?);

    let duplicate_packages = duplicates
        .iter()
        .flat_map(|(_, deps)| {
            deps.iter()
                .rev()
                .map(|dep| format!("{}@{}", dep.name(), dep.version()))
        })
        .collect::<Vec<String>>();
    let mut edge_kinds = HashSet::new();
    edge_kinds.insert(tree::EdgeKind::Dep(DepKind::Normal));
    edge_kinds.insert(tree::EdgeKind::Dep(DepKind::Build));
    edge_kinds.insert(tree::EdgeKind::Dep(DepKind::Development));

    tree::build_and_print(
        &ws,
        &TreeOptions {
            cli_features: args.cli_features()?,
            packages: Packages::Packages(duplicate_packages.clone()),
            target: tree::Target::All,
            edge_kinds,
            invert: duplicate_packages,
            pkgs_to_prune: Default::default(),
            prefix: tree::Prefix::Indent,
            no_dedupe: true,
            duplicates: false,
            format: "{p}".to_string(),
            graph_features: false,
            display_depth: tree::DisplayDepth::MaxDisplayDepth(u32::MAX),
            no_proc_macro: false,
        },
    )
}
