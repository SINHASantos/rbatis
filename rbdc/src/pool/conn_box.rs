use crate::db::{Connection, ExecResult, Row};
use crate::pool::conn_manager::ConnManager;
use crate::Error;
use futures_core::future::BoxFuture;
use rbs::Value;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub struct ConnectionBox {
    pub conn: Option<Box<dyn Connection>>,
    pub manager_proxy: ConnManager,
    pub auto_close: bool,
}
impl Debug for ConnectionBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionBox")
            .field("manager_proxy", &self.manager_proxy)
            .field("auto_close", &self.auto_close)
            .finish()
    }
}

unsafe impl Sync for ConnectionBox {}

impl Connection for ConnectionBox {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.deref_mut().get_rows(sql, params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.deref_mut().exec(sql, params)
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().close()
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().ping()
    }

    fn begin(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().begin()
    }
    fn commit(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().commit()
    }
    fn rollback(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().rollback()
    }
}

impl Deref for ConnectionBox {
    type Target = Box<dyn Connection>;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl DerefMut for ConnectionBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

impl Drop for ConnectionBox {
    fn drop(&mut self) {
        if self.auto_close {
            if let Some(mut conn) = self.conn.take() {
                self.manager_proxy.spawn_task(async move {
                    let _ = conn.close().await;
                });
            }
        }
    }
}
