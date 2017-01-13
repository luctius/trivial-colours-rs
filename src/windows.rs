use winapi::{WORD, COORD, SMALL_RECT, STD_OUTPUT_HANDLE, FOREGROUND_RED, FOREGROUND_BLUE, FOREGROUND_GREEN, BACKGROUND_RED, BACKGROUND_BLUE, BACKGROUND_GREEN,
             CONSOLE_SCREEN_BUFFER_INFO};
use kernel32::{GetStdHandle, SetConsoleTextAttribute, GetConsoleScreenBufferInfo};
use self::super::{BgColour, Colour, Reset};
use std::fmt;


lazy_static! {
    static ref DEFAULT_ATTRIBUTES: WORD = current_csbi().wAttributes;
}

fn current_csbi() -> CONSOLE_SCREEN_BUFFER_INFO {
    let mut info = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };

    unsafe {
        GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut info);
    }

    info
}

fn set_clr(slf: usize, r: u32, g: u32, b: u32, mask: WORD) {
    let mut attrs = current_csbi().wAttributes;

    let cid = match slf {
        0 => 0,
        1 => r,
        2 => g,
        3 => g | r,
        4 => b,
        5 => b | r,
        6 => b | g,
        7 => b | g | r,
        _ => 0,
    };

    attrs = attrs & !(attrs & mask);
    attrs = attrs | (cid as u16);
    unsafe {
        SetConsoleTextAttribute(GetStdHandle(STD_OUTPUT_HANDLE), attrs);
    }
}


impl fmt::Display for Colour {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        set_clr(*self as usize, FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, 0x0F);
        Ok(())
    }
}

impl fmt::Display for BgColour {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        set_clr(*self as usize, BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, 0xF0);
        Ok(())
    }
}

impl fmt::Display for Reset {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            SetConsoleTextAttribute(GetStdHandle(STD_OUTPUT_HANDLE), *DEFAULT_ATTRIBUTES);
        }
        Ok(())
    }
}
