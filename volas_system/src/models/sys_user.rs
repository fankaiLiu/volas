use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SysUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    password: &'a str,
}
