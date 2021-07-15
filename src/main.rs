//! A basic utility script used for debugging notification call-out events
//! in mdevctl. Output can be observed via system logs.
//!
//! Place this script in /etc/mdevctl.d/scripts.d/notifiers/ and run
//! a supported command, then check the system logs for output.

use crate::scripttemplate::*;

mod scripttemplate;

impl ScriptFunctions for ScriptOpts {
    fn check_type(&self) -> bool {
        // likely overkill, but an example how to have the script support
        // multiple types
        let list = vec![String::from("vfio_ap-passthrough")];
        list.contains(&self.mdev_type.as_ref().unwrap())
    }

    fn check_parent(&self) -> bool {
        // Does it make sense for an mdev type to have multiple parents?
        let list = vec![String::from("matrix")];
        list.contains(&self.parent)
    }

    fn notify(&self) -> i32 {
        println!("logger_script: {:?}", self);
        return 0
    }
}

fn main() {
    run_script();
}
