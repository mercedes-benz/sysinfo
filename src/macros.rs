// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "debug")]
#[doc(hidden)]
#[allow(unused)]
macro_rules! sysinfo_debug {
    ($($x:tt)*) => {{
        eprintln!($($x)*);
    }}
}

#[cfg(not(feature = "debug"))]
#[doc(hidden)]
#[allow(unused)]
macro_rules! sysinfo_debug {
    ($($x:tt)*) => {{}};
}

macro_rules! declare_signals {
    ($kind:ty, _ => None,) => (
        use crate::Signal;

        pub(crate) const fn supported_signals() -> &'static [Signal] {
            &[]
        }
    );

    ($kind:ty, $(Signal::$signal:ident => $map:expr,)+ _ => None,) => (
        use crate::Signal;

        pub(crate) const fn supported_signals() -> &'static [Signal] {
            &[$(Signal::$signal,)*]
        }

        #[inline]
        pub(crate) fn convert_signal(s: Signal) -> Option<$kind> {
            match s {
                $(Signal::$signal => Some($map),)*
                _ => None,
            }
        }
    );

    ($kind:ty, $(Signal::$signal:ident => $map:expr,)+) => (
        use crate::Signal;

        pub(crate) const fn supported_signals() -> &'static [Signal] {
            &[$(Signal::$signal,)*]
        }

        #[inline]
        pub(crate) fn convert_signal(s: Signal) -> Option<$kind> {
            match s {
                $(Signal::$signal => Some($map),)*
            }
        }
    )
}

#[cfg(all(unix, not(feature = "unknown-ci")))]
macro_rules! retry_eintr {
    ($($t:tt)+) => {
        loop {
            let ret = $($t)+;
            if ret < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            break ret;
        }
    }
}
