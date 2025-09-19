pub struct Config {
    pub deps: Vec<String>,
}

impl Config {
    pub fn merge(&mut self, other: Config) {
        for dep in other.deps {
            if !self.deps.contains(&dep) {
                self.deps.push(dep);
            }
        }
    }
}

fn parse_deps(bleh_node: &kdl::KdlNode) -> Vec<String> {
    bleh_node
        .children()
        .unwrap()
        .nodes()
        .iter()
        .filter(|v| v.name().value() == "dependencies")
        .flat_map(|v| {
            v.children()
                .unwrap()
                .nodes()
                .iter()
                .map(|v| v.name().value().to_string())
        })
        .collect::<Vec<_>>()
}

pub fn load_config(_args: &crate::Args, config: Option<String>) -> Config {
    let cfg_doc = kdl::KdlDocument::parse(&config.unwrap_or_else(|| {
        if let Ok(v) = std::fs::read("/etc/bleh/bleh.kdl") {
            String::from_utf8(v).unwrap()
        } else {
            eprintln!("Please --init first");
            std::process::exit(1);
        }
    }))
    .unwrap();
    let bleh_node = cfg_doc
        .nodes()
        .iter()
        .filter(|v| v.name().value() == "bleh")
        .collect::<Vec<_>>();
    let deps = bleh_node
        .iter()
        .flat_map(|v| parse_deps(v))
        .collect::<Vec<_>>();
    let includes = cfg_doc
        .nodes()
        .iter()
        .filter(|v| v.name().value() == "include")
        .map(|v| v.entries().first().unwrap().value().as_string().unwrap())
        .collect::<Vec<_>>();
    let mut config = Config { deps };
    for include in includes {
        // println!("Including {}", include);
        if include.ends_with(".kdl") {
            let other_str = String::from_utf8(std::fs::read(include).unwrap()).unwrap();
            let other = load_config(_args, Some(other_str));
            config.merge(other);
        }
    }
    config
}
