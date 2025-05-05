use super::error::{SecurityError, SecurityResult};
use std::collections::HashMap;

pub struct SecurityManager {
    users: HashMap<UserId, User>,
    groups: HashMap<GroupId, Group>,
    permissions: PermissionManager,
    policies: SecurityPolicies,
    audit_log: AuditLog,
}

struct User {
    id: UserId,
    name: String,
    password_hash: String,
    groups: Vec<GroupId>,
    permissions: Permissions,
    session: Option<SessionId>,
}

struct Group {
    id: GroupId,
    name: String,
    permissions: Permissions,
    members: Vec<UserId>,
}

struct SecurityPolicies {
    password_policy: PasswordPolicy,
    access_control: AccessControl,
    network_policy: NetworkPolicy,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            users: HashMap::new(),
            groups: HashMap::new(),
            permissions: PermissionManager::new(),
            policies: SecurityPolicies::from_config(config),
            audit_log: AuditLog::new(),
        }
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> SecurityResult<SessionId> {
        let user = self.find_user(username)?;
        
        if !self.verify_password(password, &user.password_hash)? {
            self.audit_log.record_failed_login(username);
            return Err(SecurityError::InvalidCredentials);
        }
        
        let session = self.create_session(user.id)?;
        user.session = Some(session);
        
        self.audit_log.record_successful_login(username);
        Ok(session)
    }

    pub fn check_permission(&self, session: SessionId, permission: Permission) -> SecurityResult<bool> {
        let user = self.get_session_user(session)?;
        
        // Check user permissions
        if user.permissions.has(permission) {
            return Ok(true);
        }
        
        // Check group permissions
        for group_id in &user.groups {
            if let Some(group) = self.groups.get(group_id) {
                if group.permissions.has(permission) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }

    pub fn grant_permission(&mut self, granter: SessionId, grantee: UserId, permission: Permission) -> SecurityResult<()> {
        // Check if granter has permission to grant
        if !self.check_permission(granter, Permission::GrantPermissions)? {
            return Err(SecurityError::InsufficientPermissions);
        }
        
        let user = self.users.get_mut(&grantee)
            .ok_or(SecurityError::UserNotFound)?;
            
        user.permissions.add(permission);
        self.audit_log.record_permission_granted(granter, grantee, permission);
        
        Ok(())
    }
} 