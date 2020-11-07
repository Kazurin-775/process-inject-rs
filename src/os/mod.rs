use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        pub mod windows;
    } else {
        compile_error!("unsupported OS");
    }
}
