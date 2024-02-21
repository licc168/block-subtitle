// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use rdev::{listen, Event, EventType, Key};
use tauri::Manager;

fn get_key_name(key: Key) -> String {
    let key_some = Key::from(key);
    return match key_some {
        // Alt key on Linux and Windows (option key on macOS)
        rdev::Key::Alt => "Alt".to_string(),
        rdev::Key::AltGr => "AltGr".to_string(),
        rdev::Key::Backspace => "Backspace".to_string(),
        rdev::Key::CapsLock => "CapsLock".to_string(),
        rdev::Key::ControlLeft => "ControlLeft".to_string(),
        rdev::Key::ControlRight => "ControlRight".to_string(),
        rdev::Key::Delete => "Delete".to_string(),
        rdev::Key::DownArrow => "DownArrow".to_string(),
        rdev::Key::End => "End".to_string(),
        rdev::Key::Escape => "Escape".to_string(),
        rdev::Key::F1 => "F1".to_string(),
        rdev::Key::F10 => "F10".to_string(),
        rdev::Key::F11 => "F11".to_string(),
        rdev::Key::F12 => "F12".to_string(),
        rdev::Key::F2 => "F2".to_string(),
        rdev::Key::F3 => "F3".to_string(),
        rdev::Key::F4 => "F4".to_string(),
        rdev::Key::F5 => "F5".to_string(),
        rdev::Key::F6 => "F6".to_string(),
        rdev::Key::F7 => "F7".to_string(),
        rdev::Key::F8 => "F8".to_string(),
        rdev::Key::F9 => "F9".to_string(),
        rdev::Key::Home => "Home".to_string(),
        rdev::Key::LeftArrow => "LeftArrow".to_string(),
        // also known as windows.to_string(), super.to_string(), and command
        rdev::Key::MetaLeft => "MetaLeft".to_string(),
        // also known as windows.to_string(), super.to_string(), and command
        rdev::Key::MetaRight => "MetaRight".to_string(),
        rdev::Key::PageDown => "PageDown".to_string(),
        rdev::Key::PageUp => "PageUp".to_string(),
        rdev::Key::Return => "Return".to_string(),
        rdev::Key::RightArrow => "RightArrow".to_string(),
        rdev::Key::ShiftLeft => "ShiftLeft".to_string(),
        rdev::Key::ShiftRight => "ShiftRight".to_string(),
        rdev::Key::Space => "Space".to_string(),
        rdev::Key::Tab => "Tab".to_string(),
        rdev::Key::UpArrow => "UpArrow".to_string(),
        rdev::Key::PrintScreen => "PrintScreen".to_string(),
        rdev::Key::ScrollLock => "ScrollLock".to_string(),
        rdev::Key::Pause => "Pause".to_string(),
        rdev::Key::NumLock => "NumLock".to_string(),
        rdev::Key::BackQuote => "BackQuote".to_string(),
        rdev::Key::Num1 => "Num1".to_string(),
        rdev::Key::Num2 => "Num2".to_string(),
        rdev::Key::Num3 => "Num3".to_string(),
        rdev::Key::Num4 => "Num4".to_string(),
        rdev::Key::Num5 => "Num5".to_string(),
        rdev::Key::Num6 => "Num6".to_string(),
        rdev::Key::Num7 => "Num7".to_string(),
        rdev::Key::Num8 => "Num8".to_string(),
        rdev::Key::Num9 => "Num9".to_string(),
        rdev::Key::Num0 => "Num0".to_string(),
        rdev::Key::Minus => "Minus".to_string(),
        rdev::Key::Equal => "Equal".to_string(),
        rdev::Key::KeyQ => "KeyQ".to_string(),
        rdev::Key::KeyW => "KeyW".to_string(),
        rdev::Key::KeyE => "KeyE".to_string(),
        rdev::Key::KeyR => "KeyR".to_string(),
        rdev::Key::KeyT => "KeyT".to_string(),
        rdev::Key::KeyY => "KeyY".to_string(),
        rdev::Key::KeyU => "KeyU".to_string(),
        rdev::Key::KeyI => "KeyI".to_string(),
        rdev::Key::KeyO => "KeyO".to_string(),
        rdev::Key::KeyP => "KeyP".to_string(),
        rdev::Key::LeftBracket => "LeftBracket".to_string(),
        rdev::Key::RightBracket => "RightBracket".to_string(),
        rdev::Key::KeyA => "KeyA".to_string(),
        rdev::Key::KeyS => "KeyS".to_string(),
        rdev::Key::KeyD => "KeyD".to_string(),
        rdev::Key::KeyF => "KeyF".to_string(),
        rdev::Key::KeyG => "KeyG".to_string(),
        rdev::Key::KeyH => "KeyH".to_string(),
        rdev::Key::KeyJ => "KeyJ".to_string(),
        rdev::Key::KeyK => "KeyK".to_string(),
        rdev::Key::KeyL => "KeyL".to_string(),
        rdev::Key::SemiColon => "SemiColon".to_string(),
        rdev::Key::Quote => "Quote".to_string(),
        rdev::Key::BackSlash => "BackSlash".to_string(),
        rdev::Key::IntlBackslash => "IntlBackslash".to_string(),
        rdev::Key::KeyZ => "KeyZ".to_string(),
        rdev::Key::KeyX => "KeyX".to_string(),
        rdev::Key::KeyC => "KeyC".to_string(),
        rdev::Key::KeyV => "KeyV".to_string(),
        rdev::Key::KeyB => "KeyB".to_string(),
        rdev::Key::KeyN => "KeyN".to_string(),
        rdev::Key::KeyM => "KeyM".to_string(),
        rdev::Key::Comma => "Comma".to_string(),
        rdev::Key::Dot => "Dot".to_string(),
        rdev::Key::Slash => "Slash".to_string(),
        rdev::Key::Insert => "Insert".to_string(),
        rdev::Key::KpReturn => "KpReturn".to_string(),
        rdev::Key::KpMinus => "KpMinus".to_string(),
        rdev::Key::KpPlus => "KpPlus".to_string(),
        rdev::Key::KpMultiply => "KpMultiply".to_string(),
        rdev::Key::KpDivide => "KpDivide".to_string(),
        rdev::Key::Kp0 => "Kp0".to_string(),
        rdev::Key::Kp1 => "Kp1".to_string(),
        rdev::Key::Kp2 => "Kp2".to_string(),
        rdev::Key::Kp3 => "Kp3".to_string(),
        rdev::Key::Kp4 => "Kp4".to_string(),
        rdev::Key::Kp5 => "Kp5".to_string(),
        rdev::Key::Kp6 => "Kp6".to_string(),
        rdev::Key::Kp7 => "Kp7".to_string(),
        rdev::Key::Kp8 => "Kp8".to_string(),
        rdev::Key::Kp9 => "Kp9".to_string(),
        rdev::Key::KpDelete => "KpDelete".to_string(),
        rdev::Key::Function => "Function".to_string(),
        rdev::Key::Unknown(_) => "Unknown".to_string(),
    };
}
#[derive(Clone, serde::Serialize)]
struct Payload {
    action: String,
    key: String,
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let callback = move |event: Event| match event.event_type {

                EventType::KeyRelease(release_key) => main_window
                    .emit(
                        "key",
                        Payload {
                            action: "release".into(),
                            key: get_key_name(release_key),
                        },
                    )
                    .unwrap(),
                _ => (),
            };



            tauri::async_runtime::spawn(async move {
                if let Err(error) = listen(callback) {
                    println!("Error: {:?}", error)
                }
            });

            Ok(())
        })
        .device_event_filter(tauri::DeviceEventFilter::Always)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
