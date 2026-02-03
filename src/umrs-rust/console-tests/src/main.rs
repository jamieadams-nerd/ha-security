use umrs_core::console::{self, VERBOSE};
use std::sync::atomic::Ordering;
use umrs_core::console::ConsoleEvent;
use umrs_core::prelude::*;

fn main() {
    console::init(); // optional but recommended
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--verbose" | "-v" => {
                VERBOSE.store(true, Ordering::Relaxed);
            }
            _ => {
                // handle other args
            }
        }
    }

    console_event!(ConsoleEvent::BeginTask {
        name: "Test the console_*() macros",
    });
    console_info!("TEST OF THE console_info()!");
    console_warn!("TEST OF THE console_warn()!");
    console_error!("Some Error message using console_error!()");
    console_event!(ConsoleEvent::EndTask {
        name: "Finished testing console_*() macros",
    });

    println!("\n");
    verbose!("Console's verbose!() macro test");



    console_event!(ConsoleEvent::BeginTask {
        name: "STATUS: Test the console_status() macros",
    });
    console_status!(true, "Update file");
    console_status!(false, "Update file");


    println!("Hello, world!");
}
