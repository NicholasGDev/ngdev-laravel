pub struct ControllerArgs {
    pub name: String,
    pub resource: bool,
    pub model: Option<String>,
}

pub struct ModelArgs {
    pub name: String,
    pub migration: bool,
    pub controller: bool,
}

pub struct MigrationArgs {
    pub name: String,
    pub table: Option<String>,
}

pub struct PdvArgs {
    pub migrations_only: bool,
    pub models_only: bool,
}

