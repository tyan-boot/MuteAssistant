use gtk::prelude::*;
use gtk::{
    Application, Builder, Button, ButtonsType, CheckButton, ComboBoxText, DialogFlags, Label,
    MessageDialog, MessageType, Switch, Window,
};

use lazy_static::lazy_static;

use anyhow::{Context, Result};

use crate::utils::{register_hotkey, unregister_hotkey};
use crate::volume::Volume;
use glib::WeakRef;
use std::collections::BTreeMap;
use std::rc::Rc;

lazy_static! {
    static ref KEY_MAP: BTreeMap<u32, &'static str> = {
        let mut map = BTreeMap::new();

        map.insert(0x01, "Left mouse button");
        map.insert(0x02, "Right mouse button");
        map.insert(0x03, "Control-break processing");
        map.insert(0x04, "Middle mouse button (three-button mouse)");
        map.insert(0x05, "X1 mouse button");
        map.insert(0x06, "X2 mouse button");
        map.insert(0x08, "BACKSPACE key");
        map.insert(0x09, "TAB key");
        map.insert(0x0C, "CLEAR key");
        map.insert(0x0D, "ENTER key");
        map.insert(0x10, "SHIFT key");
        map.insert(0x11, "CTRL key");
        map.insert(0x12, "ALT key");
        map.insert(0x13, "PAUSE key");
        map.insert(0x14, "CAPS LOCK key");
        map.insert(0x15, "IME Kana mode");
        map.insert(0x15,"IME Hanguel mode");
        map.insert(0x15, "IME Hangul mode");
        map.insert(0x16, "IME On");
        map.insert(0x17, "IME Junja mode");
        map.insert(0x18, "IME final mode");
        map.insert(0x19, "IME Hanja mode");
        map.insert(0x19, "IME Kanji mode");
        map.insert(0x1A, "IME Off");
        map.insert(0x1B, "ESC key");
        map.insert(0x1C, "IME convert");
        map.insert(0x1D, "IME nonconvert");
        map.insert(0x1E, "IME accept");
        map.insert(0x1F, "IME mode change request");
        map.insert(0x20, "SPACEBAR");
        map.insert(0x21, "PAGE UP key");
        map.insert(0x22, "PAGE DOWN key");
        map.insert(0x23, "END key");
        map.insert(0x24, "HOME key");
        map.insert(0x25, "LEFT ARROW key");
        map.insert(0x26, "UP ARROW key");
        map.insert(0x27, "RIGHT ARROW key");
        map.insert(0x28, "DOWN ARROW key");
        map.insert(0x29, "SELECT key");
        map.insert(0x2A, "PRINT key");
        map.insert(0x2B, "EXECUTE key");
        map.insert(0x2C, "PRINT SCREEN key");
        map.insert(0x2D, "INS key");
        map.insert(0x2E, "DEL key");
        map.insert(0x2F, "HELP key");
        map.insert(0x30, "0 key");
        map.insert(0x31, "1 key");
        map.insert(0x32, "2 key");
        map.insert(0x33, "3 key");
        map.insert(0x34, "4 key");
        map.insert(0x35, "5 key");
        map.insert(0x36, "6 key");
        map.insert(0x37, "7 key");
        map.insert(0x38, "8 key");
        map.insert(0x39, "9 key");
        map.insert(0x41, "A key");
        map.insert(0x42, "B key");
        map.insert(0x43, "C key");
        map.insert(0x44, "D key");
        map.insert(0x45, "E key");
        map.insert(0x46, "F key");
        map.insert(0x47, "G key");
        map.insert(0x48, "H key");
        map.insert(0x49, "I key");
        map.insert(0x4A, "J key");
        map.insert(0x4B, "K key");
        map.insert(0x4C, "L key");
        map.insert(0x4D, "M key");
        map.insert(0x4E, "N key");
        map.insert(0x4F, "O key");
        map.insert(0x50, "P key");
        map.insert(0x51, "Q key");
        map.insert(0x52, "R key");
        map.insert(0x53, "S key");
        map.insert(0x54, "T key");
        map.insert(0x55, "U key");
        map.insert(0x56, "V key");
        map.insert(0x57, "W key");
        map.insert(0x58, "X key");
        map.insert(0x59, "Y key");
        map.insert(0x5A, "Z key");
        map.insert(0x5B, "Left Windows key (Natural keyboard)");
        map.insert(0x5C, "Right Windows key (Natural keyboard)");
        map.insert(0x5D, "Applications key (Natural keyboard)");
        map.insert(0x5F, "Computer Sleep key");
        map.insert(0x60, "Numeric keypad 0 key");
        map.insert(0x61, "Numeric keypad 1 key");
        map.insert(0x62, "Numeric keypad 2 key");
        map.insert(0x63, "Numeric keypad 3 key");
        map.insert(0x64, "Numeric keypad 4 key");
        map.insert(0x65, "Numeric keypad 5 key");
        map.insert(0x66, "Numeric keypad 6 key");
        map.insert(0x67, "Numeric keypad 7 key");
        map.insert(0x68, "Numeric keypad 8 key");
        map.insert(0x69, "Numeric keypad 9 key");
        map.insert(0x6A, "Multiply key");
        map.insert(0x6B, "Add key");
        map.insert(0x6C, "Separator key");
        map.insert(0x6D, "Subtract key");
        map.insert(0x6E, "Decimal key");
        map.insert(0x6F, "Divide key");
        map.insert(0x70, "F1 key");
        map.insert(0x71, "F2 key");
        map.insert(0x72, "F3 key");
        map.insert(0x73, "F4 key");
        map.insert(0x74, "F5 key");
        map.insert(0x75, "F6 key");
        map.insert(0x76, "F7 key");
        map.insert(0x77, "F8 key");
        map.insert(0x78, "F9 key");
        map.insert(0x79, "F10 key");
        map.insert(0x7A, "F11 key");
        map.insert(0x7B, "F12 key");
        map.insert(0x7C, "F13 key");
        map.insert(0x7D, "F14 key");
        map.insert(0x7E, "F15 key");
        map.insert(0x7F, "F16 key");
        map.insert(0x80, "F17 key");
        map.insert(0x81, "F18 key");
        map.insert(0x82, "F19 key");
        map.insert(0x83, "F20 key");
        map.insert(0x84, "F21 key");
        map.insert(0x85, "F22 key");
        map.insert(0x86, "F23 key");
        map.insert(0x87, "F24 key");
        map.insert(0x90, "NUM LOCK key");
        map.insert(0x91, "SCROLL LOCK key");
        map.insert(0xA0, "Left SHIFT key");
        map.insert(0xA1, "Right SHIFT key");
        map.insert(0xA2, "Left CONTROL key");
        map.insert(0xA3, "Right CONTROL key");
        map.insert(0xA4, "Left MENU key");
        map.insert(0xA5, "Right MENU key");
        map.insert(0xA6, "Browser Back key");
        map.insert(0xA7, "Browser Forward key");
        map.insert(0xA8, "Browser Refresh key");
        map.insert(0xA9, "Browser Stop key");
        map.insert(0xAA, "Browser Search key");
        map.insert(0xAB, "Browser Favorites key");
        map.insert(0xAC, "Browser Start and Home key");
        map.insert(0xAD, "Volume Mute key");
        map.insert(0xAE, "Volume Down key");
        map.insert(0xAF, "Volume Up key");
        map.insert(0xB0, "Next Track key");
        map.insert(0xB1, "Previous Track key");
        map.insert(0xB2, "Stop Media key");
        map.insert(0xB3, "Play/Pause Media key");
        map.insert(0xB4, "Start Mail key");
        map.insert(0xB5, "Select Media key");
        map.insert(0xB6, "Start Application 1 key");
        map.insert(0xB7, "Start Application 2 key");

        map
    };
}

