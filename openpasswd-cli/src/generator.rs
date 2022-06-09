use crate::clipboard::copy_password_to_clipboard;
use clap::Args;
use rand::Rng;

fn generate_string_vec_u8(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (&mut rng)
        .sample_iter(rand::distributions::Alphanumeric)
        .take(size)
        .collect()
}

pub fn generate_string(size: usize) -> String {
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

        if self.print {
            println!("{}", &password);
        }
        copy_password_to_clipboard(password, 5);
    }
}
