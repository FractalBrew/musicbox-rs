mod events;
mod hardware;
mod hw_config;
mod musicbox;
mod player;
mod playlist;
mod track;

use std::fmt::Display;

use log::error;

pub use musicbox::MusicBox;

pub type MusicResult<T> = Result<T, String>;
pub type VoidResult = MusicResult<()>;

trait ResultErrorLogger<E>
where
    E: Display,
{
    fn log_error<F>(self, f: F) -> Self
    where
        F: FnOnce(&E) -> String;
}

impl<O, E> ResultErrorLogger<E> for Result<O, E>
where
    E: Display,
{
    fn log_error<F>(self, f: F) -> Result<O, E>
    where
        F: FnOnce(&E) -> String,
    {
        if let Err(ref e) = self {
            error!("{}", f(e));
        }

        self
    }
}
