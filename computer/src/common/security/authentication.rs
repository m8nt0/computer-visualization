// Identity verification
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    Password,
    Biometric,
    Token,
    TwoFactor,
    Certificate,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    user_id: String,
    authenticated: bool,
    method: AuthMethod,
    timestamp: Instant,
    expiry: Option<Duration>,
}

impl AuthSession {
    pub fn new(user_id: &str, method: AuthMethod, expiry: Option<Duration>) -> Self {
        Self {
            user_id: user_id.to_string(),
            authenticated: true,
            method,
            timestamp: Instant::now(),
            expiry,
        }
    }
    
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.authenticated && !self.is_expired()
    }
    
    pub fn auth_method(&self) -> &AuthMethod {
        &self.method
    }
    
    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            self.timestamp.elapsed() > expiry
        } else {
            false
        }
    }
    
    pub fn invalidate(&mut self) {
        self.authenticated = false;
    }
    
    pub fn renew(&mut self) {
        self.timestamp = Instant::now();
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthManager {
    users: HashMap<String, String>, // user_id -> password_hash
    sessions: HashMap<String, AuthSession>, // session_id -> session
    failed_attempts: HashMap<String, (u32, Instant)>, // user_id -> (attempts, last_attempt)
    lock_threshold: u32,
    lock_duration: Duration,
}

impl AuthManager {
    pub fn new(lock_threshold: u32, lock_duration: Duration) -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            failed_attempts: HashMap::new(),
            lock_threshold,
            lock_duration,
        }
    }
    
    pub fn add_user(&mut self, user_id: &str, password_hash: &str) -> Result<(), &'static str> {
        if self.users.contains_key(user_id) {
            return Err("User already exists");
        }
        
        self.users.insert(user_id.to_string(), password_hash.to_string());
        Ok(())
    }
    
    pub fn remove_user(&mut self, user_id: &str) -> Result<(), &'static str> {
        if self.users.remove(user_id).is_none() {
            return Err("User does not exist");
        }
        
        // Remove any sessions for this user
        self.sessions.retain(|_, session| session.user_id != user_id);
        self.failed_attempts.remove(user_id);
        
        Ok(())
    }
    
    pub fn authenticate(&mut self, user_id: &str, password_hash: &str) -> Result<String, &'static str> {
        // Check if account is locked
        if let Some((attempts, timestamp)) = self.failed_attempts.get(user_id) {
            if *attempts >= self.lock_threshold && timestamp.elapsed() < self.lock_duration {
                return Err("Account is locked");
            }
        }
        
        // Check if user exists
        if let Some(stored_hash) = self.users.get(user_id) {
            if stored_hash == password_hash {
                // Reset failed attempts
                self.failed_attempts.remove(user_id);
                
                // Create session
                let session_id = format!("session_{}", user_id);
                let session = AuthSession::new(user_id, AuthMethod::Password, Some(Duration::from_secs(3600)));
                self.sessions.insert(session_id.clone(), session);
                
                Ok(session_id)
            } else {
                // Increment failed attempts
                let (attempts, _) = self.failed_attempts
                    .entry(user_id.to_string())
                    .or_insert((0, Instant::now()));
                
                *attempts += 1;
                
                Err("Invalid credentials")
            }
        } else {
            Err("User does not exist")
        }
    }
    
    pub fn validate_session(&self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get(session_id) {
            session.is_authenticated()
        } else {
            false
        }
    }
    
    pub fn invalidate_session(&mut self, session_id: &str) -> Result<(), &'static str> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.invalidate();
            Ok(())
        } else {
            Err("Session not found")
        }
    }
    
    pub fn renew_session(&mut self, session_id: &str) -> Result<(), &'static str> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if session.is_authenticated() {
                session.renew();
                Ok(())
            } else {
                Err("Session is not valid")
            }
        } else {
            Err("Session not found")
        }
    }
    
    pub fn cleanup_sessions(&mut self) {
        self.sessions.retain(|_, session| !session.is_expired());
    }
}