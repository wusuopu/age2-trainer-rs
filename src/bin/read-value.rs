use tokio;
use trainer_rs::winapi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
  let mut dll_api = winapi::WinApi::new();
  let manager = winapi::process::ProcessManager::default();
  // let list = manager.get_process_list(&dll_api.psapi_lib, &dll_api.kernel32_lib);

  // let mut game_process: Option<winapi::process::ProcessItem> = None;
  // for p in list {
  //   if p.name.starts_with("age2") && p.name.ends_with(".exe") {
  //     println!("Process {} - {}", p.pid, p.name);
  //     game_process = Some(p);
  //     break;
  //   }
  // }

  let mut game_process: Option<winapi::process::ProcessItem> = manager.find_game_process(&dll_api.psapi_lib, &dll_api.kernel32_lib);
  if game_process.is_none() {
    println!("Cannot find Age of Empires II process");
    return Ok(());
  }

  let mut game_process = game_process.unwrap();
  if let Err(e) = dll_api.set_game_process(game_process) {
    println!("Failed to inject into process: {}", e);
    return Ok(());
  }

  println!("开始读取游戏内存数据...");

  let info = dll_api.read_game_info()?;
  println!("Game Info: {:?}", info);

  Ok(())
}