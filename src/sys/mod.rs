use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else {
        compile_error!("unsupported OS");
    }
}
