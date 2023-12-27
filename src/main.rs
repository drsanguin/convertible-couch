use clap::Parser;
use convertible_couch::{
    display_settings::{DisplaySettings, Win32DevicesDisplayImpl, Win32GraphicsGdiImpl},
    log::{configure_logger, LogLevel},
};
use log::{error, info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    desktop_monitor_name: String,
    #[arg(short, long)]
    couch_monitor_name: String,
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

fn main() {
    let args: Args = Args::parse();

    configure_logger(args.log_level);

    let win32_devices_display = Win32DevicesDisplayImpl;
    let win32_graphics_gdi = Win32GraphicsGdiImpl;
    let display_settings = DisplaySettings::new(win32_devices_display, win32_graphics_gdi);

    unsafe {
        match display_settings
            .swap_primary_monitors(&args.desktop_monitor_name, &args.couch_monitor_name)
        {
            Ok(response) => {
                match response.new_primary {
                    Some(new_primary) => info!("Primary monitor set to {}", new_primary),
                    None => error!("Primary monitor has not been changed for an unknow reason"),
                }

                if response.reboot_required {
                    warn!("The settings change was successful but the computer must be restarted for the graphics mode to work.");
                }
            }
            Err(message) => error!("{}", message),
        }
    }
}
