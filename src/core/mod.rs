
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use crate::organizer::{manager::Manager, supplier::Supplier, product::Product};

#[derive(Debug)]
pub struct Core{
    manager: Manager,
    savepath: String,
    savefile: String,
    backupfile: String
}

impl Core{
    
    // Getter
    pub fn manager(&self) -> &Manager {
        &self.manager
    }

    // Public methods
    pub fn new() -> Core{
        let mut sp: String = String::new();
        match home::home_dir() {
            Some(path) => (sp = path.into_os_string().into_string().unwrap()+"/.local/share/prices/".into()),
            None => println!("Impossible to get your home dir! No data can be stored"),
        }
        Core{manager: Manager::new(), savepath: sp, savefile: "prices.json".into(), backupfile: "prices.bkp".into()}
    }
    pub fn load(&mut self){
        let dirpath: &Path = Path::new(&self.savepath[..]);
        let filepath: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        let mut file: File = match fs::create_dir_all(dirpath){
            Err(why) => panic!("couldn't create missing directory {}: {}", dirpath.display(), why),
            Ok(()) => match File::open(&filepath){
                Err(_why) => {
                    match File::create(&filepath){
                        Err(why) => panic!("couldn't create file {}: {}", filepath.display(), why),
                        Ok(mut file) => {
                            match file.write_all("{\"name\":\"Default\",\"suppliers\":[]}".as_bytes()){
                                Err(why) => panic!("couldn't write to {}: {}", filepath.display(), why),
                                Ok(()) => () 
                            }
                            match File::open(&filepath){
                                Err(why) => panic!("couldn't open {}: {}", filepath.display(), why),
                                Ok(file) => file
                            }
                        }
                    }
                },
                Ok(file) => file
            }
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", filepath.display(), why),
            Ok(_) => self.manager = serde_json::from_str(&s).unwrap()
        }
    }
    pub fn save(&self){
        let filepath: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        let mut file: File = match File::create(&filepath){
            Err(why) => panic!("couldn't open {}: {}", filepath.display(), why),
            Ok(file) => file,
        };
        match file.write_all(serde_json::to_string_pretty(self.manager()).unwrap().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", filepath.display(), why),
            Ok(_) => ()
        }
    }
    pub fn backup(&self){
        let from: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        let to: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.backupfile[..]));
        match File::open(&to){
            Err(_why) => {
                match File::create(&to){
                    Err(why) => panic!("couldn't create file {}: {}", to.display(), why),
                    Ok(_) => ()
                }
            }
            Ok(_) => ()
        }
        match fs::copy(from, to){
            Err(why) => println!("coulnd't backup: {}", why),
            Ok(_) => ()
        }
    }
    pub fn restore(&self){
        let from: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.backupfile[..]));
        let to: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        match File::open(&from){
            Err(_why) => {
                match File::create(&to){
                    Err(why) => panic!("couldn't create file {}: {}", to.display(), why),
                    Ok(_) => ()
                }
            }
            Ok(_) => ()
        }
        match fs::copy(from, to){
            Err(why) => println!("coulnd't restore: {}", why),
            Ok(_) => ()
        }
    }

    // Add methods
    pub fn add_supplier_from_name(&mut self, name: &str){
        self.check_supplier_before_adding(name, Supplier::from_name)
    }
    pub fn add_product_from_name(&mut self, sup: &str, name: &str){
        self.check_product_before_adding(sup, name, 0., 0., 0., Product::from_gainpercentage)
    }
    pub fn add_product_from_gainpercentage(&mut self, sup: &str, name: &str, buyprice: f32, taxpercentage: f32, gainpercentage: f32){
        self.check_product_before_adding(sup, name, buyprice, taxpercentage, gainpercentage, Product::from_gainpercentage)
    }
    pub fn add_product_from_sellprice(&mut self, sup: &str, name: &str, buyprice: f32, taxpercentage: f32, sellprice: f32){
        self.check_product_before_adding(sup, name, buyprice, taxpercentage, sellprice, Product::from_sellprice)
    }

    // Show methods
    pub fn show_all(&self){
        print!("{}", &self.manager());
    }
    pub fn show_supplier(&self, name: &str){
        for i in self.manager().suppliers().iter(){
            if name == i.name(){
                print!("{}", i.to_string())
            }
        }
    }
    pub fn show_product(&self, name: &str){
        for i in self.manager().suppliers().iter(){
            for j in i.products().iter(){
                if name == j.name(){
                    print!("+ {}\n", i.name());
                    print!("{}", j.to_string());
                }
            }
        }
    }

    // Remove methods
    pub fn remove_supplier_from_name(&mut self, name: &str){
        self.check_supplier_before_removing(name)
    }
    pub fn remove_product_from_name(&mut self, sup: &str, name: &str){
        self.check_product_before_removing(sup, name)
    }

    // Private methods
    fn check_product_before_adding(&mut self, sup: &str, name: &str, buyprice: f32, taxpercentage: f32, gainpercentage_or_sellprice: f32, f: fn(&str, f32, f32, f32) -> Product){
        if self.manager.exists_supplier(sup){
            if let Some(pos) = self.manager.suppliers().iter().position(|x| x.name() == sup) {
                if !self.manager.suppliers[pos].exists_product(name){
                    self.manager.suppliers[pos].add_product(f(name, buyprice, taxpercentage, gainpercentage_or_sellprice));
                }
                else{
                    println!("product with this name already exist")
                }
            }
        }
        else{
            println!("supplier with this name doesn't exist")
        }
    }
    fn check_product_before_removing(&mut self, sup: &str, name: &str){
        if self.manager.exists_supplier(sup){
            if let Some(pos) = self.manager.suppliers().iter().position(|x| x.name() == sup) {
                if self.manager.suppliers[pos].exists_product(name){
                    self.manager.suppliers[pos].remove_product_from_name(name);
                }
                else{
                    println!("product with this name doesn't exist")
                }
            }
        }
        else{
            println!("supplier with this name doesn't exist")
        }
    }
    fn check_supplier_before_adding(&mut self, name: &str, f: fn(&str) -> Supplier){
        if !self.manager.exists_supplier(name){
            self.manager.add_supplier(f(name))
        }
        else{
            println!("supplier with this name already exist")
        }
    }
    fn check_supplier_before_removing(&mut self, name: &str){
        if self.manager.exists_supplier(name){
            self.manager.remove_supplier_from_name(name)
        }
        else{
            println!("supplier with this name doesn't exist")
        }
    }



}

