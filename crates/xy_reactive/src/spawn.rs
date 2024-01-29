use cfg_if::cfg_if;
use std::future::Future;

pub fn spawn_local<F>(fut: F)
where
    F: Future<Output = ()> + 'static,
{
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(fut)
        } else if #[cfg(feature = "glib")] {
            let main_context = glib::MainContext::default();
            main_context.spawn_local(fut);
        } else if #[cfg(any(test, doctest, feature = "tokio"))] {
            tokio::task::spawn_local(fut);
        }else if #[cfg(feature = "bevy")] {
            bevy_tasks::AsyncComputeTaskPool::get().spawn_local(fut).detach();
        }  else {
            futures::executor::block_on(fut)
        }
    }
}

pub fn spawn<F>(fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(fut)
        } else if #[cfg(feature = "glib")] {
            let main_context = glib::MainContext::default();
            main_context.spawn(fut);
        } else if #[cfg(feature = "bevy")] {
            bevy_tasks::AsyncComputeTaskPool::get().spawn(fut).detach();
        } else if #[cfg(any(test, doctest, feature = "tokio"))] {
            tokio::task::spawn(fut);
        }  else {
            futures::executor::block_on(fut)
        }
    }
}
