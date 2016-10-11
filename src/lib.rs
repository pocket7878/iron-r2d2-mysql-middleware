
extern crate iron;
extern crate r2d2;
extern crate r2d2_mysql;

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};

use std::any::Any;
use std::error::Error;
use std::sync::Arc;

pub struct MysqlMiddleware {
  pub pool: Arc<r2d2::Pool<r2d2_mysql::MysqlConnectionManager>>
}

impl typemap::Key for MysqlMiddleware {
    type Value = Arc<r2d2::Pool<r2d2_mysql::MysqlConnectionManager>>;
}

impl MysqlMiddleware {
    pub fn new(connection_str: &str) -> Result<MysqlMiddleware, Box<Error>> {
        let config = r2d2::Config::default();
        let manager = r2d2_mysql::MysqlConnectionManager::new(connection_str).unwrap();
        let pool = try!(r2d2::Pool::new(config, manager));

        Ok(MysqlMiddleware {
          pool: Arc::new(pool),
        })
    }
}

impl BeforeMiddleware for MysqlMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<MysqlMiddleware>(self.pool.clone());
        Ok(())
    }
}

pub trait MysqlReqExt {
  fn db_conn(&self) -> r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager>;
}

impl<'a, 'b>  MysqlReqExt for Request<'a, 'b> {
  fn db_conn(&self) -> r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager> {
    let poll_value = self.extensions.get::<MysqlMiddleware>().unwrap();
    return poll_value.get().unwrap();
  }
}
