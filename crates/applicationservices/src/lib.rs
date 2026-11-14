type CGWindowLevel = i32;

/// `CGWindowLevelKey`, as the `int32_t` a guest passes it as: a key outside the
/// enumeration is a value like any other, which a Rust `enum` could not hold.
type CGWindowLevelKey = i32;

/// The `kCG...WindowLevelKey`s of `<CoreGraphics/CGWindowLevel.h>`.
mod key {
    use super::CGWindowLevelKey;

    pub const BASE: CGWindowLevelKey = 0;
    pub const MINIMUM: CGWindowLevelKey = 1;
    pub const DESKTOP: CGWindowLevelKey = 2;
    pub const BACKSTOP_MENU: CGWindowLevelKey = 3;
    pub const NORMAL: CGWindowLevelKey = 4;
    pub const FLOATING: CGWindowLevelKey = 5;
    pub const TORN_OFF_MENU: CGWindowLevelKey = 6;
    pub const DOCK: CGWindowLevelKey = 7;
    pub const MAIN_MENU: CGWindowLevelKey = 8;
    pub const STATUS: CGWindowLevelKey = 9;
    pub const MODAL_PANEL: CGWindowLevelKey = 10;
    pub const POP_UP_MENU: CGWindowLevelKey = 11;
    pub const DRAGGING: CGWindowLevelKey = 12;
    pub const SCREEN_SAVER: CGWindowLevelKey = 13;
    pub const MAXIMUM: CGWindowLevelKey = 14;
    pub const OVERLAY: CGWindowLevelKey = 15;
    pub const HELP: CGWindowLevelKey = 16;
    pub const UTILITY: CGWindowLevelKey = 17;
    pub const DESKTOP_ICON: CGWindowLevelKey = 18;
    pub const CURSOR: CGWindowLevelKey = 19;
    pub const ASSISTIVE_TECH_HIGH: CGWindowLevelKey = 20;
}

/// The `kCG...WindowLevel` each key names, which `NSWindowLevel`'s constants
/// are defined from.
mod level {
    use super::CGWindowLevel;

    pub const BASE: CGWindowLevel = i32::MIN;
    pub const MINIMUM: CGWindowLevel = BASE + 5;
    pub const DESKTOP: CGWindowLevel = MINIMUM + 20;
    pub const DESKTOP_ICON: CGWindowLevel = DESKTOP + 20;
    pub const BACKSTOP_MENU: CGWindowLevel = -20;
    pub const NORMAL: CGWindowLevel = 0;
    pub const FLOATING: CGWindowLevel = 3;
    pub const TORN_OFF_MENU: CGWindowLevel = 3;
    pub const MODAL_PANEL: CGWindowLevel = 8;
    pub const UTILITY: CGWindowLevel = 19;
    pub const DOCK: CGWindowLevel = 20;
    pub const MAIN_MENU: CGWindowLevel = 24;
    pub const STATUS: CGWindowLevel = 25;
    pub const POP_UP_MENU: CGWindowLevel = 101;
    pub const OVERLAY: CGWindowLevel = 102;
    pub const HELP: CGWindowLevel = 200;
    pub const DRAGGING: CGWindowLevel = 500;
    pub const SCREEN_SAVER: CGWindowLevel = 1000;
    pub const ASSISTIVE_TECH_HIGH: CGWindowLevel = 1500;
    pub const MAXIMUM: CGWindowLevel = i32::MAX - 16;
    pub const CURSOR: CGWindowLevel = MAXIMUM - 1;
}

