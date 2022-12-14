#![allow(unused)]
use anyhow::{anyhow, Ok, Result};
use std::collections::BTreeMap;
use std::vec;
use surrealdb::sql::{thing, Array, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

type DB = (Datastore, Session); // CREATE NEW TYPE

#[tokio::main]
async fn main() -> Result<()> {
    // USE NEW TYPE 'DB' RO DEFINE SDATASTORE AND ESSION
    let db: &DB = &(
        Datastore::new("memory").await?,
        Session::for_db("my_ns", "my_db"),
    );

    //      DECONSTRUCT TUPLE!
    let (ds, ses) = db;

    // Original definition of datastore and session.
    // let ds = Datastore::new("memory").await?;
    // let ses = Session::for_db("my_ns", "my_db");

    // ==============
    // CREATE RECORDS
    // ==============

    //      CREATE - NOT USING PARAmetERIZED CREATE FUNCTION!!
    // let sql = "CREATE task SET title = 'Task 01', priority = 10";
    // let ress = ds.execute(sql, ses, None, false).await?;

    // let sql = "CREATE task SET title = 'Task 02', priority = 5";
    // let ress = ds.execute(sql, ses, None, false).await?;

    //      CREATE - USING THE PARAMETERIZED CREATE FUNCTION!!
    let t1 = create_task(db, "TASK 01", 15).await?;
    let t2 = create_task(db, "TASK 02", 150).await?;

    let mut task_array = Vec::new();
    task_array.push(t1.clone());
    task_array.push(t2.clone());

    print_string_vector(task_array);

    // ======================
    // MERGE [UPDATE] RECORDS
    // ======================

    let sql = "UPDATE $th MERGE $data RETURN id";
    let data: BTreeMap<String, Value> = [
        ("title".into(), "Task 02 UPDATED".into()),
        ("done".into(), true.into()), // Add a new property on the fly!
    ]
    .into();

    let vars: BTreeMap<String, Value> = [
        ("th".into(), thing(&t2)?.into()),
        ("data".into(), data.into()),
    ]
    .into();

    ds.execute(sql, ses, Some(vars), true).await?;

    // ==============
    // DELETE RECORDS
    // ==============
    let sql = "DELETE $th";
    let vars: BTreeMap<String, Value> = [("th".into(), thing(&t1)?.into())].into();
    ds.execute(sql, ses, Some(vars), true).await?;

    // ==============
    // SELECT RECORDS
    // ==============

    //  SELECT
    let sql = "SELECT * from task";
    let ress = ds.execute(sql, ses, None, false).await?;

    //  PRINT RETURNED OBJECTS
    // println!("{:?}", ress);

    //  PARSE RETURNED OBJECTS ONE BY ONE
    for object in into_iter_objects(ress)? {
        println!("record {:?}\n", object?); // Print the whole record.
                                            // println!("record {:?}", object?.get("id")); // Print a particular field.
    }

    Ok(())
}

// CREATE function.
async fn create_task((ds, ses): &DB, title: &str, priority: i32) -> Result<String> {
    let sql = "CREATE task CONTENT $data"; // NOTE: CONTENT, $data!

    let data: BTreeMap<String, Value> = [
        ("title".into(), title.into()),
        ("priority".into(), priority.into()),
    ]
    .into();

    let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

    let ress = ds.execute(sql, ses, Some(vars), false).await?;

    into_iter_objects(ress)?
        .next()
        .transpose()?
        .and_then(|obj| obj.get("id").map(|id| id.to_string()))
        .ok_or_else(|| anyhow!("No id returned."))
}

// FUNCTION TO RETURN RESULT OF AN ITEM ITERATOR
//      Result<impl Iterator<Item = Result<Object>>>
fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow!("A record was not an Object")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("No records were found!")),
    }
}

// FUNCTION TO PRINT AN ARRAY OF CREATED RECORD IDs
fn print_string_vector(vec: Vec<String>) {
    println!(
        "\n\n==================================\nRECORDS:\n=================================="
    );
    for el in vec {
        println!("\t{}", el);
    }
    println!("==================================\n\n");
}
