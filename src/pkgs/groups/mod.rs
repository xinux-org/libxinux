pub mod schema;

pubb struct OldGroups {
    arch: String,
    name: String,
    packs: Vec<Package>
}

pub struct Package {
    arch: String,
    repo: String,
    name: String,
    version: String,
    desc: String,
    updated: String,
    flag: String,
}

pub struct ListGroup {
    arch: String,
    name: String,
    packs: String,
    update: String,
}