use std::error::Error;

use crate::{application::service::viewer_service::ViewerService, dto::{request::{count_viewer::CountViewerRequest, viewer_create::ViewerCreateRequest}, response::{count_response::CountResponse, viewer_response::ViewerResponse}}};

pub struct ViewerUsercase{
    viewer_service: ViewerService
}

impl ViewerUsercase {
    pub fn new(viewer_service: ViewerService) -> ViewerUsercase{
        ViewerUsercase { viewer_service: viewer_service }
    }

    pub async  fn count_viewer_usecase(&self, count_request: CountViewerRequest) -> Result<CountResponse, Box<dyn Error>>{
        if count_request.channel.is_empty(){
            return Err("Channel is empty".into());
        }

        self.viewer_service.count_viewer(count_request).await
    }

    pub async fn create_viewer_usecase(
        &self,
        viewer_create: ViewerCreateRequest,
    ) -> Result<ViewerResponse, Box<dyn Error>> {
        if viewer_create.get_user_id().is_empty() {
            return Err("User ID is empty".into());
        }

        if viewer_create.get_chanel().is_empty() {
            return Err("Channel is empty".into());
        }

        if viewer_create.get_status().is_empty() {
            return Err("Status is empty".into());
        }

        self.viewer_service.create_viewer(viewer_create).await
    }

    pub async fn delete_viewer_usecase(&self, user_id: String, channel: String) -> Result<(), Box<dyn Error>>{
        if user_id.is_empty() {
            return Err("User ID is empty".into());
        }

        if channel.is_empty() {
            return Err("Channel is empty".into());
        }

        self.viewer_service.delete_viewer(user_id, channel).await
    }
}