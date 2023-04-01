use std::sync::{Mutex, Arc, RwLock};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use crate::tree::{Tree, Leaf, self};

    pub enum ErrorStates 
    {
        KeyNotFound,
        DatabaseNonExistant
    } 
    pub struct State{
       pub db: Arc<RwLock<PickleDb>>,
       pub tree: Arc<RwLock<Tree>>
    }

    impl State {

        pub(crate) fn init() -> Self {
            let database = PickleDb::load("data.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap_or({
                PickleDb::new("data.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)});
           
            let tree: Tree = Tree::read_tree("/tree.json").unwrap_or({
                // Todo: Get data from peer in p2p network , if empty => spawn() 
                Tree::spawn()
            });

            Self {
                db: Arc::new(RwLock::new(database)), 
                tree: Arc::new(RwLock::new(tree)),                    
            }

        }

    }
