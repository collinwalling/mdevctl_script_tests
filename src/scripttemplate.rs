//! Call-out script template

use std::io::{self, Read};
use structopt::StructOpt;
use strum_macros::EnumString;

#[derive(EnumString, Debug)]
#[strum(ascii_case_insensitive)]
pub enum CalloutEvent {
    Pre,
    Post,
    Get,
    Notify,
}

#[derive(EnumString, Debug)]
#[strum(ascii_case_insensitive)]
pub enum CommandAction {
    Define,
    Modify,
    Start,
    Stop,
    Undefine,
    List,
    Attributes,
}

#[derive(EnumString, Debug)]
#[strum(ascii_case_insensitive)]
pub enum CommandState {
    None,
    Success,
    Failure,
}

#[derive(StructOpt, Debug)]
pub struct ScriptOpts {
    #[structopt(
        name = "type",
        short,
        long,
        required_ifs(&[("event", "pre"),("event", "post"),("event", "get")]),
    )]
    pub mdev_type: Option<String>,
    #[structopt(
        short,
        long,
        help = "pre, post, notify, get"
    )]
    event: CalloutEvent,
    #[structopt(
        short,
        long,
        help = "define, modify, start, stop, undefine, list, attributes (get only)"
    )]
    action: CommandAction,
    #[structopt(
        short,
        long,
        help = "success, failure, none"
    )]
    state: CommandState,
    #[structopt(
        short,
        long,
    )]
    pub uuid: String,
    #[structopt(
        short,
        long,
    )]
    pub parent: String,
    #[structopt(
        skip,
    )]
    pub json: String,
}

pub trait ScriptFunctions {
    fn check_type(&self) -> bool;
    fn check_parent(&self) -> bool;
    fn pre_start(&self) -> i32 { 0 }
    fn post_start(&self) -> i32 { 0 }
    fn pre_define(&self) -> i32 { 0 }
    fn post_define(&self) -> i32 { 0 }
    fn pre_modify(&self) -> i32 { 0 }
    fn post_modify(&self) -> i32 { 0 }
    fn pre_stop(&self) -> i32 { 0 }
    fn post_stop(&self) -> i32 { 0 }
    fn pre_undefine(&self) -> i32 { 0 }
    fn post_undefine(&self) -> i32 { 0 }
    fn pre_list(&self) -> i32 { 0 }
    fn post_list(&self) -> i32 { 0 }
    fn get_attributes(&self) -> i32 { 0 }
    fn notify(&self) -> i32 { 0 }
}

fn read_json_from_stdin() -> io::Result<String> {
    let mut json = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // TODO: figure out how to make this non-blocking in the
    // case where no stdin is present...
    handle.read_to_string(&mut json)?;
    Ok(json)
}

pub fn run_script() -> i32 {
    let mut opts = ScriptOpts::from_args();

    if !opts.check_parent() {
        return 2;
    }

    /*
     * Get does not expect data on stdin and
     * notify does not expect a device type.
     */
    match opts.event {
        CalloutEvent::Get => (),
        _ => {
            opts.json = read_json_from_stdin().unwrap();
            match opts.event {
                CalloutEvent::Notify => (),
                _ => {
                    if !opts.check_type() {
                        return 2;
                    }
                }
            }
        }
    }

    match opts.event {
        CalloutEvent::Get =>
            match opts.action {
                CommandAction::Attributes => opts.get_attributes(),
                _ => return 0,
            }
        CalloutEvent::Notify => opts.notify(),
        CalloutEvent::Pre =>
            match opts.action {
                CommandAction::Define => opts.pre_define(),
                CommandAction::Modify => opts.pre_modify(),
                CommandAction::Start => opts.pre_start(),
                CommandAction::Stop => opts.pre_stop(),
                CommandAction::Undefine => opts.pre_undefine(),
                CommandAction::List => opts.pre_list(),
                _ => return 0,
            }
        CalloutEvent::Post =>
            match opts.action {
                CommandAction::Define => opts.post_define(),
                CommandAction::Modify => opts.post_modify(),
                CommandAction::Start => opts.post_start(),
                CommandAction::Stop => opts.post_stop(),
                CommandAction::Undefine => opts.post_undefine(),
                CommandAction::List => opts.post_list(),
                _ => return 0,
            }
    }
}
