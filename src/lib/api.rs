use rustless::batteries::swagger;
use rustless::{Api, Application, Nesting, Versioning};
use std::default::Default;

use topics;


fn build_api(node: &mut Api) {
    node.version("v1", Versioning::AcceptHeader("shove"));

    node.mount(swagger::create_api("api-docs"));

    topics::attach_endpoints(node);
}

pub fn new() -> Application {
    let mut app = Application::new(Api::build(build_api));

    swagger::enable(&mut app, swagger::Spec {
        info: swagger::Info {
            title: "Shove".to_string(),
            description: Some("A push-oriented message broker.".to_string()),
            contact: Some(swagger::Contact {
                name: "Mark Burnett".to_string(),
                url: Some("https://mark-burnett.github.io/shove".to_string()),
                ..Default::default()
            }),
            license: Some(swagger::License {
                name: "MIT".to_string(),
                url: "http://opensource.org/licenses/MIT".to_string()
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    app
}
