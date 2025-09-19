use std::collections::HashMap;

use alpm::SigLevel;

use crate::config::load_config;

pub fn sync(args: crate::Args) {
    let config = load_config(&args, None);
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman").unwrap();
    alpm.register_syncdb("core", SigLevel::PACKAGE).unwrap();
    alpm.register_syncdb("extra", SigLevel::PACKAGE).unwrap();
    let r = alpm.syncdbs();
    println!("Synced {} databases", r.len());
    println!(
        "Found {} packages",
        r.iter().map(|db| db.pkgs().len()).sum::<usize>()
    );
    let pkgs = alpm.localdb().pkgs();
    let pkgs = pkgs
        .into_iter()
        .map(|v| (v.name().to_string(), v))
        .collect::<HashMap<String, _>>();
    for dep in config.deps.iter() {
        if pkgs.contains_key(dep) {
            if pkgs[dep].reason() == alpm::PackageReason::Depend {
                println!("Marking {} explicitly", dep);
            }
            continue;
        }
        println!("Installing {}", dep);
    }

    alpm.release().unwrap()
}
