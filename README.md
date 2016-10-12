# iron-r2d2-mysql-middleware

## install

Add to `Cargo.toml`

```TOML
iron-r2d2-mysql-middleware = { git = "https://github.com/pocket7878/iron-r2d2-mysql-middleware.git" }
```

## Example

```rust
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate iron_r2d2_mysql_middleware;

use std::collections::HashMap;
use std::error::Error;
use iron::prelude::*;
use iron::status;
use router::{Router, url_for};
use hbs::{Template, HandlebarsEngine, DirectorySource};
use iron_r2d2_mysql_middleware::{MysqlMiddleware, MysqlReqExt};

fn main() {

    fn user_index(req: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();
        let mut data = HashMap::new();
        let mut con = req.db_conn();
        let mut users = vec![];
        for result in con.prep_exec("SELECT id, first_name, last_name FROM users", ()).unwrap() {
            let mut row = result.unwrap();
            let mut user_data: HashMap<String, String> = HashMap::new();
            let user_id: u64 = row.take("id").unwrap();
            let last_name: String = row.take("last_name").unwrap();
            let first_name: String = row.take("first_name").unwrap();
            user_data.insert(String::from("user_id"), format!("{}", user_id));
            user_data.insert(String::from("user_name"), format!("{} {}", last_name, first_name));
            users.push(user_data);
        }
        data.insert(String::from("users"), users);
        resp.set_mut(Template::new("user_index", data)).set_mut(status::Ok);
        return Ok(resp);
    }

    //Create Router
    let mut router = Router::new();
    router.get("/users", user_index, "user_index");

    //Create Chain
    let mut chain = Chain::new(router);
    // Add HandlerbarsEngine to middleware Chain
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(
        DirectorySource::new("./src/templates/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    chain.link_after(hbse);

    //MysqlMiddleware
    let mysql_middleware = MysqlMiddleware::new("mysql://root:12345678@localhost/test").unwrap();
    chain.link_before(mysql_middleware);

    println!("Listen on localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}

```