pub struct ModKeyView {
    ctrl: CheckButton,
    alt: CheckButton,
    shift: CheckButton,
    win: CheckButton,
}

pub trait MuteViewExt {
    fn init(&self, app: &Application);

    fn update_hotkey(&self) -> bool;
}

pub struct MuteView {
    mute_btn: Switch,

    mute_mod: ModKeyView,
    mute_key: ComboBoxText,

    mute_key_set: Button,

    window: WeakRef<Window>,
    volume: Rc<Volume>,
}

impl MuteView {
    pub fn new(
        builder: &Builder,
        window: WeakRef<Window>,
        volume: Rc<Volume>,
    ) -> Result<Rc<MuteView>> {
        let mute_btn = builder.get_object("mute_btn").context("mute btn")?;

        let ctrl = builder.get_object("mute_ctrl").context("ctrl")?;
        let alt = builder.get_object("mute_alt").context("alt")?;
        let shift = builder.get_object("mute_shift").context("shift")?;
        let win = builder.get_object("mute_win").context("win")?;

        let mute_key: ComboBoxText = builder.get_object("mute_key").context("key")?;
        let mute_key_set = builder.get_object("hotkey_set").unwrap();

        for (code, desc) in KEY_MAP.iter() {
            mute_key.append(Some(&code.to_string()), desc);
        }
        mute_key.set_active(Some(0));

        let view = MuteView {
            mute_btn,
            mute_mod: ModKeyView {
                ctrl,
                alt,
                shift,
                win,
            },
            mute_key,
            mute_key_set,
            window,
            volume,
        };

        Ok(Rc::new(view))
    }
}

