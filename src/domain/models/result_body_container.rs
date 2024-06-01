use serde::Serialize;
use utoipa::ToSchema;

use crate::infra::repositories::{targets_repository::Target, upstream_repository::Upstream};

#[derive(Serialize, ToSchema)]
#[aliases(
    ResultBodyContainerTarget = ResultBodyContainer<Target>,
    ResultBodyContainerUpstream = ResultBodyContainer<Upstream>,
)]
pub struct ResultBodyContainer<T> {
    pub success: bool,
    pub result: T,
    pub errors: Vec<String>,
}

impl<T> ResultBodyContainer<T> {
    pub fn success(result: T) -> ResultBodyContainer<T> {
        ResultBodyContainer {
            success: true,
            result,
            errors: vec![],
        }
    }

    pub fn error(errors: Vec<String>) -> ResultBodyContainer<Option<T>> {
        ResultBodyContainer {
            success: false,
            result: None,
            errors,
        }
    }
}
