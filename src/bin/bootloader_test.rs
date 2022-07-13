#![forbid(unsafe_code)]
use clap::{arg, command, Arg, App};

use bootloader_test::set_successful_boot;
use tracing::Level;
fn main() {
    let args = std::env::args();
    let matches = App::new("Bootloader test application")
    .version("1.0")
    .author("Anagha")
    .about("Tester application to set successboot for the active slot")
        .arg(
            Arg::with_name("set-successful-boot")
                .short('s')
                .long("--set-successful-boot")
                .help("Sets successful boot for current active slot"),
        )
        .get_matches_from(args);
        let set_boot= matches.is_present("set-successful-boot");
    println!("set_successful_boot{}",set_boot);
   
    if set_boot
    {
        set_successful_boot();
    }
}
