
use super::{supplier::Supplier, product};
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manager{
    name: String,
    pub suppliers: Vec<Supplier>
}

impl Manager{
    

    // Getter methods
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    #[allow(dead_code)]
    pub fn suppliers(&self) -> &[Supplier] {
        self.suppliers.as_ref()
    }

    // Setter methods
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    #[allow(dead_code)]
    pub fn set_suppliers(&mut self, suppliers: Vec<Supplier>) {
        self.suppliers = suppliers;
    }

    // Public methods
    pub fn new() -> Manager{
        Manager{name: String::new(), suppliers: Vec::new()}
    }
    pub fn add_supplier(&mut self, p: Supplier){
        self.suppliers.push(p);
    }
    pub fn remove_supplier_from_name(&mut self, name: &str){
        if let Some(pos) = self.suppliers.iter().position(|x| x.name() == name) {
            self.suppliers.remove(pos);
        }
    }
    pub fn exists_supplier(&self, name: &str) -> bool{
        if let Some(_pos) = self.suppliers.iter().position(|x| x.name() == name) {
            return true
        }
        false
    }
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.suppliers().iter(){
            for j in i.products().iter(){
                product::update_MAXLEN(&j.name());
            }
        }
        let mut s: String = String::from("* ")+self.name().into()+"\n";
        for i in self.suppliers().iter(){
            s.push_str(&i.to_string());
        }
        write!(f, "{}", s)
    }
}