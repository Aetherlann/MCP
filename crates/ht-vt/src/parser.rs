use crate::color::Color;
use crate::grid::CellAttributes;
use crate::graphics::{GraphicsCommand, KittyGraphics, ItermImage};

#[derive(Debug, Clone, PartialEq)]
pub enum VtToken {
    /// Printable character
    Print(char),
    /// Carriage return
    CarriageReturn,
    /// Line feed
    LineFeed,
    /// Backspace
    Backspace,
    /// Tab
    Tab,
    /// Bell
    Bell,
    /// Cursor movement (x, y)
    CursorMove(usize, usize),
    /// Cursor relative movement (dx, dy)
    CursorRelative(isize, isize),
    /// Set graphics attributes
    SetGraphics(CellAttributes),
    /// Clear screen
    ClearScreen,
    /// Clear line
    ClearLine,
    /// Graphics command (Kitty/iTerm2/Sixel)
    Graphics(GraphicsCommand),
    /// OSC 8 hyperlink
    Hyperlink { url: String, id: Option<String> },
    /// OSC 52 clipboard
    Clipboard(String),
    /// Unknown/unsupported sequence
    Unknown,
}

pub struct VtParser {
    state: ParserState,
    params: Vec<u32>,
    intermediate: Vec<u8>,
    osc_buffer: Vec<u8>,
    current_attrs: CellAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserState {
    Ground,
    Escape,
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    OscString,
    DcsEntry,
}

impl VtParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            params: Vec::new(),
            intermediate: Vec::new(),
            osc_buffer: Vec::new(),
            current_attrs: CellAttributes::default(),
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Vec<VtToken> {
        let mut tokens = Vec::new();

        for &byte in data {
            if let Some(token) = self.parse_byte(byte) {
                tokens.push(token);
            }
        }

        tokens
    }

    fn parse_byte(&mut self, byte: u8) -> Option<VtToken> {
        match self.state {
            ParserState::Ground => self.parse_ground(byte),
            ParserState::Escape => self.parse_escape(byte),
            ParserState::CsiEntry => self.parse_csi_entry(byte),
            ParserState::CsiParam => self.parse_csi_param(byte),
            ParserState::CsiIntermediate => self.parse_csi_intermediate(byte),
            ParserState::OscString => self.parse_osc_string(byte),
            ParserState::DcsEntry => self.parse_dcs_entry(byte),
        }
    }

