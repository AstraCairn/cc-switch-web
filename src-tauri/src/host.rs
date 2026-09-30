//! Desktop keeps Tauri's `AppHandle` and async runtime.
//! The web build uses a small stand-in so services can emit events and
//! block on futures without linking WebKitGTK.

#[cfg(feature = "desktop")]
pub use tauri::async_runtime;
#[cfg(feature = "desktop")]
pub use tauri::State;

#[cfg(feature = "desktop")]
pub type AppHandle = tauri::AppHandle;

#[cfg(not(feature = "desktop"))]
mod web_host {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::ops::Deref;
    use std::sync::{Arc, Mutex, OnceLock};

    use serde::Serialize;
    use tokio::sync::broadcast;

    #[derive(Clone, Debug)]
    pub struct WebEvent {
        pub name: String,
        pub payload: serde_json::Value,
    }

    struct HandleInner {
        states: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
        events: broadcast::Sender<WebEvent>,
    }

    #[derive(Clone)]
    pub struct AppHandle {
        inner: Arc<HandleInner>,
    }

    pub struct State<'a, T> {
        inner: Arc<T>,
        _marker: PhantomData<&'a ()>,
    }

    impl<T> Clone for State<'_, T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
                _marker: PhantomData,
            }
        }
    }

    impl<T> Deref for State<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.inner
        }
    }

    impl<T> State<'_, T> {
        pub fn from_arc(inner: Arc<T>) -> State<'static, T> {
            State {
                inner,
                _marker: PhantomData,
            }
        }

        pub fn inner(&self) -> &T {
            &self.inner
        }
    }

    impl AppHandle {
        pub fn new() -> Self {
            let (events, _) = broadcast::channel(256);
            Self {
                inner: Arc::new(HandleInner {
                    states: Mutex::new(HashMap::new()),
                    events,
                }),
            }
        }

        pub fn manage<T: Send + Sync + 'static>(&self, state: T) {
            let mut guard = self
                .inner
                .states
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            guard.insert(TypeId::of::<T>(), Arc::new(state));
        }

        pub fn try_state<T: Send + Sync + 'static>(&self) -> Option<State<'static, T>> {
            let guard = self
                .inner
                .states
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            guard.get(&TypeId::of::<T>()).and_then(|value| {
                value
                    .clone()
                    .downcast::<T>()
                    .ok()
                    .map(State::from_arc)
            })
        }

        pub fn state<T: Send + Sync + 'static>(&self) -> State<'static, T> {
            self.try_state::<T>()
                .unwrap_or_else(|| panic!("state {} is not managed", std::any::type_name::<T>()))
        }

        pub fn emit<S: Serialize>(&self, event: &str, payload: S) -> Result<(), String> {
            let payload = serde_json::to_value(&payload).map_err(|error| error.to_string())?;
            let _ = self.inner.events.send(WebEvent {
                name: event.to_string(),
                payload,
            });
            Ok(())
        }

        pub fn subscribe(&self) -> broadcast::Receiver<WebEvent> {
            self.inner.events.subscribe()
        }
    }

    static RUNTIME: OnceLock<tokio::runtime::Handle> = OnceLock::new();

    pub fn install_runtime(handle: tokio::runtime::Handle) {
        let _ = RUNTIME.set(handle);
    }

    fn runtime_handle() -> tokio::runtime::Handle {
        tokio::runtime::Handle::try_current()
            .unwrap_or_else(|_| RUNTIME.get().expect("web runtime is not installed").clone())
    }

    pub mod async_runtime {
        use super::runtime_handle;
        use std::future::Future;

        pub struct Handle(tokio::runtime::Handle);

        impl Handle {
            pub fn block_on<F: Future>(&self, future: F) -> F::Output {
                self.0.block_on(future)
            }

            pub fn spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
            where
                F: Future + Send + 'static,
                F::Output: Send + 'static,
            {
                self.0.spawn(future)
            }
        }

        pub fn handle() -> Handle {
            Handle(runtime_handle())
        }

        pub fn block_on<F: Future>(future: F) -> F::Output {
            if tokio::runtime::Handle::try_current().is_ok() {
                tokio::task::block_in_place(|| runtime_handle().block_on(future))
            } else {
                runtime_handle().block_on(future)
            }
        }

        pub fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
        {
            runtime_handle().spawn(future)
        }

        pub fn spawn_blocking<F, R>(function: F) -> tokio::task::JoinHandle<R>
        where
            F: FnOnce() -> R + Send + 'static,
            R: Send + 'static,
        {
            tokio::task::spawn_blocking(function)
        }
    }
}

#[cfg(not(feature = "desktop"))]
pub use web_host::{async_runtime, install_runtime, AppHandle, State, WebEvent};
