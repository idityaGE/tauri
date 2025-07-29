use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub age: u8,
}

#[derive(Default)]
pub struct MyState {
    pub s: Mutex<User>,
}
#[tauri::command]
pub fn login(
    state: tauri::State<'_, MyState>,
    username: String,
    password: String,
    age: u8,
) -> Result<(), String> {
    let mut user = state.s.lock().unwrap();
    println!("User Before login : {user:#?}");
    *user = User {
        username,
        password,
        age,
    };
    println!("User Adter login : {user:#?}");
    Ok(())
}

#[tauri::command]
pub fn get_user(state: tauri::State<'_, MyState>, username: String) -> User {
    println!("Getting user : {username}");
    let user = &*state.s.lock().unwrap();
    user.clone()
}

// Parsing Arguments 
// #[tauri::command]
// fn my_custom_command(invoke_message: String) {
//   println!("I was invoked from JavaScript, with this message: {}", invoke_message);
// }

// Arguments should be passed as a JSON object with camelCase keys:
// invoke('my_custom_command', { invokeMessage: 'Hello!' });


/* You can use snake_case for the arguments with the rename_all attribute:
 * #[tauri::command(rename_all = "snake_case")]
 * fn my_custom_command(invoke_message: String) {}
 * 
 * invoke('my_custom_command', { invoke_message: 'Hello!' });
 */
