use tokio;
use age2_trainer_rs::winapi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
  let psapi_lib = winapi::load_psapi_library();
  let kernel32_lib = winapi::load_kernel32_library();
  let manager = winapi::process::ProcessManager::default();
  let list = manager.get_process_list(&psapi_lib, &kernel32_lib);

  for p in list {
    println!("Process {} - {}", p.pid, p.name);
  }
  Ok(())
}