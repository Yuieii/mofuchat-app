use std::collections::HashMap;

use async_trait::async_trait;
use eyre::Result;
use tauri::{App, Manager, Runtime};
use tokio::{
    sync::{Mutex, MutexGuard, broadcast},
    task::JoinHandle,
};

use crate::{
    prefs::Prefs,
    services::{ServiceEvent, adapter::AdapterRegistry},
};

pub struct AppState {
    pub event_tx: broadcast::Sender<ServiceEvent>,
    pub active_tasks: Mutex<HashMap<String, JoinHandle<()>>>,
    pub prefs: Mutex<Prefs>,
    pub registry: AdapterRegistry,
}

pub fn setup(app: &mut App) -> Result<()> {
    let (event_tx, _) = broadcast::channel(512);

    let handle = app.handle();
    let prefs = Prefs::try_from_local(handle)
        .or_else(|_| Prefs::try_from_onecomme(handle))
        .unwrap_or_else(|err| {
            println!("Error while reading OneComme config: {:?}", err);
            Prefs::default()
        });

    let state = AppState {
        event_tx,
        prefs: Mutex::new(prefs),
        active_tasks: Mutex::new(HashMap::new()),
        registry: AdapterRegistry::new(),
    };

    app.manage(state);

    Ok(())
}

impl AppState {
    pub async fn lock_prefs(&self) -> MutexGuard<'_, Prefs> {
        self.prefs.lock().await
    }

    pub async fn lock_active_tasks(&self) -> MutexGuard<'_, HashMap<String, JoinHandle<()>>> {
        self.active_tasks.lock().await
    }
}

#[async_trait]
pub trait ManagerExt<R> {
    fn app_state(&self) -> &AppState;

    async fn lock_active_tasks(&self) -> MutexGuard<'_, HashMap<String, JoinHandle<()>>> {
        self.app_state().lock_active_tasks().await
    }

    async fn lock_prefs(&self) -> MutexGuard<'_, Prefs> {
        self.app_state().lock_prefs().await
    }
}

impl<T, R> ManagerExt<R> for T
where
    T: Manager<R>,
    R: Runtime,
{
    fn app_state(&self) -> &AppState {
        self.state::<AppState>().inner()
    }
}
