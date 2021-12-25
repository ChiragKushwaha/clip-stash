use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug,Constructor, Deserialize, Serialize)]
pub struct Hits(u64);

imp Hits {
    pub into_inner(self) -> u64{
        self.0;
    }
}