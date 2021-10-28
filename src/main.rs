
mod core;
mod organizer;
use crate::core::Core;
use argparse::{ArgumentParser, List, Store, StoreTrue};
use std::num::ParseFloatError;
use std::process;

#[derive(Debug)]
struct CliError{
    description: String
}

impl CliError {
    fn new(msg: &str) -> CliError {
        CliError{description: msg.to_string()}
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
impl From<ParseFloatError> for CliError {
    fn from(err: ParseFloatError) -> Self {
        CliError::new(&err.to_string()[..])
    }
}

fn main() -> Result<(), CliError> {

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    
    let mut version: bool = false;
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
        ap.refer(&mut version)
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

    if version {
        println!("Prices v.{}", VERSION);
        println!("Built with \u{2764} by {}", AUTHORS);
        process::exit(0x0100);
    }

    let mut o: Core = Core::new();
    o.load();
    if undo {
        o.restore();
        process::exit(0x0100);
    } 

    if add{
        if !sup.is_empty() && prod.is_empty(){
            o.add_supplier_from_name(&sup[..]);
        }
        else if !sup.is_empty() && !prod.is_empty(){
            match prod.len(){
                1 => o.add_product_from_name(&sup[..], &prod[0]),
                2 => o.add_product_from_gainpercentage(&sup[..], &prod[0], prod[1].parse::<f32>()?, 0., 0.),
                3 => o.add_product_from_gainpercentage(&sup[..], &prod[0], prod[1].parse::<f32>()?, prod[2].parse::<f32>()?, 0.),
                4 => {
                    if !fin{
                        o.add_product_from_gainpercentage(&sup[..], &prod[0], prod[1].parse::<f32>()?, prod[2].parse::<f32>()?, prod[3].parse::<f32>()?)
                    }
                    else{
                        o.add_product_from_sellprice(&sup[..], &prod[0], prod[1].parse::<f32>()?, prod[2].parse::<f32>()?, prod[3].parse::<f32>()?)
                    }
                }
                _ => println!("Too many arguments")
            }
        }
        o.backup();
        o.save();
    }
    else if rem{
        if !sup.is_empty() && prod.is_empty(){
            o.remove_supplier_from_name(&sup[..])
        }
        else if !sup.is_empty() && !prod.is_empty(){
            o.remove_product_from_name(&sup[..], &prod[0])
        }
        else{
            println!("Always provide a supplier when removing a product")
        }
        o.backup();
        o.save();
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
