use futures_util::{FutureExt as ExternalFutureExt};
use serde::Serialize;

pub struct CommandError(pub eyre::Report);

impl<T> From<T> for CommandError 
where T: std::error::Error + Send + Sync + 'static
{
    fn from(value: T) -> Self {
        CommandError(eyre::Report::from(value))
    }
}

pub type TauriResult<T> = std::result::Result<T, CommandError>;

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(&format!("{:#}", self.0))
    }
}

pub trait ResultExt<R>: sealed::PrivateResult {
    fn tauri(self) -> TauriResult<R>;
}

impl<R> sealed::PrivateResult for Result<R, eyre::Report> {}

impl<R> ResultExt<R> for Result<R, eyre::Report> {
    fn tauri(self) -> TauriResult<R> {
        self.map_err(|err| CommandError(err))
    }
}

pub trait FutureExt: Future + sealed::PrivateFuture {
    fn tauri<R>(self) -> impl Future<Output = TauriResult<R>>
    where 
        Self: Future<Output = Result<R, eyre::Report>> + Sized 
    {
        self.map(|f| f.map_err(|r| CommandError(r)))
    }
}

impl<T, R> FutureExt for T
where 
    T: Future<Output = Result<R, eyre::Report>> + Sized
{

}

impl<T> sealed::PrivateFuture for T
where T : Future {}

mod sealed {
    pub trait PrivateResult {}
    pub trait PrivateFuture {}
}