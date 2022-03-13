extern crate native_windows_gui as nwg;

use crate::ui::system_tray::SystemTray;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct SystemTrayUi {
    inner: Rc<SystemTray>,
    default_handler: RefCell<Vec<nwg::EventHandler>>,
}

impl nwg::NativeUi<SystemTrayUi> for SystemTray {
    fn build_ui(mut data: SystemTray) -> Result<SystemTrayUi, nwg::NwgError> {
        use nwg::Event as E;

        // Resources
        nwg::Icon::builder()
            .source_file(Some("./res/system-tray-icon-64.ico"))
            .build(&mut data.icon)?;

        // Controls
        nwg::MessageWindow::builder().build(&mut data.window)?;

        nwg::TrayNotification::builder()
            .parent(&data.window)
            .icon(Some(&data.icon))
            .tip(Some("Molet - Clipboard Data Staging Tool"))
            .build(&mut data.tray)?;

        nwg::Menu::builder()
            .popup(true)
            .parent(&data.window)
            .build(&mut data.tray_menu)?;

        nwg::MenuItem::builder()
            .text("Store Clipboard Data")
            .parent(&data.tray_menu)
            .build(&mut data.tray_item_data_push)?;

        nwg::MenuItem::builder()
            .text("Show Clipboard Data")
            .parent(&data.tray_menu)
            .build(&mut data.tray_item_data_show)?;

        nwg::MenuItem::builder()
            .text("Exit")
            .parent(&data.tray_menu)
            .build(&mut data.tray_item_exit)?;

        // Wrap-up
        let ui = SystemTrayUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_ui) = evt_ui.upgrade() {
                match evt {
                    E::OnMousePress(e) => {
                        if &handle == &evt_ui.tray {
                            println!("OnMousePress: {:?}", e);
                            SystemTray::show_menu(&evt_ui);
                        }
                    }
                    E::OnContextMenu => {
                        if &handle == &evt_ui.tray {
                            SystemTray::show_menu(&evt_ui);
                        }
                    }
                    E::OnMenuItemSelected => {
                        if &handle == &evt_ui.tray_item_data_push {
                            SystemTray::store_clipboard_data(&evt_ui);
                        } else if &handle == &evt_ui.tray_item_data_show {
                            SystemTray::show_clipboard_data(&evt_ui);
                        } else if &handle == &evt_ui.tray_item_exit {
                            SystemTray::exit(&evt_ui);
                        }
                    }
                    _ => {}
                }
            }
        };

        ui.default_handler
            .borrow_mut()
            .push(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

        Ok(ui)
    }
}

impl Drop for SystemTrayUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let mut handlers = self.default_handler.borrow_mut();
        for handler in handlers.drain(0..) {
            nwg::unbind_event_handler(&handler);
        }
    }
}

impl Deref for SystemTrayUi {
    type Target = SystemTray;

    fn deref(&self) -> &SystemTray {
        &self.inner
    }
}
