//! Event handlers.

use crate::OutputType;
use htmlescape::encode_attribute;
use std::fmt::{Display, Error, Formatter};
use std::iter;

/// Trait for event handlers.
pub trait EventHandler<T: OutputType, E> {
    /// Build a callback function from this event handler.
    ///
    /// Returns `None` is this event handler can't be used to build a callback
    /// function. This is usually the case if the event handler is a string
    /// intended for server side rendering.
    // fn build(self) -> Option<Box<FnMut(EventType) + 'static>>;

    fn attach(&mut self, target: &mut T::EventTarget) -> T::EventListenerHandle;

    /// Render this event handler as a string.
    ///
    /// Returns `None` if this event handler cannot be rendered. Normally, the
    /// only event handlers that can be rendered are string values intended for
    /// server side rendering.
    fn render(&self) -> Option<String>;
}

macro_rules! declare_events_struct {
    ($($name:ident,)*) => {
        pub struct Events<T> {
            $(
                pub $name: Option<T>,
            )*
        }

        impl<T> Events<T> {
            pub fn iter(&self) -> impl Iterator<Item = (&'static str, &T)> {
                iter::empty()
                $(
                    .chain(
                        self.$name.iter()
                        .map(|value| (stringify!($name), value))
                    )
                )*
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = (&'static str, &mut T)> {
                iter::empty()
                $(
                    .chain(
                        self.$name.iter_mut()
                        .map(|value| (stringify!($name), value))
                    )
                )*
            }
        }

        impl<T> Default for Events<T> {
            fn default() -> Self {
                Events {
                    $(
                        $name: None,
                    )*
                }
            }
        }

        impl<T: Display> Display for Events<T> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(
                    if let Some(ref value) = self.$name {
                        let attribute = encode_attribute(&value.to_string());
                        write!(f, " on{}=\"{}\"", stringify!($name), attribute)?;
                    }
                )*
                Ok(())
            }
        }
    }
}

declare_events_struct! {
    abort,
    autocomplete,
    autocompleteerror,
    blur,
    cancel,
    canplay,
    canplaythrough,
    change,
    click,
    close,
    contextmenu,
    cuechange,
    dblclick,
    drag,
    dragend,
    dragenter,
    dragexit,
    dragleave,
    dragover,
    dragstart,
    drop,
    durationchange,
    emptied,
    ended,
    error,
    focus,
    input,
    invalid,
    keydown,
    keypress,
    keyup,
    load,
    loadeddata,
    loadedmetadata,
    loadstart,
    mousedown,
    mouseenter,
    mouseleave,
    mousemove,
    mouseout,
    mouseover,
    mouseup,
    mousewheel,
    pause,
    play,
    playing,
    progress,
    ratechange,
    reset,
    resize,
    scroll,
    seeked,
    seeking,
    select,
    show,
    sort,
    stalled,
    submit,
    suspend,
    timeupdate,
    toggle,
    volumechange,
    waiting,
}
