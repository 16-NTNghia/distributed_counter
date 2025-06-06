use crate::{
    domain::schema::viewer::Viewer, dto::{
        request::{count_viewer::CountViewerRequest, viewer_create::ViewerCreateRequest},
        response::{count_response::CountResponse, viewer_response::ViewerResponse},
    }, infrastructure::scylladb::repository::viewer_repository::ViewerRepository
};
use std::error::Error;

pub struct ViewerService {
    viewer_repository: ViewerRepository,
}

impl ViewerService {
    pub fn new(viewer_repository: ViewerRepository) -> ViewerService {
        ViewerService { viewer_repository }
    }

    pub async fn create_viewer(
        &self,
        viewer_create: ViewerCreateRequest,
    ) -> Result<ViewerResponse, Box<dyn Error>> {
        // Chuyển ViewerCreateRequest thành Viewer
        let viewer: Viewer = viewer_create.into();

        // Gọi save với Viewer
        let saved_viewer = self.viewer_repository.save(viewer).await?;

        // Trả về ViewerResponse
        Ok(ViewerResponse::from(saved_viewer))
    }

    pub async fn count_viewer(&self, count_request: CountViewerRequest) -> Result<CountResponse, Box<dyn Error>> {
        let count = self.viewer_repository.count_by_channel(count_request.channel).await?;
        
        Ok(CountResponse::from(count))
    }

    pub async fn delete_viewer(&self, user_id: String, channel: String) -> Result<(), Box<dyn Error>> {
        self.viewer_repository.delete_by_userid_and_channel(user_id, channel).await?;
        Ok(())
    }
}