impl MuteViewExt for Rc<MuteView> {
    fn init(&self, _app: &Application) {
        self.mute_btn.connect_changed_active(|_this| {});

        let this = self.clone();
        self.mute_key_set.connect_clicked(move |_it| {
            if this.update_hotkey() {
                this.mute_btn.set_active(true);
                let dialog = MessageDialog::new(
                    this.window.upgrade().as_ref(),
                    DialogFlags::MODAL,
                    MessageType::Info,
                    ButtonsType::Ok,
                    "设置成功",
                );
                dialog.connect_response(|dialog, _| {
                    dialog.emit_close();
                });
                dialog.run();
            } else {
                this.mute_btn.set_active(false);
                let dialog = MessageDialog::new(
                    this.window.upgrade().as_ref(),
                    DialogFlags::MODAL,
                    MessageType::Error,
                    ButtonsType::Ok,
                    "设置失败, 快捷键已被占用",
                );
                dialog.connect_response(|dialog, _| {
                    dialog.emit_close();
                });
                dialog.run();
            }
        });

        let this = self.clone();
        self.mute_btn.connect_state_set(move |_it, enable| {
            if enable {
                this.update_hotkey();
            } else {
                unregister_hotkey(1);
            }

            Inhibit(false)
        });
    }

    fn update_hotkey(&self) -> bool {
        let key_code = self
            .mute_key
            .get_active_id()
            .and_then(|it| it.as_str().parse::<u32>().ok())
            .unwrap();

        let mod_code = (self.mute_mod.ctrl.get_active() as u32) << 1
            | (self.mute_mod.alt.get_active() as u32) << 0
            | (self.mute_mod.shift.get_active() as u32) << 2
            | (self.mute_mod.win.get_active() as u32) << 3
            | 0x4000;

        unregister_hotkey(1);
        register_hotkey(mod_code, key_code, 1)
    }
}

pub struct PushView {
    push_btn: Switch,

    push_mod: ModKeyView,
    push_key: ComboBoxText,

    push_key_set: Button,

    window: WeakRef<Window>,
}

pub trait PushViewExt {
    fn init(&self);

    fn update_hotkey(&self) -> bool;
}

impl PushViewExt for Rc<PushView> {
    fn init(&self) {
        let this = self.clone();
        self.push_key_set.connect_clicked(move |_it| {
            if this.update_hotkey() {
                this.push_btn.set_active(true);
                let dialog = MessageDialog::new(
                    this.window.upgrade().as_ref(),
                    DialogFlags::MODAL,
                    MessageType::Info,
                    ButtonsType::Ok,
                    "设置成功",
                );
                dialog.connect_response(|dialog, _| {
                    dialog.emit_close();
                });
                dialog.run();
            } else {
                this.push_btn.set_active(false);
                let dialog = MessageDialog::new(
                    this.window.upgrade().as_ref(),
                    DialogFlags::MODAL,
                    MessageType::Error,
                    ButtonsType::Ok,
                    "设置失败, 快捷键已被占用",
                );
                dialog.connect_response(|dialog, _| {
                    dialog.emit_close();
                });
                dialog.run();
            }
        });

        let this = self.clone();
        self.push_btn.connect_state_set(move |_it, enable| {
            if enable {
                this.update_hotkey();
            } else {
                unregister_hotkey(2);
            }
            Inhibit(false)
        });
    }

