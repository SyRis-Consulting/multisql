use bincode;
use nom_sql::CreateTableStatement;
use sled::{self, Db, Iter};

use crate::executor::Row;
use crate::storage::Store;

struct ResultSet {
    iter: Iter,
}

impl Iterator for ResultSet {
    type Item = Row;

    fn next(&mut self) -> Option<Row> {
        self.iter
            .next()
            .and_then(|result| result.ok())
            .map(|(_, value)| bincode::deserialize(&value).expect("Stop iterate"))
    }
}

pub struct SledStorage {
    tree: Db,
}

impl SledStorage {
    pub fn new(filename: String) -> SledStorage {
        let tree = sled::open(filename).unwrap();

        SledStorage { tree }
    }
}

impl Store for SledStorage {
    fn set_schema(&self, statement: CreateTableStatement) -> Result<(), ()> {
        let k = format!("schema/{}", &statement.table.name);
        let k = k.as_bytes();
        let v: Vec<u8> = bincode::serialize(&statement).unwrap();

        self.tree.insert(k, v).unwrap();

        Ok(())
    }

    fn get_schema(&self, table_name: &str) -> Result<CreateTableStatement, &str> {
        let k = format!("schema/{}", table_name);
        let k = k.as_bytes();
        let v: &[u8] = &self.tree.get(&k).unwrap().unwrap();
        let statement = bincode::deserialize(v).unwrap();

        Ok(statement)
    }

    fn set_data(&self, table_name: &str, row: Row) -> Result<(), ()> {
        let k = format!("data/{}/{}", table_name, row.key);
        let k = k.as_bytes();
        let v: Vec<u8> = bincode::serialize(&row).unwrap();

        self.tree.insert(k, v).unwrap();

        Ok(())
    }

    fn get_data(&self, table_name: &str) -> Result<Box<dyn Iterator<Item = Row>>, ()> {
        let k = format!("data/{}/", table_name);
        let k = k.as_bytes();

        let iter = self.tree.scan_prefix(k);
        let result_set = ResultSet { iter };

        Ok(Box::new(result_set))
    }
}
