use penrose::{
    contrib::{actions::update_monitors_via_xrandr, extensions::dmenu::*},
    core::{bindings::KeyEventHandler, data_types::RelativePosition},
};

use crate::{Conn, Wm};

pub fn power_menu() -> KeyEventHandler<Conn> {
    Box::new(move |wm: &mut Wm| {
        let options = vec!["lock", "logout", "restart-wm", "shutdown", "reboot"];
        let menu = DMenu::new(">>> ", options, DMenuConfig::default());
        let screen_index = wm.active_screen_index();

        if let Ok(MenuMatch::Line(_, choice)) = menu.run(screen_index) {
            match choice.as_ref() {
                "lock" => spawn!("dm-tool switch-to-greeter"),
                "logout" => spawn!("pkill -fi run-penrose"),
                "shutdown" => spawn!("sudo shutdown -h now"),
                "reboot" => spawn!("sudo reboot"),
                "restart-wm" => wm.exit(),
                _ => unimplemented!(),
            }
        } else {
            Ok(())
        }
    })
}
