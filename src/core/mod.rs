
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use home::home_dir;

use crate::organizer::{manager::Manager, supplier::Supplier, product::Product};


#[derive(Debug)]
pub struct CoreError{
    description: String
}
impl std::error::Error for CoreError {}
impl CoreError {
    fn new(msg: &str) -> CoreError {
        CoreError{description: "[CoreError] ".to_string() + msg}
    }
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}
impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.description())
    }
}

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
    pub fn new() -> Result<Core, CoreError>{
        let homedir: Result<PathBuf, CoreError> = home_dir().ok_or(CoreError::new("Impossible to get your home dir! No data can be stored"));
        let sp: String = homedir?.into_os_string().into_string().unwrap()+"/.local/share/prices/".into();
        Ok(Core{manager: Manager::new(), savepath: sp, savefile: "prices.json".into(), backupfile: "prices.bkp".into()})
    }
    pub fn load(&mut self) -> Result<(), CoreError>{
        let dirpath: &Path = Path::new(&self.savepath[..]);
        let filepath: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        if let Ok(_) = fs::create_dir(dirpath){
            return Err(CoreError::new("Can't create parent data directory"))
        }
        let file: Result<File, CoreError> = match File::open(&filepath){
            Err(_) => {
                let wf: Result<File, CoreError> = File::create(&filepath).map_err(|_| CoreError::new("Can't open nor create main file"));
                wf?.write_all("{\"name\":\"Default\",\"suppliers\":[]}".as_bytes()).ok();
                match File::open(&filepath){
                    Err(_) => return Err(CoreError::new("Can't open main file")),
                    Ok(f) => Ok(f)
                }
            }
            Ok(f) => Ok(f)
        };
        let mut s = String::new();
        if let Err(_) = file?.read_to_string(&mut s){
            return Err(CoreError::new("Main file is corrupted, try restoring from previous backup with --undo"));
        };
        match serde_json::from_str(&s){
            Err(_) => return Err(CoreError::new("Main file is corrupted, try restoring from previous backup with --undo")),
            Ok(j) => self.manager = j
        };
        Ok(())
    }
    pub fn save(&self) -> Result<(), CoreError>{
        let filepath: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        let file = File::create(&filepath).map_err(|e| CoreError::new(&e.to_string()[..]));
        match file?.write_all(serde_json::to_string_pretty(self.manager()).unwrap().as_bytes()) {
            Err(why) => Err(CoreError::new(&why.to_string()[..])),
            Ok(_) => Ok(())
        }
    }
    pub fn backup(&self) -> Result<(), CoreError>{
        let from: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        let to: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.backupfile[..]));
        if let Err(_) = File::open(&to){
            if let Err(_) = File::create(&to){
                return Err(CoreError::new("Can't open nor create backup file"))
            }
        }
        match fs::copy(from, to){
            Err(why) => Err(CoreError::new(&why.to_string()[..])),
            Ok(_) => Ok(())
        }
    }
    pub fn restore(&self) -> Result<(), CoreError>{
        let from: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.backupfile[..]));
        let to: PathBuf = Path::new(&self.savepath[..]).join(Path::new(&self.savefile[..]));
        if let Err(_) = File::open(&from){
            if let Err(_) = File::create(&to){
                return Err(CoreError::new("Can't open nor create save file"))
            }
        }
        match fs::copy(from, to){
            Err(why) => Err(CoreError::new(&why.to_string()[..])),
            Ok(_) => Ok(())
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

