use crate::domain::schema::viewer::Viewer;
use scylla::client::session::Session;
use std::{error::Error, sync::Arc};

pub struct ViewerRepository {
    session: Arc<Session>,
}

impl ViewerRepository {
    pub fn new(session: Arc<Session>) -> ViewerRepository {
        ViewerRepository { session: session }
    }

    pub async fn save(&self, viewer: Viewer) -> Result<Viewer, Box<dyn Error>> {
        let statement =
            "INSERT INTO count_view.viewers (user_id, channel, create_at) VALUES (?, ?, ?)";

        let values = (
            &viewer.get_user_id(),
            &viewer.get_channel(),
            &viewer.get_create_at(),
        );
        self.session.query_unpaged(statement, values).await?;

        // println!("Saved successfully");

        Ok(viewer)
    }

    pub async fn count_by_channel(&self, channel: String) -> Result<i64, Box<dyn Error>> {
        let statement = "SELECT count(user_id) FROM count_view.viewers WHERE channel = ?";
        let values = (&channel,);

        let query_result = self.session.query_unpaged(statement, values).await?;

        if query_result.is_rows() {
            let rows_result = query_result.into_rows_result()?; // từ tài liệu
            let mut rows = rows_result.rows::<(i64,)>()?;

            if let Some(row) = rows.next() {
                let (count,) = row?;
                println!("viewer in channel {}: {}", channel, count);
                return Ok(count);
            }
        }

        Ok(0)
    }

    pub async fn delete_by_userid_and_channel(
        &self,
        user_id: String,
        channel: String,
    ) -> Result<(), Box<dyn Error>> {

        let statement = "DELETE FROM count_view.viewers WHERE user_id = ? AND channel = ?";
        let values = (&user_id, &channel,);
        self.session.query_unpaged(statement, values).await?;

        // println!("Deleted successfully");

        Ok(())
    }
}
