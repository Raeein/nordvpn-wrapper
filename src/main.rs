use std::process::Command;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.on_check_if_pair_solved(move || {
        println!("yo i got clicked");
        let output = Command::new("ls")
            .arg("-a")
            .output()
            .expect("ls command failed");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    });
    main_window.run()
}
