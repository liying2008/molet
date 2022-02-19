use clipboard_win::{formats, get_clipboard, SysResult};

pub struct DataOp {}

impl DataOp {
    pub fn clipboard_to_db() {
        let result: SysResult<String> = get_clipboard(formats::Unicode);
        let data = match result {
            Ok(data) => {
                println!("ok = {}", data);
                data
            }
            Err(err) => {
                println!("err = {}", err.message());
                String::new()
            }
        };
        if data == "" {
            println!("data is empty");
            return;
        }
        println!("data is {}", data)
    }
}
