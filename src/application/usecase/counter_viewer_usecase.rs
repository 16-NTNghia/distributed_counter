use std::error::Error;

use crate::{application::service::counter_viewer_service::CounterViewerService, dto::response::{count_response::CountResponse, }};


pub struct CounterViewerUsercase {
    cv_service : CounterViewerService
}

impl CounterViewerUsercase {
    pub fn new(cv_service: CounterViewerService) -> CounterViewerUsercase {
        CounterViewerUsercase { cv_service: cv_service }
    }

    pub async fn add_viewer_usecase(&self, channel: String) -> Result<(), Box<dyn Error>>{
        self.cv_service.add_viewer(channel).await
    }

    pub async fn sub_viewer_usecase(&self, channel: String) -> Result<(), Box<dyn Error>>{
        self.cv_service.sub_viewer(channel).await
    }

    pub async fn update_viewer_usecase(&self, term: i64, channel: String) -> Result<(), Box<dyn Error>>{
        self.cv_service.update_viewer(term, channel).await
    }

    pub async fn get_viewer_usecase(&self, channel: String) -> Result<CountResponse, Box<dyn Error>>{
        self.cv_service.get_viewer(channel).await
    }
}