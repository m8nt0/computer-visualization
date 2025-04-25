use super::error::{PackageError, PackageResult};
use std::collections::HashMap;

pub struct PackageManager {
    packages: HashMap<PackageId, Package>,
    repositories: Vec<Repository>,
    installed: HashMap<PackageId, InstalledPackage>,
    dependencies: DependencyGraph,
    config: PackageConfig,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct PackageId {
    name: String,
    version: Version,
}

struct Package {
    id: PackageId,
    description: String,
    dependencies: Vec<Dependency>,
    files: Vec<PackageFile>,
    scripts: PackageScripts,
    checksum: String,
}

struct Repository {
    url: String,
    priority: u8,
    packages: HashMap<PackageId, PackageMetadata>,
    last_update: Timestamp,
}

impl PackageManager {
    pub fn new(config: PackageConfig) -> Self {
        Self {
            packages: HashMap::new(),
            repositories: Vec::new(),
            installed: HashMap::new(),
            dependencies: DependencyGraph::new(),
            config,
        }
    }

    pub fn install(&mut self, package_id: PackageId) -> PackageResult<()> {
        // Check if already installed
        if self.installed.contains_key(&package_id) {
            return Ok(());
        }

        // Resolve dependencies
        let deps = self.resolve_dependencies(&package_id)?;
        
        // Download package and dependencies
        let packages = self.download_packages(&deps)?;
        
        // Install in correct order
        for pkg in packages {
            self.install_single_package(pkg)?;
        }

        Ok(())
    }

    pub fn remove(&mut self, package_id: PackageId) -> PackageResult<()> {
        // Check reverse dependencies
        if let Some(rdeps) = self.dependencies.reverse_deps(&package_id) {
            return Err(PackageError::HasDependents(rdeps));
        }

        let package = self.installed.remove(&package_id)
            .ok_or(PackageError::NotInstalled)?;

        // Run pre-removal script
        package.scripts.pre_remove()?;

        // Remove files
        for file in package.files {
            self.remove_file(&file)?;
        }

        // Run post-removal script
        package.scripts.post_remove()?;

        Ok(())
    }

    pub fn update(&mut self) -> PackageResult<()> {
        // Update repository metadata
        for repo in &mut self.repositories {
            repo.update()?;
        }

        // Find upgradeable packages
        let upgrades = self.find_upgrades()?;

        // Perform upgrades
        for (old_id, new_id) in upgrades {
            self.upgrade_package(old_id, new_id)?;
        }

        Ok(())
    }
} 