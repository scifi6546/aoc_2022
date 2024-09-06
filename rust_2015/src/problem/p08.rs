use super::Problem;
pub struct P08;
impl Problem for P08 {
    fn number() -> u32 {
        8
    }

    type AOutput = u32;
    type BOutput = u32;

    fn a(input: &str) -> Option<Self::AOutput> {
        enum EscapeState {
            /// No escape has been recieved
            NoEscape,
            /// the previous character recieved was a \
            FirstEscape,
            /// The previous 2 characters were "\x"
            FirstHex,
            SecondHex,
        }
        struct CurrentState {
            chars_received: u32,
            raw_chars_received: u32,
            in_quote: bool,
            escape_state: EscapeState,
        }
        impl CurrentState {
            fn new() -> Self {
                Self {
                    chars_received: 0,
                    raw_chars_received: 0,
                    in_quote: false,
                    escape_state: EscapeState::NoEscape,
                }
            }
            fn add_character(mut self, char: char) -> Self {
                self.raw_chars_received += 1;
                if self.in_quote {
                    match self.escape_state {
                        EscapeState::NoEscape => {
                            if char == '\\' {
                                self.escape_state = EscapeState::FirstEscape;
                                self
                            } else if char == '"' {
                                self.in_quote = false;
                                self
                            } else {
                                self.chars_received += 1;
                                self
                            }
                        }
                        EscapeState::FirstEscape => {
                            if char == '"' {
                                self.chars_received += 1;
                                self.escape_state = EscapeState::NoEscape;
                                self
                            } else if char == '\\'{
                                self.chars_received += 1;
                                self.escape_state = EscapeState::NoEscape;
                                self
                            }
                            else if char == 'x' {
                                self.escape_state = EscapeState::FirstHex;
                                self
                            } else {
                                todo!("handle {} after \\", char)
                            }
                        }
                        EscapeState::FirstHex => {
                            self.escape_state = EscapeState::SecondHex;
                            self
                        }
                        EscapeState::SecondHex => {
                            self.escape_state = EscapeState::NoEscape;
                            self.chars_received+=1;
                            self
                        }
                    }
                } else {
                    if char == '"' {
                        self.in_quote = true;
                        self
                    } else {
                        panic!("invalid char")
                    }
                }
            }
            fn get_delta(&self) -> u32 {
                self.raw_chars_received - self.chars_received
            }
        }
        Some(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .fold(CurrentState::new(), |state, c| state.add_character(c))
                        .get_delta()
                })
                .sum(),
        )
    }

    fn b(_input: &str) -> Option<Self::BOutput> {
        None
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a_total() {
        assert_eq!(
            P08::a("\"\"\n\"abc\"\"aaa\\\"aaa\"\n\"\\x27\"").unwrap(),
            12
        )
    }
    #[test]
    fn a_parts() {
        assert_eq!(P08::a("\"\"").unwrap(), 2);
        assert_eq!(P08::a("\"abc\"").unwrap(), 2);
        assert_eq!(P08::a("\"aaa\\\"aaa\"").unwrap(), 3);
        assert_eq!(P08::a("\"\\x27\"").unwrap(), 5);
    }
}
