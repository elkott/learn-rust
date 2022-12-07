#![allow(unused)]
use anyhow::{anyhow, Ok, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

#[tokio::main]
async fn main() -> Result<()> {
    let ds = Datastore::new("memory").await?;
    let ses = Session::for_db("my_ns", "my_db");

    // CREATE
    let sql = "CREATE task SET title = 'Task 01', priority = 10";
    let ress = ds.execute(sql, &ses, None, false).await?;

    let sql = "CREATE task SET title = 'Task 02', priority = 5";
    let ress = ds.execute(sql, &ses, None, false).await?;

    // SELECT
    let sql = "SELECT * from task";
    let ress = ds.execute(sql, &ses, None, false).await?;
    println!("{:?}", ress);

    Ok(())
}
