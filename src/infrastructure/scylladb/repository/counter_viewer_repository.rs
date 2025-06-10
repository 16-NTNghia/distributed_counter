use std::{error::Error, sync::Arc};

use scylla::{client::session::Session, value::Counter};

pub struct CounterViewerRepository {
    session: Arc<Session>,
}

impl CounterViewerRepository {
    pub fn new(session: Arc<Session>) -> CounterViewerRepository {
        CounterViewerRepository { session: session }
    }

    pub async fn add_viewer(&self, channel: String) -> Result<(), Box<dyn Error>> {
        // Update counter
        let statement =
            "UPDATE count_view.counter_viewers SET viewers = viewers + 1 WHERE channel = ?";
        self.session.query_unpaged(statement, (&channel,)).await?;

        Ok(())
    }

    pub async fn sub_viewer(&self, channel: String) -> Result<(), Box<dyn Error>> {
        let statement =
            "UPDATE count_view.counter_viewers SET viewers = viewers - 1 WHERE channel = ?";

        let values = (&channel,);
        self.session.query_unpaged(statement, values).await?;

        Ok(())
    }

    pub async fn update_viewer(&self, term: i64, channel: String) -> Result<(), Box<dyn Error>> {
        println!("term: {}", term);

        let counter = Counter(term);

        let statement =
            "UPDATE count_view.counter_viewers SET viewers = viewers + ? WHERE channel = ?";

        let values = (&counter, &channel);
        self.session.query_unpaged(statement, values).await?;

        Ok(())
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
