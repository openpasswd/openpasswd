use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_password_to_clipboard(password: String, timeout: u64) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password).unwrap();

    println!("Password ready do be pasted for {timeout} timeout");

    std::thread::sleep(std::time::Duration::from_secs(timeout));
    ctx.set_contents("".to_owned()).unwrap();
    println!("Clipboard unset");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
