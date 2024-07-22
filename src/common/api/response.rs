use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
}
