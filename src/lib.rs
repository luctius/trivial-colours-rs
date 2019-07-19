/// Very simple and naive colour changing for your terminal, but that's exactly what you need sometimes.
///
/// All colour-related things are used by `Display`ing them.
///
/// # Examples
///
/// ```
/// # use trivial_colours::{BgColour, Reset};
/// print!("{}", Reset);
///
/// println!("{}BgColour::Black", BgColour::Black);
/// println!("{}BgColour::Red", BgColour::Red);
/// println!("{}BgColour::Green", BgColour::Green);
/// println!("{}BgColour::Yellow", BgColour::Yellow);
/// println!("{}BgColour::Blue", BgColour::Blue);
/// println!("{}BgColour::Magenta", BgColour::Magenta);
/// println!("{}BgColour::Cyan", BgColour::Cyan);
/// println!("{}BgColour::White", BgColour::White);
///
/// println!("{}This text has default colours", Reset);
/// ```

#[cfg(feature = "no_std")]
#[no_std]

#[cfg(all(target_os = "windows", feature = "use_winapi"))]
#[macro_use]
extern crate lazy_static;
#[cfg(all(target_os = "windows", feature = "use_winapi"))]
extern crate kernel32;
#[cfg(all(target_os = "windows", feature = "use_winapi"))]
extern crate winapi;

#[cfg(all(feature = "use_winapi", feature = "no_std"))]
compile_error!("features `use_winapi` and `no_std` are mutually exclusive");

#[cfg(all(target_os = "windows", feature = "use_winapi"))]
mod windows_winapi;
#[cfg(all(target_os = "windows", not(feature = "use_winapi")))]
mod dummy;
#[cfg(not(target_os = "windows"))]
mod not_windows;

#[cfg(all(target_os = "windows", feature = "use_winapi"))]
pub use self::windows_winapi::*;
#[cfg(all(target_os = "windows", not(feature = "use_winapi")))]
pub use self::dummy::*;
#[cfg(not(target_os = "windows"))]
pub use self::not_windows::*;

/// The supported foreground colours.
///
/// Use them with `Display` to engage setting colour.
///
/// You can use this with `{:.0}` which will *not* change the colour.
///
/// Note: take *extreme* care, as each and every call to `Display::fmt()` on this enum might change the *terminal*'s foreground
/// colour on some platforms.
///
/// # Examples
///
/// ```
/// # use trivial_colours::{Colour, Reset};
/// # print!("{}", Reset);
/// println!("{}C{}M{}Y{}K", Colour::Cyan, Colour::Magenta, Colour::Yellow, Colour::Black);
/// # print!("{}", Reset);
/// ```
#[repr(usize)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Colour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// The supported background colours.
///
/// Use them with `Display` to engage setting colour.
///
/// You can use this with `{:.0}` which will *not* change the colour.
///
/// Note: take *extreme* care, as each and every call to `Display::fmt()` on this enum might change the *terminal*'s background
/// colour on some platforms.
///
/// # Examples
///
/// ```
/// # use trivial_colours::{BgColour, Reset};
/// # print!("{}", Reset);
/// println!("{}C{}M{}Y{}K", BgColour::Cyan, BgColour::Magenta, BgColour::Yellow, BgColour::Black);
/// # print!("{}", Reset);
/// ```
#[repr(usize)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum BgColour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// The attribute resetter.
///
/// Use this with `Display` to reset the attributes.
///
/// You can use this with `{:.0}` which will *not* reset the colour.
///
/// Note: you *need* to `Display::fmt()` on this at least once before using it to reset changes.
///
/// Note: take *extreme* care, as each and every call to `Display::fmt()` on this enum might reset the *terminal*'s colours on
/// some platforms.
///
/// # Examples
///
/// ```
/// # use trivial_colours::{Colour, Reset};
/// // Set defaults
/// print!("{}", Reset);
///
/// println!("{}Colourful", Colour::Magenta);
/// println!("{}Not Colourful", Reset);
/// ```
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Reset;