    fn parse_ground(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            0x07 => Some(VtToken::Bell),
            0x08 => Some(VtToken::Backspace),
            0x09 => Some(VtToken::Tab),
            0x0A | 0x0B | 0x0C => Some(VtToken::LineFeed),
            0x0D => Some(VtToken::CarriageReturn),
            0x1B => {
                self.state = ParserState::Escape;
                None
            }
            0x20..=0x7E => Some(VtToken::Print(byte as char)),
            0x80..=0xFF => {
                // UTF-8 handling would go here
                Some(VtToken::Print(byte as char))
            }
            _ => None,
        }
    }

    fn parse_escape(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            b'[' => {
                self.state = ParserState::CsiEntry;
                self.params.clear();
                self.intermediate.clear();
                None
            }
            b']' => {
                self.state = ParserState::OscString;
                self.osc_buffer.clear();
                None
            }
            b'P' => {
                self.state = ParserState::DcsEntry;
                self.osc_buffer.clear();
                None
            }
            _ => {
                self.state = ParserState::Ground;
                Some(VtToken::Unknown)
            }
        }
    }

    fn parse_csi_entry(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            b'0'..=b'9' => {
                self.state = ParserState::CsiParam;
                self.params.push((byte - b'0') as u32);
                None
            }
            b';' => {
                self.state = ParserState::CsiParam;
                self.params.push(0);
                None
            }
            0x40..=0x7E => self.execute_csi(byte),
            _ => {
                self.state = ParserState::Ground;
                Some(VtToken::Unknown)
            }
        }
    }

    fn parse_csi_param(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            b'0'..=b'9' => {
                if let Some(last) = self.params.last_mut() {
                    *last = *last * 10 + (byte - b'0') as u32;
                }
                None
            }
            b';' => {
                self.params.push(0);
                None
            }
            0x40..=0x7E => self.execute_csi(byte),
            _ => {
                self.state = ParserState::Ground;
                Some(VtToken::Unknown)
            }
        }
    }

    fn parse_csi_intermediate(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            0x40..=0x7E => self.execute_csi(byte),
            _ => {
                self.state = ParserState::Ground;
                Some(VtToken::Unknown)
            }
        }
    }

    fn execute_csi(&mut self, command: u8) -> Option<VtToken> {
        self.state = ParserState::Ground;

        let token = match command {
            b'H' | b'f' => {
                // Cursor position
                let y = self.params.get(0).copied().unwrap_or(1).saturating_sub(1) as usize;
                let x = self.params.get(1).copied().unwrap_or(1).saturating_sub(1) as usize;
                VtToken::CursorMove(x, y)
            }
            b'A' => {
                // Cursor up
                let n = self.params.get(0).copied().unwrap_or(1) as isize;
                VtToken::CursorRelative(0, -n)
            }
            b'B' => {
                // Cursor down
                let n = self.params.get(0).copied().unwrap_or(1) as isize;
                VtToken::CursorRelative(0, n)
            }
            b'C' => {
                // Cursor forward
                let n = self.params.get(0).copied().unwrap_or(1) as isize;
                VtToken::CursorRelative(n, 0)
            }
            b'D' => {
                // Cursor backward
                let n = self.params.get(0).copied().unwrap_or(1) as isize;
                VtToken::CursorRelative(-n, 0)
            }
            b'm' => {
                // SGR - Select Graphic Rendition
                self.parse_sgr();
                VtToken::SetGraphics(self.current_attrs)
            }
            b'J' => {
                // Clear screen
                VtToken::ClearScreen
            }
            b'K' => {
                // Clear line
                VtToken::ClearLine
            }
            _ => VtToken::Unknown,
        };

        Some(token)
    }

    fn parse_sgr(&mut self) {
        let mut i = 0;
        while i < self.params.len() {
            match self.params[i] {
                0 => self.current_attrs = CellAttributes::default(),
                1 => self.current_attrs.bold = true,
                3 => self.current_attrs.italic = true,
                4 => self.current_attrs.underline = true,
                7 => self.current_attrs.inverse = true,
                9 => self.current_attrs.strikethrough = true,
                22 => self.current_attrs.bold = false,
                23 => self.current_attrs.italic = false,
                24 => self.current_attrs.underline = false,
                27 => self.current_attrs.inverse = false,
                29 => self.current_attrs.strikethrough = false,
                30..=37 => self.current_attrs.foreground = Color::Indexed((self.params[i] - 30) as u8),
                38 => {
                    // Extended foreground color
                    if i + 1 < self.params.len() {
                        match self.params[i + 1] {
                            5 => {
                                // 256 color
                                if i + 2 < self.params.len() {
                                    self.current_attrs.foreground = Color::Indexed(self.params[i + 2] as u8);
                                    i += 2;
                                }
                            }
                            2 => {
                                // RGB
                                if i + 4 < self.params.len() {
                                    self.current_attrs.foreground = Color::Rgb(
                                        self.params[i + 2] as u8,
                                        self.params[i + 3] as u8,
                                        self.params[i + 4] as u8,
                                    );
                                    i += 4;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                40..=47 => self.current_attrs.background = Color::Indexed((self.params[i] - 40) as u8),
                48 => {
                    // Extended background color
                    if i + 1 < self.params.len() {
                        match self.params[i + 1] {
                            5 => {
                                if i + 2 < self.params.len() {
                                    self.current_attrs.background = Color::Indexed(self.params[i + 2] as u8);
                                    i += 2;
                                }
                            }
                            2 => {
                                if i + 4 < self.params.len() {
                                    self.current_attrs.background = Color::Rgb(
                                        self.params[i + 2] as u8,
                                        self.params[i + 3] as u8,
                                        self.params[i + 4] as u8,
                                    );
                                    i += 4;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                90..=97 => self.current_attrs.foreground = Color::Indexed((self.params[i] - 90 + 8) as u8),
                100..=107 => self.current_attrs.background = Color::Indexed((self.params[i] - 100 + 8) as u8),
                _ => {}
            }
            i += 1;
        }
    }

    fn parse_osc_string(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            0x07 | 0x9C => {
                // BEL or ST terminates OSC
                self.state = ParserState::Ground;
                self.handle_osc()
            }
            0x1B => {
                // Could be ESC \ (ST)
                if let Some(&b'\\') = self.osc_buffer.last() {
                    self.osc_buffer.pop();
                    self.state = ParserState::Ground;
                    return self.handle_osc();
                }
                self.osc_buffer.push(byte);
                None
            }
            _ => {
                self.osc_buffer.push(byte);
                None
            }
        }
    }

    fn handle_osc(&mut self) -> Option<VtToken> {
        let osc_str = String::from_utf8_lossy(&self.osc_buffer);
        let mut parts = osc_str.splitn(2, ';');

        let osc_type = parts.next()?;
        let osc_data = parts.next().unwrap_or("");

        match osc_type {
            "8" => {
                // Hyperlink: OSC 8;params;URI ST
                let mut link_parts = osc_data.splitn(2, ';');
                let params = link_parts.next().unwrap_or("");
                let url = link_parts.next().unwrap_or("");

                let id = if params.starts_with("id=") {
                    Some(params[3..].to_string())
                } else {
                    None
                };

                Some(VtToken::Hyperlink {
                    url: url.to_string(),
                    id,
                })
            }
            "52" => {
                // Clipboard: OSC 52;c;base64data ST
                Some(VtToken::Clipboard(osc_data.to_string()))
            }
            "1337" => {
                // iTerm2 inline images
                if let Some(file_data) = osc_data.strip_prefix("File=") {
                    if let Some(image) = ItermImage::parse(file_data) {
                        Some(VtToken::Graphics(GraphicsCommand::Iterm(image)))
                    } else {
                        Some(VtToken::Unknown)
                    }
                } else {
                    Some(VtToken::Unknown)
                }
            }
            _ => Some(VtToken::Unknown),
        }
    }

    fn parse_dcs_entry(&mut self, byte: u8) -> Option<VtToken> {
        match byte {
            0x1B => {
                // Could be ESC \ (ST)
                if let Some(&b'\\') = self.osc_buffer.last() {
                    self.osc_buffer.pop();
                    self.state = ParserState::Ground;
                    return self.handle_dcs();
                }
                self.osc_buffer.push(byte);
                None
            }
            0x9C => {
                // ST terminates DCS
                self.state = ParserState::Ground;
                self.handle_dcs()
            }
            _ => {
                self.osc_buffer.push(byte);
                None
            }
        }
    }

    fn handle_dcs(&mut self) -> Option<VtToken> {
        let dcs_str = String::from_utf8_lossy(&self.osc_buffer);

        // Check for Kitty graphics protocol
        if dcs_str.starts_with('G') {
            if let Some(graphics) = KittyGraphics::parse(&dcs_str[1..]) {
                return Some(VtToken::Graphics(GraphicsCommand::Kitty(graphics)));
            }
        }

        Some(VtToken::Unknown)
    }
}

impl Default for VtParser {
    fn default() -> Self {
        Self::new()
    }
}
