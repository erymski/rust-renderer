use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Args {
    #[arg(short, long, default_value = "outputs/render.png")]
    output: PathBuf,

    #[arg(short, long, default_value_t = 1024)]
    width: u32,

    #[arg(short, long, default_value_t = 768)]
    heigth: u32,

    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

fn ensure_filename(candidate: &PathBuf) -> PathBuf {
    if candidate.is_absolute() {
        return candidate.to_path_buf();
    } else {
        let cwd = std::env::current_dir().unwrap();
        let full_path = cwd.join(candidate);

        std::fs::create_dir_all(full_path.parent().unwrap())
            .expect("Cannot create dir for output image");

        full_path
    }
}

pub(crate) struct ImageParams {
    pub width_px: u32,
    pub height_px: u32,
}

pub(crate) struct ExecutionParams {
    pub output_file: PathBuf,
    pub rendering: ImageParams,
}

pub(crate) fn get_params() -> ExecutionParams {
    let args = Args::parse();

    ExecutionParams {
        output_file: ensure_filename(&args.output),
        rendering: ImageParams {
            width_px: args.width,
            height_px: args.heigth,
        },
    }
}
