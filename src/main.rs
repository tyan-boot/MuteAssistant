#![windows_subsystem = "windows"]

use std::rc::Rc;

use anyhow::Result;
use gio::prelude::*;
use gtk::Builder;
use gtk::prelude::*;

use crate::utils::MAIN_VIEW;
use crate::view::{View, ViewExt};
use crate::volume::Volume;

mod utils;
mod view;
mod volume;

fn main() -> Result<()> {
    let volume = Rc::new(Volume::new()?);

    let ui = gtk::Application::new(Some("pw.boot.mute"), gio::ApplicationFlags::FLAGS_NONE)?;
    ui.connect_startup(move |app| {
        let glade = include_str!("../mute.glade");
        let builder = Builder::new_from_string(glade);

        let view = View::new(&builder, volume.clone()).expect("view");
        view.init(app);
        unsafe { MAIN_VIEW = Some(view.clone()) };

        app.connect_activate(move |_app| {
            view.window.show_all();

            utils::init_tray(view.clone());
            utils::init(view.clone());

            let view = view.clone();
            gtk::timeout_add(200, move || {
                if view.push_view.is_enable() {
                    let codes = view.push_view.get_key_codes();

                    if utils::is_key_release(&codes) {
                        view.mute();
                    }
                }

                Continue(true)
            });
        });
    });

    ui.run(&std::env::args().collect::<Vec<_>>());
    Ok(())
}
