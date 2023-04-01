use std::sync::{Mutex, Arc};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

    pub enum ErrorStates 
    {
        KeyNotFound,
        DatabaseNonExistant
    } 
    pub struct State{
       pub db: Arc<Mutex<PickleDb>>,
    }

    impl State {

        pub(crate) fn init() -> Self {
            Self {db: Arc::new(Mutex::new(PickleDb::load("data.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap_or({
            PickleDb::new("data.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)})))}

        }

    }