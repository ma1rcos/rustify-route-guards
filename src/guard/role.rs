use std::sync::RwLock;

static CURRENT_ROLES: RwLock<Vec<String>> = RwLock::new(Vec::new());

pub fn set_roles(roles: Vec<String>) {
    let mut current = CURRENT_ROLES.write().unwrap();
    *current = roles;
}

pub fn has_role(role: &str) -> bool {
    let current = CURRENT_ROLES.read().unwrap();
    current.contains(&role.to_string())
}