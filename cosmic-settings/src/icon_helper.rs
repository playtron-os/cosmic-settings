// Helper module for mapping freedesktop icon names to icetron_assets SVG bytes.

use cosmic::widget::icon;

/// Create an icon widget from a freedesktop icon name, using icetron_assets when available
pub fn named_icon(name: &str, size: u16) -> icon::Icon {
    if let Some(bytes) = icon_bytes_for_name(name) {
        icon::icon(icon::from_svg_bytes(bytes)).size(size)
    } else {
        icon::from_name(name).size(size).icon()
    }
}

/// Create an icon Handle from a freedesktop icon name, using icetron_assets when available
pub fn named_handle(name: &str, size: u16) -> icon::Handle {
    if let Some(bytes) = icon_bytes_for_name(name) {
        icon::from_svg_bytes(bytes)
    } else {
        icon::from_name(name).size(size).handle()
    }
}

/// Map known freedesktop icon names to icetron_assets
fn icon_bytes_for_name(name: &str) -> Option<&'static [u8]> {
    use icetron_assets::icons::*;
    match name {
        // System / Settings
        "preferences-system-symbolic" => Some(system::SETTINGS_3_LINE),
        "preferences-system" => Some(system::SETTINGS_3_LINE),

        // Navigation arrows
        "go-next-symbolic" => Some(system::ARROW_RIGHT_S_LINE),
        "go-previous-symbolic" => Some(system::ARROW_LEFT_S_LINE),
        "go-up-symbolic" => Some(system::ARROW_UP_S_LINE),
        "go-down-symbolic" => Some(system::ARROW_DOWN_S_LINE),

        // Desktop
        "video-display-symbolic" => Some(device::COMPUTER_LINE),

        // Panel / Dock / Layout
        "preferences-panel-symbolic" => Some(design::LAYOUT_TOP_LINE),
        "preferences-dock-symbolic" => Some(design::LAYOUT_BOTTOM_LINE),

        // Appearance
        "preferences-appearance-symbolic" => Some(design::PALETTE_LINE),

        // Wallpaper
        "preferences-desktop-wallpaper-symbolic" => Some(media::IMAGE_LINE),

        // Workspaces
        "preferences-workspaces-symbolic" => Some(design::LAYOUT_GRID_LINE),

        // Window Management
        "preferences-window-management-symbolic" => Some(design::LAYOUT_LINE),

        // Sound
        "preferences-sound-symbolic" => Some(media::VOLUME_UP_LINE),
        "microphone-sensitivity-muted-symbolic" => Some(media::MIC_OFF_LINE),
        "audio-input-microphone-symbolic" => Some(media::MIC_LINE),
        "audio-volume-muted-symbolic" => Some(media::VOLUME_MUTE_LINE),
        "audio-volume-high-symbolic" => Some(media::VOLUME_UP_LINE),

        // Time & Language
        "preferences-time-and-language-symbolic" => Some(system::TIME_LINE),
        "preferences-system-time-symbolic" => Some(system::TIME_LINE),
        "preferences-region-and-language-symbolic" => Some(development::GLOBE_LINE),

        // System / Users
        "system-users-symbolic" => Some(users::GROUP_LINE),
        "help-about-symbolic" => Some(system::INFORMATION_LINE),

        // Networking
        "preferences-network-and-wireless-symbolic" => Some(device::WIFI_LINE),
        "preferences-wireless-symbolic" => Some(device::WIFI_LINE),
        "preferences-wired-symbolic" => Some(device::ETHERNET),

        // Display
        "preferences-desktop-display-symbolic" => Some(device::COMPUTER_LINE),

        // Input devices
        "preferences-input-devices-symbolic" => Some(device::INPUT_DEVICES_LINE),
        "input-mouse-symbolic" => Some(device::MOUSE_LINE),
        "input-touchpad-symbolic" => Some(device::MOUSE_LINE),
        "input-keyboard-symbolic" => Some(device::KEYBOARD_LINE),

        // Applications
        "preferences-applications-symbolic" => Some(system::APPS_LINE),
        "preferences-startup-applications-symbolic" => Some(system::APPS_2_LINE),
        "preferences-default-applications-symbolic" => Some(system::APPS_LINE),

        // Bluetooth
        "bluetooth-symbolic" => Some(device::BLUETOOTH_LINE),

        // Common UI icons
        "object-select-symbolic" => Some(system::CHECK_LINE),
        "edit-delete-symbolic" => Some(system::DELETE_BIN_LINE),
        "view-more-symbolic" => Some(system::MORE_LINE),
        "dialog-information" => Some(system::INFORMATION_LINE),
        "dialog-warning" => Some(system::ERROR_WARNING_LINE),
        "dialog-error-symbolic" => Some(system::ERROR_LINE),
        "window-pop-out-symbolic" => Some(system::EXTERNAL_LINK_LINE),
        "grip-lines-symbolic" => Some(system::GRIP),
        "folder-symbolic" => Some(system::FOLDER),
        "airplane-mode-symbolic" => Some(device::AIRPLANE_MODE),
        "connection-secure-symbolic" => Some(system::LOCK_LINE),

        // WiFi signal strength
        "network-wireless-signal-excellent-symbolic" => Some(device::WIFI_STRONGEST),
        "network-wireless-signal-good-symbolic" => Some(device::WIFI_STRONGE),
        "network-wireless-signal-ok-symbolic" => Some(device::WIFI_GOOD),
        "network-wireless-signal-weak-symbolic" => Some(device::WIFI_WEAK),

        // Power/Battery device type icons
        "display-symbolic" => Some(device::COMPUTER_LINE),
        "smartphone-symbolic" => Some(device::SMARTPHONE_LINE),
        "tablet-symbolic" => Some(device::TABLET_LINE),
        "input-gaming-symbolic" => Some(device::GAMEPAD_LINE),
        "network-wired-symbolic" => Some(device::ETHERNET),
        "audio-headphones-symbolic" | "audio-headset-symbolic" => Some(device::HEADPHONE_LINE),
        "camera-photo-symbolic" => Some(media::CAMERA_LINE),

        _ => None,
    }
}
