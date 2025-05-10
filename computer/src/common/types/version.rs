// Version numbering system
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
    build: Option<String>,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }
    
    pub fn with_pre_release(mut self, pre_release: &str) -> Self {
        self.pre_release = Some(pre_release.to_string());
        self
    }
    
    pub fn with_build(mut self, build: &str) -> Self {
        self.build = Some(build.to_string());
        self
    }
    
    pub fn major(&self) -> u32 {
        self.major
    }
    
    pub fn minor(&self) -> u32 {
        self.minor
    }
    
    pub fn patch(&self) -> u32 {
        self.patch
    }
    
    pub fn pre_release(&self) -> Option<&str> {
        self.pre_release.as_deref()
    }
    
    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }
    
    pub fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self.pre_release = None;
        self.build = None;
    }
    
    pub fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
        self.pre_release = None;
        self.build = None;
    }
    
    pub fn increment_patch(&mut self) {
        self.patch += 1;
        self.pre_release = None;
        self.build = None;
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare major, minor, patch
        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
        
        match self.minor.cmp(&other.minor) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
        
        match self.patch.cmp(&other.patch) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
        
        // Pre-release versions have lower precedence than the associated normal version
        match (&self.pre_release, &other.pre_release) {
            (None, Some(_)) => return Ordering::Greater,
            (Some(_), None) => return Ordering::Less,
            (None, None) => {}
            (Some(a), Some(b)) => return a.cmp(b),
        }
        
        // Build metadata does not affect precedence
        Ordering::Equal
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        
        if let Some(build) = &self.build {
            write!(f, "+{}", build)?;
        }
        
        Ok(())
    }
}

impl FromStr for Version {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse basic version (major.minor.patch)
        let mut parts = s.split('-');
        let version_part = parts.next().unwrap(); // Safe unwrap, split always returns at least one element
        
        let mut pre_release = None;
        let mut build = None;
        
        // Parse pre-release and build metadata if present
        if let Some(rest) = parts.next() {
            let mut rest_parts = rest.split('+');
            pre_release = Some(rest_parts.next().unwrap().to_string()); // Safe unwrap, split always returns at least one element
            
            if let Some(build_part) = rest_parts.next() {
                build = Some(build_part.to_string());
            }
        } else if s.contains('+') {
            let mut build_parts = s.split('+');
            build_parts.next(); // Skip version part
            if let Some(build_part) = build_parts.next() {
                build = Some(build_part.to_string());
            }
        }
        
        // Parse major.minor.patch
        let version_parts: Vec<&str> = version_part.split('.').collect();
        if version_parts.len() != 3 {
            return Err(format!("Invalid version format: {}", s));
        }
        
        let major = version_parts[0].parse::<u32>()
            .map_err(|_| format!("Invalid major version: {}", version_parts[0]))?;
            
        let minor = version_parts[1].parse::<u32>()
            .map_err(|_| format!("Invalid minor version: {}", version_parts[1]))?;
            
        let patch = version_parts[2].parse::<u32>()
            .map_err(|_| format!("Invalid patch version: {}", version_parts[2]))?;
        
        Ok(Version {
            major,
            minor,
            patch,
            pre_release,
            build,
        })
    }
}