// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager, Window, SystemTrayMenu, SystemTrayMenuItem, api, Env};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn close_splashscreen(window: Window){
    // Close splashscreen
    window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
    window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

fn main() {
    // Menu
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let restart = CustomMenuItem::new("Restart".to_string(), "Restart");
    let submenu = Submenu::new("File", Menu::new().add_item(quit.clone()).add_item(close).add_item(restart));
    let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

    let tray_menu = SystemTrayMenu::new()
        // .add_item(SystemTrayMenuItem::restart)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("hide", "Hide"));

    let system_tray = tauri::SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .menu(menu)
        .system_tray(system_tray)
        // .on_system_tray_event(|app, event| match event)
        .on_menu_event(|event|{
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    event.window().close().unwrap();
                }
                "Restart" => {
                    // let env : Env = Env{args : Vec::<String>::new()};
                    // let mut env : Env = Env::new({});
                    // api::process::restart(&env);
                }
                _ => {}
            }
        })
        // .setup(|app| {
        //     let docs_window = tauri::WindowBuilder::new(
        //         app,
        //         "external",
        //         tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
        //     ).build()?;
        //     Ok(())
        // })
        .setup(move |app| {
            // print!("{:?}", app);

            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                println!("initializing...");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("Done initializaing.");

                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
                main_window.open_devtools();
            });
            // 이 코드 넣으면 좀비프로세스가 생성됨;;
            // let handler = app.handle();
            // std::thread::spawn(move || {
            //     handler.restart();
            // });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
