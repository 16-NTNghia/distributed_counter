use std::sync::Arc;

use scylla::client::session::Session;

use crate::{
    application::{
        service::{counter_viewer_service::CounterViewerService, viewer_service::ViewerService},
        usecase::{counter_viewer_usecase::CounterViewerUsercase, viewer_usecase::ViewerUsercase},
    },
    infrastructure::scylladb::{repository::{counter_viewer_repository::CounterViewerRepository, viewer_repository::ViewerRepository}},
};

#[derive(Clone)]
pub struct AppState {
    pub viewer_usecase: Arc<ViewerUsercase>,
    pub counter_viewer_usecase: Arc<CounterViewerUsercase>,
}

pub fn build_app(session: Session) -> AppState {

    let session = Arc::new(session);

    let viewer_repo = ViewerRepository::new(Arc::clone(&session));
    let viewer_service = ViewerService::new(viewer_repo);
    let viewer_usecase = ViewerUsercase::new(viewer_service);

    let counter_viewer_repo = CounterViewerRepository::new(Arc::clone(&session));
    let counter_viewer_service = CounterViewerService::new(counter_viewer_repo);
    let counter_viewer_usecase = CounterViewerUsercase::new(counter_viewer_service);

    AppState {
        viewer_usecase: viewer_usecase.into(),
        counter_viewer_usecase: counter_viewer_usecase.into(),
    }
}
