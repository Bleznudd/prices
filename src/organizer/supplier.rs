
use super::product;
use super::product::Product;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplier{
    name: String,
    pub products: Vec<Product>
}

impl Supplier{
    

    // Getter methods
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    #[allow(dead_code)]
    pub fn products(&self) -> &[Product] {
        self.products.as_ref()
    }

    // Setter methods
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    #[allow(dead_code)]
    pub fn set_products(&mut self, products: Vec<Product>) {
        self.products = products;
    }

    // Public methods
    #[allow(dead_code)]
    pub fn new() -> Supplier{
        Supplier{name: String::new(), products: Vec::new()}
    }
    pub fn from_name(name: &str) -> Supplier{
        Supplier{name: name.into(), products: Vec::new()}
    }
    pub fn add_product(&mut self, p: Product){
        self.products.push(p);
    }
    pub fn remove_product_from_name(&mut self, name: &str){
        if let Some(pos) = self.products.iter().position(|x| x.name() == name) {
            self.products.remove(pos);
        }
    }
    pub fn exists_product(&self, name: &str) -> bool{
        if let Some(_pos) = self.products.iter().position(|x| x.name() == name) {
            true
        }
        else{
            false
        }
    }
    
}

impl std::fmt::Display for Supplier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.products().iter(){
            product::update_MAXLEN(&i.name());
        }
        let mut s: String = String::from("+ ")+self.name().into()+"\n";
        for i in self.products().iter(){
            s.push_str(&i.to_string());
        }
        write!(f, "{}", s)
    }
}