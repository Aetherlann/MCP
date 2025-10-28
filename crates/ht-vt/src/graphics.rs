use serde::{Serialize, Deserialize};

/// Graphics protocol commands
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphicsCommand {
    Kitty(KittyGraphics),
    Iterm(ItermImage),
    Sixel(Vec<u8>),
}

/// Kitty Graphics Protocol
/// Format: ESC_G<key>=<value>,...;<payload>ESC\
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KittyGraphics {
    pub action: KittyAction,
    pub format: KittyFormat,
    pub transmission: KittyTransmission,
    pub placement: KittyPlacement,
    pub image_id: Option<u32>,
    pub image_number: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x_offset: Option<u32>,
    pub y_offset: Option<u32>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KittyAction {
    Transmit,
    TransmitAndDisplay,
    Query,
    Display,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KittyFormat {
    Rgb,
    Rgba,
    Png,
    Jpeg,
    Webp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KittyTransmission {
    Direct,
    File,
    TempFile,
    SharedMem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KittyPlacement {
    Inline,
    Overlay,
}

impl Default for KittyGraphics {
    fn default() -> Self {
        Self {
            action: KittyAction::TransmitAndDisplay,
            format: KittyFormat::Png,
            transmission: KittyTransmission::Direct,
            placement: KittyPlacement::Inline,
            image_id: None,
            image_number: None,
            width: None,
            height: None,
            x_offset: None,
            y_offset: None,
            rows: None,
            cols: None,
            payload: Vec::new(),
        }
    }
}

impl KittyGraphics {
    pub fn parse(data: &str) -> Option<Self> {
        let mut parts = data.split(';');
        let keys = parts.next()?;
        let payload_str = parts.next().unwrap_or("");

        let mut graphics = KittyGraphics::default();

        // Parse key-value pairs
        for pair in keys.split(',') {
            let mut kv = pair.split('=');
            let key = kv.next()?;
            let value = kv.next()?;

            match key {
                "a" => graphics.action = Self::parse_action(value)?,
                "f" => graphics.format = Self::parse_format(value)?,
                "t" => graphics.transmission = Self::parse_transmission(value)?,
                "p" => graphics.placement = Self::parse_placement(value)?,
                "i" => graphics.image_id = value.parse().ok(),
                "I" => graphics.image_number = value.parse().ok(),
                "w" => graphics.width = value.parse().ok(),
                "h" => graphics.height = value.parse().ok(),
                "x" => graphics.x_offset = value.parse().ok(),
                "y" => graphics.y_offset = value.parse().ok(),
                "r" => graphics.rows = value.parse().ok(),
                "c" => graphics.cols = value.parse().ok(),
                _ => {}
            }
        }

        // Decode base64 payload
        if !payload_str.is_empty() {
            graphics.payload = base64::decode(payload_str).ok()?;
        }

        Some(graphics)
    }

    fn parse_action(value: &str) -> Option<KittyAction> {
        match value {
            "t" | "T" => Some(KittyAction::Transmit),
            "a" | "q" => Some(KittyAction::TransmitAndDisplay),
            "Q" => Some(KittyAction::Query),
            "p" => Some(KittyAction::Display),
            "d" => Some(KittyAction::Delete),
            _ => None,
        }
    }

    fn parse_format(value: &str) -> Option<KittyFormat> {
        match value {
            "24" => Some(KittyFormat::Rgb),
            "32" => Some(KittyFormat::Rgba),
            "100" => Some(KittyFormat::Png),
            "101" => Some(KittyFormat::Jpeg),
            "102" => Some(KittyFormat::Webp),
            _ => None,
        }
    }

    fn parse_transmission(value: &str) -> Option<KittyTransmission> {
        match value {
            "d" => Some(KittyTransmission::Direct),
            "f" => Some(KittyTransmission::File),
            "t" => Some(KittyTransmission::TempFile),
            "s" => Some(KittyTransmission::SharedMem),
            _ => None,
        }
    }

    fn parse_placement(value: &str) -> Option<KittyPlacement> {
        match value {
            "0" | "1" => Some(KittyPlacement::Inline),
            "2" => Some(KittyPlacement::Overlay),
            _ => None,
        }
    }
}

/// iTerm2 Inline Images
/// Format: OSC 1337;File=<args>:<base64> BEL
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItermImage {
    pub name: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub preserve_aspect_ratio: bool,
    pub inline: bool,
    pub payload: Vec<u8>,
}

impl ItermImage {
    pub fn parse(data: &str) -> Option<Self> {
        let mut parts = data.split(':');
        let args = parts.next()?;
        let payload_str = parts.next()?;

        let mut image = ItermImage {
            name: None,
            width: None,
            height: None,
            preserve_aspect_ratio: true,
            inline: true,
            payload: Vec::new(),
        };

        // Parse arguments
        for pair in args.split(';') {
            let mut kv = pair.split('=');
            let key = kv.next()?;
            let value = kv.next().unwrap_or("");

            match key {
                "name" => image.name = Some(base64::decode(value).ok().and_then(|v| String::from_utf8(v).ok())?),
                "width" => image.width = Some(value.to_string()),
                "height" => image.height = Some(value.to_string()),
                "preserveAspectRatio" => image.preserve_aspect_ratio = value == "1",
                "inline" => image.inline = value == "1",
                _ => {}
            }
        }

        // Decode payload
        image.payload = base64::decode(payload_str).ok()?;

        Some(image)
    }
}
