#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_basic_text() {
        let mut parser = VtParser::new();
        let tokens = parser.parse(b"Hello");

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], VtToken::Print('H')));
        assert!(matches!(tokens[1], VtToken::Print('e')));
        assert!(matches!(tokens[2], VtToken::Print('l')));
        assert!(matches!(tokens[3], VtToken::Print('l')));
        assert!(matches!(tokens[4], VtToken::Print('o')));
    }

    #[test]
    fn test_carriage_return_linefeed() {
        let mut parser = VtParser::new();
        let tokens = parser.parse(b"Test\r\n");

        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[4], VtToken::CarriageReturn));
        assert!(matches!(tokens[5], VtToken::LineFeed));
    }

    #[test]
    fn test_cursor_movement() {
        let mut parser = VtParser::new();

        // Test cursor position ESC[H
        let tokens = parser.parse(b"\x1b[H");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::CursorMove(0, 0)));

        // Test cursor up ESC[A
        let tokens = parser.parse(b"\x1b[A");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::CursorRelative(0, -1)));

        // Test cursor down ESC[3B (move 3 down)
        let tokens = parser.parse(b"\x1b[3B");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::CursorRelative(0, 3)));
    }

    #[test]
    fn test_sgr_colors() {
        let mut parser = VtParser::new();

        // Reset
        let tokens = parser.parse(b"\x1b[0m");
        assert_eq!(tokens.len(), 1);
        if let VtToken::SetGraphics(attrs) = &tokens[0] {
            assert_eq!(attrs.foreground, Color::Default);
            assert_eq!(attrs.background, Color::Default);
        } else {
            panic!("Expected SetGraphics token");
        }

        // Foreground color
        let tokens = parser.parse(b"\x1b[31m");
        assert_eq!(tokens.len(), 1);
        if let VtToken::SetGraphics(attrs) = &tokens[0] {
            assert_eq!(attrs.foreground, Color::Indexed(1)); // Red
        } else {
            panic!("Expected SetGraphics token");
        }

        // RGB color
        let tokens = parser.parse(b"\x1b[38;2;255;128;64m");
        assert_eq!(tokens.len(), 1);
        if let VtToken::SetGraphics(attrs) = &tokens[0] {
            assert_eq!(attrs.foreground, Color::Rgb(255, 128, 64));
        } else {
            panic!("Expected SetGraphics token");
        }
    }

    #[test]
    fn test_sgr_attributes() {
        let mut parser = VtParser::new();

        // Bold
        let tokens = parser.parse(b"\x1b[1m");
        assert_eq!(tokens.len(), 1);
        if let VtToken::SetGraphics(attrs) = &tokens[0] {
            assert!(attrs.bold);
        } else {
            panic!("Expected SetGraphics token");
        }

        // Italic and underline
        let tokens = parser.parse(b"\x1b[3;4m");
        assert_eq!(tokens.len(), 1);
        if let VtToken::SetGraphics(attrs) = &tokens[0] {
            assert!(attrs.italic);
            assert!(attrs.underline);
        } else {
            panic!("Expected SetGraphics token");
        }
    }

    #[test]
    fn test_clear_screen() {
        let mut parser = VtParser::new();
        let tokens = parser.parse(b"\x1b[2J");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::ClearScreen));
    }

    #[test]
    fn test_clear_line() {
        let mut parser = VtParser::new();
        let tokens = parser.parse(b"\x1b[K");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::ClearLine));
    }

    #[test]
    fn test_mixed_content() {
        let mut parser = VtParser::new();
        let tokens = parser.parse(b"Hello\x1b[31mWorld\x1b[0m!");

        // Should have: H e l l o [31m W o r l d [0m !
        assert!(tokens.len() >= 11);

        // Check first chars
        assert!(matches!(tokens[0], VtToken::Print('H')));
        assert!(matches!(tokens[4], VtToken::Print('o')));

        // Find the color change
        let has_color = tokens.iter().any(|t| matches!(t, VtToken::SetGraphics(_)));
        assert!(has_color);
    }

    #[test]
    fn test_tab_and_backspace() {
        let mut parser = VtParser::new();

        let tokens = parser.parse(b"\t");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::Tab));

        let tokens = parser.parse(b"\x08");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::Backspace));
    }

    #[test]
    fn test_bell() {
        let mut parser = VtParser::new();
        let tokens = parser.parse(b"\x07");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], VtToken::Bell));
    }
}