/// `CGWindowLevel CGWindowLevelForKey(CGWindowLevelKey)`
#[allow(non_snake_case)]
extern "C" fn CGWindowLevelForKey(key: CGWindowLevelKey) -> CGWindowLevel {
    match key {
        key::BASE => level::BASE,
        key::MINIMUM => level::MINIMUM,
        key::DESKTOP => level::DESKTOP,
        key::BACKSTOP_MENU => level::BACKSTOP_MENU,
        key::NORMAL => level::NORMAL,
        key::FLOATING => level::FLOATING,
        key::TORN_OFF_MENU => level::TORN_OFF_MENU,
        key::DOCK => level::DOCK,
        key::MAIN_MENU => level::MAIN_MENU,
        key::STATUS => level::STATUS,
        key::MODAL_PANEL => level::MODAL_PANEL,
        key::POP_UP_MENU => level::POP_UP_MENU,
        key::DRAGGING => level::DRAGGING,
        key::SCREEN_SAVER => level::SCREEN_SAVER,
        key::MAXIMUM => level::MAXIMUM,
        key::OVERLAY => level::OVERLAY,
        key::HELP => level::HELP,
        key::UTILITY => level::UTILITY,
        key::DESKTOP_ICON => level::DESKTOP_ICON,
        key::CURSOR => level::CURSOR,
        key::ASSISTIVE_TECH_HIGH => level::ASSISTIVE_TECH_HIGH,
        // FIXME (ACCURACY): what a Mac answers for a key outside the
        // enumeration is not checked; zero is the normal level.
        _ => level::NORMAL,
    }
}

/// `Boolean UAZoomEnabled(void)`: whether the screen zoom is on, which it
/// never is for a guest.
#[allow(non_snake_case)]
extern "C" fn UAZoomEnabled() -> u8 {
    0
}

/// `OSStatus UAZoomChangeFocus(const CGRect *, const CGRect *, UAZoomChangeFocusType)`:
/// where the zoom should look. With no zoom there is nothing to move.
#[allow(non_snake_case)]
extern "C" fn UAZoomChangeFocus(_rect: *const f64, _highlight: *const f64, _kind: u32) -> i32 {
    0
}

exports::exports! {
    functions {
        "UAZoomEnabled" => UAZoomEnabled,
        "UAZoomChangeFocus" => UAZoomChangeFocus,
        "CGWindowLevelForKey" => CGWindowLevelForKey,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AppKit's `NSWindowLevel` constants are these numbers, and a guest that
    /// asks CoreGraphics instead has to be told the same ones.
    #[test]
    fn window_level_keys_name_appkits_levels() {
        assert_eq!(CGWindowLevelForKey(key::NORMAL), 0);
        assert_eq!(CGWindowLevelForKey(key::FLOATING), 3);
        assert_eq!(CGWindowLevelForKey(key::MODAL_PANEL), 8);
        assert_eq!(CGWindowLevelForKey(key::MAIN_MENU), 24);
        assert_eq!(CGWindowLevelForKey(key::STATUS), 25);
        assert_eq!(CGWindowLevelForKey(key::POP_UP_MENU), 101);
        assert_eq!(CGWindowLevelForKey(key::SCREEN_SAVER), 1000);
    }

    #[test]
    fn window_levels_span_the_whole_range() {
        assert_eq!(CGWindowLevelForKey(key::BASE), -2147483648);
        assert_eq!(CGWindowLevelForKey(key::MINIMUM), -2147483643);
        assert_eq!(CGWindowLevelForKey(key::DESKTOP), -2147483623);
        assert_eq!(CGWindowLevelForKey(key::DESKTOP_ICON), -2147483603);
        assert_eq!(CGWindowLevelForKey(key::CURSOR), 2147483630);
        assert_eq!(CGWindowLevelForKey(key::MAXIMUM), 2147483631);
    }

    #[test]
    fn a_key_outside_the_enumeration_is_a_value_like_any_other() {
        assert_eq!(CGWindowLevelForKey(-1), level::NORMAL);
        assert_eq!(
            CGWindowLevelForKey(key::ASSISTIVE_TECH_HIGH + 1),
            level::NORMAL
        );
        assert_eq!(CGWindowLevelForKey(i32::MAX), level::NORMAL);
    }
}
