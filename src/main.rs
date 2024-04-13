use slint::format;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increment_counter ( move || {
            let ui = ui_handle.unwrap();
            let counter = ui.get_counter() + 1;
            let result = {
                if counter % 5 == 0 && counter % 3 == 0 {
                    format!("FizzBuzz")
                } else if  counter % 5 == 0 {
                    format!("Buzz")
                } else if counter % 3 == 0 {
                    format!("Fizz")
                } else {
                    format!("")
                }
            };
            ui.set_result(result.into());
            ui.set_counter(counter);
        }
    );

    let ui_handle2 = ui.as_weak();
    ui.on_request_reset_counter ( move || {
            let ui = ui_handle2.unwrap();
            let result = format!("");
            ui.set_result(result.into());
            ui.set_counter(0);
        }
    );

    ui.run()
}
