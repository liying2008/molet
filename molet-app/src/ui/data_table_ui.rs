extern crate native_windows_gui as nwg;

use crate::ui::data_table::DataTable;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct DataTableUi {
    inner: Rc<DataTable>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl nwg::NativeUi<DataTableUi> for DataTable {
    fn build_ui(mut data: DataTable) -> Result<DataTableUi, nwg::NwgError> {
        use nwg::Event as E;

        // Controls
        nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((300, 135))
            .position((300, 300))
            .title("Basic example")
            .build(&mut data.window)?;

        nwg::TextInput::builder()
            .size((280, 35))
            .position((10, 10))
            .text("Heisenberg")
            .parent(&data.window)
            .focus(true)
            .build(&mut data.name_edit)?;

        nwg::Button::builder()
            .size((280, 70))
            .position((10, 50))
            .text("Say my name")
            .parent(&data.window)
            .build(&mut data.hello_button)?;

        // Wrap-up
        let ui = DataTableUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    E::OnButtonClick => {
                        if &handle == &ui.hello_button {
                            DataTable::say_hello(&ui);
                        }
                    }
                    E::OnWindowClose => {
                        if &handle == &ui.window {
                            DataTable::say_goodbye(&ui);
                        }
                    }
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.window.handle,
            handle_events,
        ));

        Ok(ui)
    }
}

impl Drop for DataTableUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for DataTableUi {
    type Target = DataTable;

    fn deref(&self) -> &DataTable {
        &self.inner
    }
}
