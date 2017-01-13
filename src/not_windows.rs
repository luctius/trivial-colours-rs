impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}m", 30 + (*self as usize))
    }
}

impl fmt::Display for BgColour {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}m", 40 + (*self as usize))
    }
}

impl fmt::Display for Reset {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[00m")
    }
}
