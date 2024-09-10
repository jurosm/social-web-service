use serde::{Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};

use crate::posts::schema::ResponsePost;

#[derive(Serialize, ToSchema, Deserialize)]
#[aliases(ListOfPosts = ListResponse<ResponsePost>)]
pub struct ListResponse<T: PartialSchema> {
    pub data: Vec<T>,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
}
