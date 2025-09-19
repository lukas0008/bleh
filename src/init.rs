use kdl::{KdlDocument, KdlNode, KdlValue};

pub fn init() {
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman").unwrap();
    let a = alpm.localdb().pkgs();
    let mut root_node = KdlNode::new("bleh");
    let mut root_doc = KdlDocument::new();
    let mut version_node = KdlNode::new("version");
    version_node.push(KdlValue::Integer(1));
    root_doc.nodes_mut().push(version_node);
    let mut dep_node = KdlNode::new("dependencies");
    let mut dep_doc = KdlDocument::new();
    for p in a.iter() {
        match p.reason() {
            alpm::PackageReason::Depend => continue,
            alpm::PackageReason::Explicit => {}
        }
        let dep = KdlNode::new(p.name());
        // dep.push(p.name());
        dep_doc.nodes_mut().push(dep);
        // dep_node.children_mut().insert()
        // dep_doc.ch(p.name());
    }
    let _ = dep_node.children_mut().insert(dep_doc);
    root_doc.nodes_mut().push(dep_node);
    let _ = root_node.children_mut().insert(root_doc);
    let mut doc = KdlDocument::new();
    doc.nodes_mut().push(root_node);
    // let mut doc = KdlDocument::new();
    // doc.nodes_mut().push(r);
    doc.autoformat();
    std::fs::write("/etc/bleh/bleh.kdl", doc.to_string()).unwrap();

    // println!("{}", doc.to_string());
    alpm.release().unwrap()
}
