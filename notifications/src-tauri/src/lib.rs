use tauri::{command, AppHandle};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[command]
fn greet(name: &str, app: AppHandle) {
    // Create the greeting message
    let greeting_message = format!("Hello, {}! You've been greeted from Rust!", name);

    // Show the message dialog asynchronously (non-blocking)
    tauri::async_runtime::spawn(async move {
        // Display the dialog with Ok/Cancel buttons
        app.dialog()
            .message(&greeting_message) // The greeting message
            .title("Greeting") // Dialog title
            .buttons(MessageDialogButtons::OkCancel) // Buttons: Ok/Cancel
            .show(|result| {
                if result {
                    // If Ok is clicked
                    println!("User clicked Ok");
                } else {
                    // If Cancel is clicked
                    println!("User clicked Cancel");
                }
            });
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init()) // Initialize the dialog plugin
        .plugin(tauri_plugin_shell::init()) // Initialize the shell plugin (optional)
        .invoke_handler(tauri::generate_handler![greet]) // Register the greet command
        .run(tauri::generate_context!()) // Run the application
        .expect("error while running tauri application");
}
