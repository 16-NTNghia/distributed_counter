use std::{error::Error, sync::Arc};

use scylla::{client::session::Session, value::Counter};

use crate::domain::schema::counter::CounterViewer;

pub struct CounterViewerRepository {
    session: Arc<Session>,
}

impl CounterViewerRepository {
    pub fn new(session: Arc<Session>) -> CounterViewerRepository {
        CounterViewerRepository { session: session }
    }

    pub async fn add_viewer(&self, channel: String) -> Result<CounterViewer, Box<dyn Error>> {
        // Update counter
        let statement =
            "UPDATE count_view.counter_viewers SET viewers = viewers + 1 WHERE channel = ?";
        self.session.query_unpaged(statement, (&channel,)).await?;

        // Get the updated counter value - now this should work!
        let select_statement =
            "SELECT channel, viewers FROM count_view.counter_viewers WHERE channel = ?";
        let result = self
            .session
            .query_unpaged(select_statement, (&channel,))
            .await?;

        let rows_result = result.into_rows_result()?;
        let mut rows = rows_result.rows::<CounterViewer>()?; // This should work now

        if let Some(row) = rows.next() {
            let counter_viewer = row?;
            println!(
                "Updated viewer count: {}",
                counter_viewer.get_viewers().0
            );
            return Ok(counter_viewer);
        }

        Err("Failed to retrieve updated counter".into())
    }

    pub async fn sub_viewer(&self, channel: String) -> Result<CounterViewer, Box<dyn Error>> {
        let statement =
            "UPDATE count_view.counter_viewers SET viewers = viewers - 1 WHERE channel = ?";
        let values = (&channel,);
        self.session.query_unpaged(statement, values).await?;

        // Get the updated counter value
        let select_statement = "SELECT * FROM count_view.counter_viewers WHERE channel = ?";
        let result = self.session.query_unpaged(select_statement, values).await?;

        let rows_result = result.into_rows_result()?;
        let mut rows = rows_result.rows::<CounterViewer>()?;

        if let Some(row) = rows.next() {
            let counter_viewer = row?;
            return Ok(counter_viewer);
        }

        Err("Failed to retrieve updated counter".into())
    }

    pub async fn get_viewer(&self, channel: String) -> Result<i64, Box<dyn Error>> {
        let statement = "SELECT viewers FROM count_view.counter_viewers WHERE channel = ?";
        let values = (&channel,);
        let result = self
            .session
            .query_unpaged(statement, values)
            .await?
            .into_rows_result()?;

        let mut rows = result.rows::<(Counter,)>()?;

        // Option 1: Use if let with next()
        if let Some(row) = rows.next() {
            let (viewers,) = row?; // Destructure the tuple to get the i64
            return Ok(viewers.0);
        }

        Err("channel not found".into())
    }
}
