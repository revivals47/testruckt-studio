//! Canvas keyboard input handling
//!
//! Implements keyboard shortcuts and command handling for the canvas.

/// Keyboard command
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardCommand {
    /// Delete selected objects
    Delete,
    /// Undo last action
    Undo,
    /// Redo last undone action
    Redo,
    /// Select all objects
    SelectAll,
    /// Deselect all objects
    DeselectAll,
    /// Duplicate selected objects
    Duplicate,
    /// Copy to clipboard
    Copy,
    /// Paste from clipboard
    Paste,
    /// Cut to clipboard
    Cut,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Reset zoom to 100%
    ZoomReset,
    /// Move selected objects left
    MoveLeft,
    /// Move selected objects right
    MoveRight,
    /// Move selected objects up
    MoveUp,
    /// Move selected objects down
    MoveDown,
    /// Group selected objects
    Group,
    /// Ungroup selected objects
    Ungroup,
}

/// Detect keyboard command from GTK key event
pub fn detect_keyboard_command(
    keyval: u32,
    state: gtk4::gdk::ModifierType,
) -> Option<KeyboardCommand> {
    use gtk4::gdk;

    // Key codes (from gdk4-sys or standard X11)
    const KEY_DELETE: u32 = 0xffff;
    const KEY_Z: u32 = 0x007a;
    const KEY_Y: u32 = 0x0079;
    const KEY_A: u32 = 0x0061;
    const KEY_D: u32 = 0x0064;
    const KEY_C: u32 = 0x0063;
    const KEY_V: u32 = 0x0076;
    const KEY_X: u32 = 0x0078;
    const KEY_G: u32 = 0x0067;
    const KEY_0: u32 = 0x0030;
    const KEY_PLUS: u32 = 0x002b;
    const KEY_EQUAL: u32 = 0x003d;
    const KEY_MINUS: u32 = 0x002d;
    const KEY_ESCAPE: u32 = 0xff1b;
    const KEY_LEFT: u32 = 0xff51;
    const KEY_RIGHT: u32 = 0xff53;
    const KEY_UP: u32 = 0xff52;
    const KEY_DOWN: u32 = 0xff54;

    match keyval {
        // Delete key
        KEY_DELETE => Some(KeyboardCommand::Delete),

        // Ctrl+Z: Undo
        KEY_Z if state.contains(gdk::ModifierType::CONTROL_MASK) => Some(KeyboardCommand::Undo),

        // Ctrl+Y or Ctrl+Shift+Z: Redo
        KEY_Y if state.contains(gdk::ModifierType::CONTROL_MASK) => Some(KeyboardCommand::Redo),
        KEY_Z
            if state.contains(gdk::ModifierType::CONTROL_MASK)
                && state.contains(gdk::ModifierType::SHIFT_MASK) =>
        {
            Some(KeyboardCommand::Redo)
        }

        // Ctrl+A: Select All
        KEY_A if state.contains(gdk::ModifierType::CONTROL_MASK) => {
            Some(KeyboardCommand::SelectAll)
        }

        // Escape: Deselect All
        KEY_ESCAPE => Some(KeyboardCommand::DeselectAll),

        // Ctrl+D: Duplicate
        KEY_D if state.contains(gdk::ModifierType::CONTROL_MASK) => {
            Some(KeyboardCommand::Duplicate)
        }

        // Ctrl+C: Copy
        KEY_C if state.contains(gdk::ModifierType::CONTROL_MASK) => Some(KeyboardCommand::Copy),

        // Ctrl+V: Paste
        KEY_V if state.contains(gdk::ModifierType::CONTROL_MASK) => Some(KeyboardCommand::Paste),

        // Ctrl+X: Cut
        KEY_X if state.contains(gdk::ModifierType::CONTROL_MASK) => Some(KeyboardCommand::Cut),

        // Ctrl++: Zoom In
        KEY_PLUS | KEY_EQUAL if state.contains(gdk::ModifierType::CONTROL_MASK) => {
            Some(KeyboardCommand::ZoomIn)
        }

        // Ctrl+-: Zoom Out
        KEY_MINUS if state.contains(gdk::ModifierType::CONTROL_MASK) => {
            Some(KeyboardCommand::ZoomOut)
        }

        // Ctrl+0: Reset Zoom
        KEY_0 if state.contains(gdk::ModifierType::CONTROL_MASK) => {
            Some(KeyboardCommand::ZoomReset)
        }

        // Arrow keys for moving objects
        KEY_LEFT => Some(KeyboardCommand::MoveLeft),
        KEY_RIGHT => Some(KeyboardCommand::MoveRight),
        KEY_UP => Some(KeyboardCommand::MoveUp),
        KEY_DOWN => Some(KeyboardCommand::MoveDown),

        // Ctrl+G: Group
        KEY_G if state.contains(gdk::ModifierType::CONTROL_MASK) => Some(KeyboardCommand::Group),

        // Ctrl+Shift+G: Ungroup
        KEY_G
            if state.contains(gdk::ModifierType::CONTROL_MASK)
                && state.contains(gdk::ModifierType::SHIFT_MASK) =>
        {
            Some(KeyboardCommand::Ungroup)
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_command_names() {
        assert_eq!(format!("{:?}", KeyboardCommand::Delete), "Delete");
        assert_eq!(format!("{:?}", KeyboardCommand::Undo), "Undo");
    }
}
