use polars::error::ArrowError::NotYetImplemented;
use polars::prelude::*;

use crate::io;

// TODO: store and retrieve from storage

/// Store `IOEvent` object in a polars dataframe
pub struct Container {
    df: DataFrame,
}

impl Container {
    fn insert<T>(&mut self, event: &io::IOEvent<T>) -> bool {
        unimplemented!()
    }
    fn by_device(&self, device_id: &i32) -> DataFrame {
        unimplemented!()
    }
    fn last(&self, n: i32) -> DataFrame {
        unimplemented!()
    }
    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn new() -> Self {
        let df = DataFrame::from(&io::IOEvent::<f64>::schema());

        Container { df }
    }

    // Compare schema of internal `DataFrame` against another
    fn check_schema(&self, other: &DataFrame) -> bool {
        self.df.schema() == other.schema()
    }
}