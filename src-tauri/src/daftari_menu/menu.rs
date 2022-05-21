use tauri::WindowMenuEvent;

pub fn handle_menu_event(menu_event: WindowMenuEvent) {
    match menu_event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            menu_event.window().close().unwrap();
        }
        _ => {}
    }
}
