use std::sync::Arc;

use crate::{
    domain::houses::{
        entities::residential::Residential,
        events::residential::{NewResidentialEvent, UpdateResidentialEvent},
    },
    infrastructure::repositories::mysql_residential_repository::MysqlResidentialRepository,
};

pub struct ResidentialService {
    repo: Arc<MysqlResidentialRepository>,
}

impl ResidentialService {
    pub fn new(repo: Arc<MysqlResidentialRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, event: NewResidentialEvent) -> Result<(), diesel::result::Error> {
        self.repo.create(event).await
    }

    pub async fn update(&self, event: UpdateResidentialEvent) -> Result<(), diesel::result::Error> {
        self.repo.update(&event).await
    }

    pub async fn list(&self) -> Vec<Residential> {
        self.repo.list().await
    }
}
