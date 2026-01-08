pub mod process;
pub mod memory;

use libloading;

/*
四项资源(float)：
0x_______0:
食物(4Bytes)            木材(4Bytes)                石头(4Bytes)                黄金(4Bytes)
人口上限差值(4Bytes)    xx                          xx                          xx
xx                      xx                          xx                          当前人口数(4Bytes)
*/

const BASE_FOOD_ADDR: u32 = 0x007A5FEC;     // 食物的基址


#[derive(Debug, Default)]
pub struct GameInfo {
  pub pid: u32,
  pub is_running: bool,
  pub food: f32,
  pub wood: f32,
  pub stone: f32,
  pub gold: f32,
  pub leave_population: f32,
  pub current_population: f32,
}

pub struct WinApi {
  pub psapi_lib: libloading::Library,
  pub kernel32_lib: libloading::Library,
  pub game_process: Option<process::ProcessItem>,
}

impl WinApi {
  pub fn new() -> Self {
    let psapi_lib= load_psapi_library();
    let kernel32_lib = load_kernel32_library();

    Self {
      psapi_lib,
      kernel32_lib,
      game_process: None,
    }
  }

  pub fn set_game_process(&mut self, p: process::ProcessItem) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = p.clone();
    p.inject(&self.kernel32_lib)?;
    self.game_process = Some(p);
    Ok(())
  }

  pub fn read_game_info(&mut self) -> Result<GameInfo, Box<dyn std::error::Error>> {
    let mut game_info = GameInfo::default();

    if self.game_process.is_none() {
      return Ok(game_info);
    }

    // 读取游戏内存数据失败
    let mut info = &mut game_info;
    if let Err(e) = self._read_game_value(info) {
      self.game_process = None;
      return Err(e);
    }

    game_info.pid = self.game_process.as_ref().unwrap().pid;

    return Ok(game_info);
  }
  fn _read_game_value(&mut self, info: &mut GameInfo) -> Result<(), Box<dyn std::error::Error>> {
    let mut value = memory::read_memory::<u32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      BASE_FOOD_ADDR,
      4
    )?;

    if value == 0 {
      // 游戏程序已运行，但是还未进入游戏状态
      return Ok(());
    }

    let food_pointer = value + 0xA8;    // 当前食物地址指针 = 基址 + 偏移地址
    let food_addr = memory::read_memory::<u32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_pointer,
      4
    )?;

    if food_addr == 0 {
      // 游戏程序已运行，但是还未进入游戏状态
      return Ok(());
    }

    info.is_running = true;
    println!("Base food addr value: 0x{:X}", food_addr);
    let mut value = memory::read_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 0,
      4
    )?;
    info.food = value;

    let mut value = memory::read_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 4,
      4
    )?;
    info.wood = value;

    let mut value = memory::read_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 8,
      4
    )?;
    info.stone = value;

    let mut value = memory::read_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 12,
      4
    )?;
    info.gold = value;

    let mut value = memory::read_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 16,
      4
    )?;
    info.leave_population = value;

    let mut value = memory::read_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 44,
      4
    )?;
    info.current_population = value;

    Ok(())
  }

  pub fn write_game_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    if self.game_process.is_none() {
      return Ok(());
    }

    // 写入游戏内存数据失败
    if let Err(e) = self._write_game_value() {
      self.game_process = None;
      return Err(e);
    }

    return Ok(());
  }
  fn _write_game_value(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let mut value = memory::read_memory::<u32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      BASE_FOOD_ADDR,
      4
    )?;
    if value == 0 {
      // 游戏程序已运行，但是还未进入游戏状态
      return Ok(());
    }

    let food_pointer = value + 0xA8;    // 当前食物地址指针 = 基址 + 偏移地址
    let food_addr = memory::read_memory::<u32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_pointer,
      4
    )?;

    if food_addr == 0 {
      // 游戏程序已运行，但是还未进入游戏状态
      return Ok(());
    }

    println!("Write 99999.0 to food pointer:");
    let new_value: f32 = 89999.0;
    memory::write_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 0,
      new_value.clone(),
      4,
    )?;

    memory::write_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 4,
      new_value.clone(),
      4,
    )?;

    memory::write_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 8,
      new_value.clone(),
      4,
    )?;

    memory::write_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 12,
      new_value.clone(),
      4,
    )?;

    memory::write_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 16,
      180.0,
      4,
    )?;

    memory::write_memory::<f32>(
      &self.kernel32_lib,
      self.game_process.as_ref().unwrap().handle,
      food_addr + 44,
      080.0,
      4,
    )?;

    return Ok(());
  }
}

pub fn load_psapi_library() -> libloading::Library {
  unsafe {
    libloading::Library::new(r"C:\Windows\System32\psapi.dll").unwrap()
  }
}
pub fn load_kernel32_library() -> libloading::Library {
  unsafe {
    libloading::Library::new(r"C:\Windows\System32\kernel32.dll").unwrap()
  }
}