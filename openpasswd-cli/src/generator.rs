use clap::Args;
use copypasta::{ClipboardContext, ClipboardProvider};
use rand::Rng;

fn generate_string_vec_u8(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (&mut rng)
        .sample_iter(rand::distributions::Alphanumeric)
        .take(size)
        .collect()
}

fn generate_string(size: usize) -> String {
    String::from_utf8(generate_string_vec_u8(size)).unwrap()
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Generator {
    #[clap(short, long, default_value_t = 16)]
    size: usize,

    #[clap(short, long)]
    print: bool,
}

impl Generator {
    pub fn execute(&self) {
        let password = generate_string(self.size);

        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(password.clone()).unwrap();

        let seconds = 5;
        println!("Password ready do be pasted for {seconds} seconds");

        if self.print {
            println!("{password}");
        }

        std::thread::sleep(std::time::Duration::from_secs(seconds));
        ctx.set_contents("".to_owned()).unwrap();
        println!("Clipboard unset");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
