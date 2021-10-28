#[allow(dead_code)]

use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product{
    name: String,
    buyprice: f32,
    taxpercentage: f32,
    gainpercentage: f32,
    sellprice: f32,
}

static mut MAXLEN: usize = 0;

impl Product{

    // Getters
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    #[allow(dead_code)]
    pub fn buyprice(&self) -> f32 {
        self.buyprice
    }
    #[allow(dead_code)]
    pub fn taxpercentage(&self) -> f32 {
        self.taxpercentage
    }
    #[allow(dead_code)]
    pub fn gainpercentage(&self) -> f32 {
        self.gainpercentage
    }
    #[allow(dead_code)]
    pub fn sellprice(&self) -> f32 {
        self.sellprice
    }

    // Setters
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    #[allow(dead_code)]
    pub fn set_buyprice(&mut self, buyprice: f32) {
        self.buyprice = approx(buyprice);
    }
    #[allow(dead_code)]
    pub fn set_taxpercentage(&mut self, taxpercentage: f32) {
        self.taxpercentage = approx(taxpercentage);
    }
    #[allow(dead_code)]
    pub fn set_gainpercentage(&mut self, gainpercentage: f32) {
        self.gainpercentage = approx(gainpercentage);
    }
    #[allow(dead_code)]
    pub fn set_sellprice(&mut self, sellprice: f32) {
        self.sellprice = approx(sellprice);
    }

    // Public methods
    #[allow(dead_code)]
    pub fn new() -> Product{
        Product{name: String::new(), buyprice: 0., taxpercentage: 0., gainpercentage: 0., sellprice: 0.}
    }
    pub fn from_gainpercentage(name: &str, buyprice: f32, taxpercentage: f32, gainpercentage: f32) -> Product{
        update_MAXLEN(&name);
        let mut p = Product{name: name.into(), buyprice: approx(buyprice), taxpercentage: approx(taxpercentage), gainpercentage: approx(gainpercentage), sellprice: 0.};
        p.set_sellprice(p.calc_sellprice());
        p
    }
    pub fn from_sellprice(name: &str, buyprice: f32, taxpercentage: f32, sellprice: f32) -> Product{
        update_MAXLEN(&name);
        let mut p = Product{name: name.into(), buyprice: approx(buyprice), taxpercentage: approx(taxpercentage), gainpercentage: 0., sellprice: sellprice};
        p.set_gainpercentage(p.calc_gainpercentage());
        p
    }

    // Private methods
    fn calc_gainpercentage(&self) -> f32{
        (((&self.sellprice/&self.buyprice)-1.-&self.taxpercentage/100.)/(1.+&self.taxpercentage/100.))*100.
    }
    fn calc_sellprice(&self) -> f32{
        &self.buyprice*(1.+ &self.taxpercentage/100.+&self.gainpercentage/100.*(1.+&self.taxpercentage/100.))
    }

}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "- {: <width$} {: >7.2}{} {: >5.1}{} {: >5.1}{} {: >7.2}{}\n", self.name, self.buyprice, "€", self.taxpercentage, "%", self.gainpercentage, "%", self.sellprice, "€", width = get_MAXLEN())
    }
}

fn approx(val: f32) -> f32{
    (val * 100.0).round() / 100.0
}

#[allow(non_snake_case)]
pub fn update_MAXLEN(name: &str){
    unsafe {
        if name.len() > MAXLEN{
            MAXLEN = name.len()
        }
    }
}

#[allow(non_snake_case)]
pub fn get_MAXLEN() -> usize{
    unsafe{
        MAXLEN
    }
}

