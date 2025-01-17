use semver::Version;

use crate::data::qpackages;

use super::DependencyRepository;



pub struct QPMRepository {}

impl QPMRepository {
    pub fn new() -> Self {
        QPMRepository {  }
    }
}

impl DependencyRepository for QPMRepository {
    fn get_versions(&self, id: &str) -> Option<Vec<crate::data::qpackages::PackageVersion>> {
        qpackages::get_versions(id)
    }

    fn get_shared_package(&self, id: &str, version: &Version) -> Option<crate::data::package::SharedPackageConfig> {
        qpackages::get_shared_package(id, version)
    }
}