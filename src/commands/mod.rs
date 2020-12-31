/// Command request structs
pub enum Commands {
    Init {
        path: Option<String>,
    },
    New {
        title: String,
        supersede: Option<usize>,
    },
    Link {
        record_a: String,
        record_b: String,
    },
    UnLink {
        record_a: String,
        record_b: String,
    },
    Build,
    Watch,
    Serve,
    Clean,
}
