use std::error::Error;

use crate::{dto::response::{count_response::CountResponse, counter_viewer_response::CounterViewerResponse}, infrastructure::scylladb::repository::counter_viewer_repository::CounterViewerRepository};


pub struct CounterViewerService {
    cv_repository: CounterViewerRepository
}

impl CounterViewerService {
    pub fn new (cv_repository: CounterViewerRepository) -> CounterViewerService {
        CounterViewerService { cv_repository: cv_repository }
    }

    pub async fn add_viewer (&self, channel: String) -> Result<(), Box<dyn Error>> {
        let counter = self.cv_repository.add_viewer(channel).await?;

        Ok(())
    }

    pub async fn sub_viewer (&self, channel: String) -> Result<CounterViewerResponse, Box<dyn Error>> {
        let counter = self.cv_repository.sub_viewer(channel).await?;

        Ok(CounterViewerResponse::from(counter))
    }

    pub async fn get_viewer (&self, channel: String) -> Result<CountResponse, Box<dyn Error>> {
        let viewer =self.cv_repository.get_viewer(channel).await?;

        Ok(CountResponse::from(viewer))
    }
}