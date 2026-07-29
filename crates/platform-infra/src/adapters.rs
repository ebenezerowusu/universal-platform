use platform_core::PlatformResult;

pub trait DatabaseAdapter: Send + Sync {
    fn name(&self) -> &'static str;

    fn readiness_summary(&self) -> PlatformResult<String> {
        Ok(format!("{} readiness check not wired yet", self.name()))
    }
}

pub trait CacheAdapter: Send + Sync {
    fn name(&self) -> &'static str;

    fn readiness_summary(&self) -> PlatformResult<String> {
        Ok(format!("{} readiness check not wired yet", self.name()))
    }
}

pub trait StorageAdapter: Send + Sync {
    fn name(&self) -> &'static str;

    fn readiness_summary(&self) -> PlatformResult<String> {
        Ok(format!("{} readiness check not wired yet", self.name()))
    }
}
