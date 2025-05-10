// Authorization logic

// Access control
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    Custom(String),
}

#[derive(Debug, Clone, Default)]
pub struct Role {
    name: String,
    permissions: HashSet<Permission>,
    description: Option<String>,
}

impl Role {
    pub fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            permissions: HashSet::new(),
            description: description.map(|s| s.to_string()),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    
    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }
    
    pub fn remove_permission(&mut self, permission: &Permission) -> bool {
        self.permissions.remove(permission)
    }
    
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
    
    pub fn permissions(&self) -> &HashSet<Permission> {
        &self.permissions
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthorizationManager {
    roles: HashMap<String, Role>,
    user_roles: HashMap<String, HashSet<String>>, // user_id -> set of role names
}

impl AuthorizationManager {
    pub fn new() -> Self {
        let mut manager = Self {
            roles: HashMap::new(),
            user_roles: HashMap::new(),
        };
        
        // Create default roles
        let mut admin_role = Role::new("admin", Some("Administrator with all permissions"));
        admin_role.add_permission(Permission::Read);
        admin_role.add_permission(Permission::Write);
        admin_role.add_permission(Permission::Execute);
        admin_role.add_permission(Permission::Admin);
        manager.roles.insert("admin".to_string(), admin_role);
        
        let mut user_role = Role::new("user", Some("Standard user"));
        user_role.add_permission(Permission::Read);
        user_role.add_permission(Permission::Write);
        manager.roles.insert("user".to_string(), user_role);
        
        let mut guest_role = Role::new("guest", Some("Guest user with limited access"));
        guest_role.add_permission(Permission::Read);
        manager.roles.insert("guest".to_string(), guest_role);
        
        manager
    }
    
    pub fn add_role(&mut self, role: Role) -> Result<(), &'static str> {
        if self.roles.contains_key(role.name()) {
            return Err("Role already exists");
        }
        
        self.roles.insert(role.name().to_string(), role);
        Ok(())
    }
    
    pub fn remove_role(&mut self, role_name: &str) -> Result<(), &'static str> {
        if role_name == "admin" || role_name == "user" || role_name == "guest" {
            return Err("Cannot remove built-in role");
        }
        
        if self.roles.remove(role_name).is_none() {
            return Err("Role does not exist");
        }
        
        // Remove role from all users
        for roles in self.user_roles.values_mut() {
            roles.remove(role_name);
        }
        
        Ok(())
    }
    
    pub fn assign_role(&mut self, user_id: &str, role_name: &str) -> Result<(), &'static str> {
        if !self.roles.contains_key(role_name) {
            return Err("Role does not exist");
        }
        
        self.user_roles.entry(user_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(role_name.to_string());
        
        Ok(())
    }
    
    pub fn revoke_role(&mut self, user_id: &str, role_name: &str) -> Result<(), &'static str> {
        if let Some(roles) = self.user_roles.get_mut(user_id) {
            if !roles.remove(role_name) {
                return Err("User does not have this role");
            }
            
            Ok(())
        } else {
            Err("User not found")
        }
    }
    
    pub fn has_permission(&self, user_id: &str, permission: &Permission) -> bool {
        if let Some(role_names) = self.user_roles.get(user_id) {
            for role_name in role_names {
                if let Some(role) = self.roles.get(role_name) {
                    if role.has_permission(permission) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    pub fn get_user_roles(&self, user_id: &str) -> HashSet<String> {
        self.user_roles.get(user_id)
            .cloned()
            .unwrap_or_default()
    }
    
    pub fn get_user_permissions(&self, user_id: &str) -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        
        if let Some(role_names) = self.user_roles.get(user_id) {
            for role_name in role_names {
                if let Some(role) = self.roles.get(role_name) {
                    for permission in role.permissions() {
                        permissions.insert(permission.clone());
                    }
                }
            }
        }
        
        permissions
    }
}