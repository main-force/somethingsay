use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self, Read};


#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Hello, I'm Mainforce!!")]
    /// What do you want to say?
    message: String,
    
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    ascii_image: Option<std::path::PathBuf>,
    
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), ExitFailure>{
    
    let options = Options::from_args();
    let mut message = String::new();
    
    // If option 'stdin' is true, read message from stdin.
    // Else, read the default message.
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    };
    
    
    // Print message ballon.
    let print_msg_with_ballon = |message: String| {        
        println!("          //");
        println!("         //");
        println!("        //");
        println!("       //");
        println!("      //");
        println!("+===== ===============================================================");
            
        println!("{}", message
            .green()
            .bold()
            );
        
        println!("=====================================================================+");
        };
        
        
    // If user input the ascii image, use that.
    // Else, use default.
    match &options.ascii_image {
        Some (path) => {
            let image = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            println!("{}", &image);
            print_msg_with_ballon(message);
        },
        None => {
            print!(
"                                        __gg__
                                   _g@@@@@@@@@@@g_
                                 #@@@@@@@@@@@@@@@@@g
                                @@@@@@@@@@@@@@@@@@@@@_
                               #@@@@@@@@@@@@@@@@@@@@@@F
                               @@@@@@@#@@@#@@@@@@@@@@@@
                               @@$##@@@##@$##@@@#@@@@@@
                               @@####$$##$g$###@@#@@@@@
                             #@#####$$##@@@g########@@
                             #############@@########@_
                             \"#@$####@@@##$@@######@##
                               `?##########@######@@$'
                                 #@#####@@#######F\"`
                                 ####@@@@@@@@@##F
                                 ######@@@@####$
                                z############$F(
                          _g@@###############DL$\\g@@@g_
                    ,a\\@@@#@@@@@########@####DJ@@@@@@@@@@_
                _w@@@@@@@@@#@@@@@@@@@@#@@@##@g@@@@@@@@@@@@\\.
             g@@@@@@@@@@@@@@@#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\\e_
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#@@@@@@@@@@@@@@@@@@@@@g
           #@@g@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@$
          @@@@@#g@@@@@@@@@@@@@@@@@g@@@#@@@@@@@@#@@@@@@@@@@@@@@@@##@g
         g##@@@@@#gggggggg@@@#@@#####@@@@@@@@@@@@@@@@@@@@@@@@@g@@@#@L
         @@@@@@@@@@@@@@@###@@@@@@@@@@@@@@@@@@@@@@@@@gg#@#@@@@@@@@g@@@
");
            print_msg_with_ballon(message);
        }
    }
    
    Ok(())
}