    fn update_hotkey(&self) -> bool {
        let key_code = self
            .push_key
            .get_active_id()
            .and_then(|it| it.as_str().parse::<u32>().ok())
            .unwrap();

        let mod_code = (self.push_mod.ctrl.get_active() as u32) << 1
            | (self.push_mod.alt.get_active() as u32) << 0
            | (self.push_mod.shift.get_active() as u32) << 2
            | (self.push_mod.win.get_active() as u32) << 3
            | 0x4000;

        unregister_hotkey(2);
        register_hotkey(mod_code, key_code, 2)
    }
}

impl PushView {
    pub fn new(builder: &Builder, window: WeakRef<Window>) -> Result<Rc<PushView>> {
        let push_btn = builder.get_object("push_btn").context("mute btn")?;

        let ctrl = builder.get_object("push_ctrl").context("ctrl")?;
        let alt = builder.get_object("push_alt").context("alt")?;
        let shift = builder.get_object("push_shift").context("shift")?;
        let win = builder.get_object("push_win").context("win")?;

        let push_key: ComboBoxText = builder.get_object("push_key").context("key")?;
        let push_key_set = builder.get_object("push_hotkey_set").unwrap();

        for (code, desc) in KEY_MAP.iter() {
            push_key.append(Some(&code.to_string()), desc);
        }
        push_key.set_active(Some(0));

        let view = PushView {
            push_btn,
            push_mod: ModKeyView {
                ctrl,
                alt,
                shift,
                win,
            },
            push_key,
            push_key_set,
            window,
        };

        Ok(Rc::new(view))
    }

    pub fn get_key_codes(&self) -> Vec<u32> {
        let mut codes = Vec::new();

        if self.push_mod.ctrl.get_active() {
            codes.push(0x11);
        }

        if self.push_mod.alt.get_active() {
            codes.push(0x12);
        }

        if self.push_mod.shift.get_active() {
            codes.push(0x10);
        }

        if self.push_mod.win.get_active() {
            codes.push(0x5b);
            codes.push(0x5c);
        }

        codes.push(
            self.push_key
                .get_active_id()
                .and_then(|it| it.as_str().parse::<u32>().ok())
                .unwrap(),
        );

        codes
    }

    pub fn is_enable(&self) -> bool {
        self.push_btn.get_active()
    }
}

pub struct View {
    pub window: Window,
    pub mute_view: Rc<MuteView>,
    pub push_view: Rc<PushView>,

    pub switch: Switch,
    pub status: Label,

    volume: Rc<Volume>,
}

pub trait ViewExt {
    fn init(&self, app: &Application);

    fn hide(&self);

    fn show(&self);

    fn mute(&self);

    fn unmute(&self);

    fn toggle(&self);
}

impl ViewExt for Rc<View> {
    fn init(&self, app: &Application) {
        self.window.set_application(Some(app));
        self.window.set_title("Mute Assistant");

        self.mute_view.init(app);
        self.push_view.init();

        let this = self.clone();
        self.switch.connect_state_set(move |_it, state| {
            if state {
                this.mute();
            } else {
                this.unmute();
            }

            Inhibit(false)
        });

        self.window.connect_delete_event(|it, _event| {
            it.hide_on_delete();

            Inhibit(true)
        });
    }

    fn hide(&self) {
        self.window.hide();
    }

    fn show(&self) {
        self.window.show();
    }

    fn mute(&self) {
        self.volume.mute();
        self.status.set_text("静音");
        self.switch.set_active(true);
    }

    fn unmute(&self) {
        self.volume.unmute();
        self.status.set_text("正常");
        self.switch.set_active(false);
    }

    fn toggle(&self) {
        self.volume.toggle();
        let is_mute = self.volume.is_mute();
        self.switch.set_active(is_mute);
    }
}

impl View {
    pub fn new(builder: &Builder, volume: Rc<Volume>) -> Result<Rc<View>> {
        let window: Window = builder.get_object("settings").context("main window")?;
        let switch: Switch = builder.get_object("switch").unwrap();
        let status = builder.get_object("status").unwrap();

        let mute_view = MuteView::new(builder, window.downgrade(), volume.clone())?;
        let push_view = PushView::new(builder, window.downgrade())?;

        let view = View {
            window,
            mute_view,
            push_view,
            switch,
            status,
            volume,
        };
        Ok(Rc::new(view))
    }
}
