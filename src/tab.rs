use csv::{StringRecord, Reader};
use std::fs::File;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeTuple};

#[derive(Clone)]
pub struct Col {
    name: Box<String>,
}

impl Col {
    pub fn new_vec(headers: &StringRecord) -> Vec<Col> {
        let mut vec: Vec<Col> = vec!();
        for header in headers.iter() {
            vec.push(Col {
                name: Box::new(String::from(header))
            });
        }
        vec
    }
}

pub struct Val {
    col: Col,
    obj: String
}

impl Val<> {
    pub fn new_vec(cols: &Vec<Col>, record: StringRecord) -> Vec<Val> {
        let mut vec: Vec<Val> = vec!();
        let mut i = 0;
        for val in record.iter() {
           vec.push(Val {
               col: cols.get(i).unwrap().to_owned(),
               obj: String::from(val),
           });
           i += 1;
        }
        vec
    }
}

pub struct Row {
    vals: Vec<Val>,
}

impl Row {
    pub fn new_vec(cols: &Vec<Col>, mut reader: Reader<File>) -> Vec<Row> {
        let mut vec: Vec<Row> = vec!();
        for record in reader.records() {
            vec.push(Row {
                vals: Val::new_vec(cols, record.unwrap())
            });
        }
        vec
    }
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_tuple(self.vals.len())?;
        for val in &self.vals {
            s.serialize_element(val.obj.as_str())?;
        }
        s.end()
    }
}

pub struct Tab {
    pub cols: Vec<Col>,
    pub rows: Vec<Row>,
}

impl From<Reader<File>> for Tab {
    fn from(mut reader: Reader<File>) -> Self {
        let cols = Col::new_vec(reader.headers().unwrap());
        Tab {
            rows: Row::new_vec(&cols, reader),
            cols,
        }
    }
}

impl Serialize for Tab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_seq(Option::Some(self.rows.len()))?;
        for row in &self.rows {
            s.serialize_element(&row)?;
        }
        s.end()
    }
}
