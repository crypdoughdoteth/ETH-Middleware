use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use crate::ErrorStates;


pub(crate) fn query_db(key: &str) -> Result<String, ErrorStates> {
    let db = PickleDb::load("data.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap();
    match db.get::<String>(key){
        Some(x) => Ok(x),
        None => Err(ErrorStates::KeyNotFound),
    }
}

pub(crate) fn set_kv(key: &str, value: String) -> Result<(), ErrorStates> {
    let mut db = PickleDb::load("data.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap();
    db.set(key, &value).unwrap();
    Ok(())
}   