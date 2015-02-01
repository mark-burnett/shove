use rustless::{Client, HandleResult, Nesting};
use serialize::json;

fn get_list<'a, 'r>(client: Client<'a>, _params: &'r json::Object
               ) -> HandleResult<Client<'a>> {
    client.json(&json::Json::Array(vec!(
                json::Json::String("Hi".to_string()),
                json::Json::String("There".to_string()),
            )
        ))
}

pub fn attach_endpoints<T: Nesting>(node: &mut T) {
    node.get("topics", |endpoint| {
        endpoint.summary("List known topics");
        endpoint.desc("Describe existing topics");
        endpoint.handle(get_list)
    });
}
