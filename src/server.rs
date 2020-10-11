use rocket::State;

use crate::deployments;

#[get("/")]
pub fn index() -> &'static str {
    "Hello World!"
}

#[get("/config")]
pub fn config(config: State<deployments::DeployConfig>) -> String {
    format!("{:?}", config)
}