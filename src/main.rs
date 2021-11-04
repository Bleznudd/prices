
mod core;
mod organizer;
use crate::core::Core;
use argparse::{ArgumentParser, List, Store, StoreTrue};
use std::process;

#[derive(Debug)]
struct CliError{
    description: String
}
impl std::error::Error for CliError {}
impl CliError {
    fn new(msg: &str) -> CliError {
        CliError{description: "[CliError] ".to_string() + msg}
    }
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}
impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.description())
    }
}

fn cli(ver: bool, add: bool, rem: bool, fin: bool, undo: bool, sup: &String, prod: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    if ver {
        println!("Prices v.{}", VERSION);
        println!("Built with \u{2764} by {}", AUTHORS);
        process::exit(0);
    }

    let mut o = Core::new()?;
    o.load()?;
    if undo {
        o.restore()?;
        process::exit(0);
    } 

    if add{
        if !sup.is_empty() && prod.is_empty(){
            o.add_supplier_from_name(&sup[..]);
        }
        else if !sup.is_empty() && !prod.is_empty(){
            match prod.len(){
                1 => o.add_product_from_name(&sup[..], &prod[0]),
                2 => o.add_product_from_gainpercentage(&sup[..], &prod[0], prod[1].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, 0., 0.),
                3 => o.add_product_from_gainpercentage(&sup[..], &prod[0], prod[1].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, prod[2].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, 0.),
                4 => {
                    if !fin{
                        o.add_product_from_gainpercentage(&sup[..], &prod[0], prod[1].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, prod[2].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, prod[3].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?)
                    }
                    else{
                        o.add_product_from_sellprice(&sup[..], &prod[0], prod[1].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, prod[2].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?, prod[3].parse::<f32>().map_err(|e| CliError::new(&e.to_string()[..]))?)
                    }
                }
                _ => return Err(Box::new(CliError::new("Too many arguments")))
            }
        }
        o.backup()?;
        o.save()?;
    }
    else if rem{
        if !sup.is_empty() && prod.is_empty(){
            o.remove_supplier_from_name(&sup[..])
        }
        else if !sup.is_empty() && !prod.is_empty(){
            o.remove_product_from_name(&sup[..], &prod[0])
        }
        else{
            return Err(Box::new(CliError::new("Always provide a supplier and a product when removing")))
        }
        o.backup()?;
        o.save()?;
    }
    else{
        if !sup.is_empty(){
            o.show_supplier(&sup[..]);
        }
        else if !prod.is_empty(){
            o.show_product(&prod[0]);
        }
        else{
            o.show_all();
        }
    }
    Ok(())
}

fn main(){
    
    let mut ver: bool = false;
    // let mut gui: bool = false;
    let mut add: bool = false;
    let mut rem: bool = false;
    let mut fin: bool = false;
    let mut undo: bool = false;
    let mut sup: String = String::new();
    let mut prod: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Manage prices of your products");
        ap.refer(&mut ver)
            .add_option(&["-v", "--version"], StoreTrue, "Print version and exit");
        // ap.refer(&mut gui)
        //     .add_option(&["-g", "--gui"], StoreTrue, "Launches the graphic interface");
        ap.refer(&mut add)
            .add_option(&["-a", "--add"], StoreTrue, "Add the supplier or product that follows");
        ap.refer(&mut rem)
            .add_option(&["-r", "--remove"], StoreTrue, "Remove the supplier or product that follows");
        ap.refer(&mut sup)
            .add_option(&["-s", "--supplier"], Store, "Name of the supplier");
        ap.refer(&mut prod)
            .add_option(&["-p", "--product"], List, "Name (and optionally the values) of the product");
        ap.refer(&mut fin)
            .add_option(&["-f", "--final"], StoreTrue, "Use the final price instead of the gain percentage");
        ap.refer(&mut undo)
            .add_option(&["-u", "--undo"], StoreTrue, "Undo the last operation (restore from backup)");
        ap.parse_args_or_exit();
    }

    match cli(ver, add, rem, fin, undo, &sup, &prod){
        Err(why) => {
            eprint!("{}\n", why);
            process::exit(1);
        },
        _ => process::exit(0)
    }

}
