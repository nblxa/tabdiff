use csv::{StringRecord, Reader};
use std::fs::File;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
use std::rc::Rc;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Col {
    name: String,
}

impl Col {
    pub fn new_vec(headers: &StringRecord) -> Vec<Col> {
        let mut vec: Vec<Col> = vec!();
        for header in headers.iter() {
            vec.push(Col {
                name: String::from(header)
            });
        }
        vec
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Val {
    obj: String
}

impl Val<> {
    pub fn new_vec(record: StringRecord) -> Vec<Val> {
        let mut vec: Vec<Val> = vec!();
        for val in record.iter() {
           vec.push(Val {
               obj: String::from(val),
           });
        }
        vec
    }
}

impl Serialize for Val {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.obj.as_str())
    }
}

pub struct Row {
    cols: Rc<Vec<Col>>,
    vals: Vec<Val>,
}

impl Row {
    pub fn new_vec(cols: &Rc<Vec<Col>>, mut reader: Reader<File>) -> Vec<Row> {
        let mut vec: Vec<Row> = vec!();
        for record in reader.records() {
            vec.push(Row {
                cols: Rc::clone(cols),
                vals: Val::new_vec(record.unwrap())
            });
        }
        vec.sort_unstable();
        vec
    }
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(self.vals.len()))?;
        for (i, val) in self.vals.iter().enumerate() {
            s.serialize_entry(&self.cols[i].name, &val)?;
        }
        s.end()
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.vals, &other.vals)
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.vals, &other.vals)
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.vals == other.vals
    }
}

impl Eq for Row {}

pub struct Tab {
    pub cols: Rc<Vec<Col>>,
    pub rows: Vec<Row>,
}

impl From<Reader<File>> for Tab {
    fn from(mut reader: Reader<File>) -> Self {
        let cols: Rc<Vec<Col>> = Rc::from(Col::new_vec(reader.headers().unwrap()));
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
