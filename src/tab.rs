use csv::{StringRecord, Reader};
use std::fs::File;
use std::rc::Rc;
use std::cmp::Ordering;

pub struct Col {
    pub name: String,
    pub key: bool,
}

impl Col {
    pub fn new_vec(headers: &StringRecord, keys: &Vec<String>) -> Vec<Col> {
        let mut vec: Vec<Col> = vec!();
        for header in headers.iter() {
            vec.push(Col {
                name: String::from(header),
                key: keys.contains(&String::from(header)),
            });
        }
        vec
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Val {
    pub obj: String
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

pub struct Row {
    pub cols: Rc<Vec<Col>>,
    pub vals: Vec<Val>,
}

impl Row {
    pub fn new_vec(cols: &Rc<Vec<Col>>, mut reader: Reader<File>) -> Vec<Rc<Box<Row>>> {
        let mut vec: Vec<Rc<Box<Row>>> = vec!();
        for record in reader.records() {
            let r = Row {
                cols: Rc::clone(cols),
                vals: Val::new_vec(record.unwrap())
            };
            let rc = Rc::new(Box::new(r));
            vec.push(rc);
        }
        vec.sort_unstable();
        vec
    }

    pub fn keys(&self) -> Vec<&Val> {
        let mut vec = vec!();
        for (i, val) in self.vals.iter().enumerate() {
            if self.cols[i].key {
                vec.push(val);
            }
        }
        vec
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
    pub rows: Vec<Rc<Box<Row>>>,
}

impl Tab {
    pub fn init(mut reader: Reader<File>, keys: &Vec<String>) -> Self {
        let mut keys = keys;
        let mut vec;
        let header = reader.headers().unwrap();
        if keys.is_empty() {
            vec = Vec::with_capacity(header.len());
            for h in header.iter() {
                vec.push(String::from(h));
            }
            keys = &vec;
        }
        let cols: Rc<Vec<Col>> = Rc::from(Col::new_vec(header, keys));
        Tab {
            rows: Row::new_vec(&cols, reader),
            cols,
        }
    }
}
