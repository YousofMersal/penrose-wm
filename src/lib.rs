#[macro_use]
extern crate penrose;

use penrose::{WindowManager, XcbConnection};

pub mod actions;

pub type Conn = XcbConnection;
pub type Wm = WindowManager<Conn>;
