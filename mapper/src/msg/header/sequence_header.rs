use derive_macro::{Deserialize, Serialize};

use crate::{Deserialize, Serialize,MapperResult};

#[derive(Debug, Serialize, Deserialize)]
pub (crate)  struct SequenceHeader {
    pub(crate) sequence_number: u32,
    pub(crate) request_id: u32,
}
