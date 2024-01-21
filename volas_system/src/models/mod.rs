use serde::Serialize;

#[derive(Debug, Serialize)]
struct SysUser<'a> {
    first_name:  &'a str,
    last_name:  &'a str,
    email:  &'a str,
    password:  &'a str,
}

