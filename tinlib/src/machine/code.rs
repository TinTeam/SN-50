pub struct Code {
    #[allow(dead_code)]
    chars: [char; 1],
}

impl Default for Code {
    fn default() -> Self {
        Self { chars: [' '; 1] }
    }
}
