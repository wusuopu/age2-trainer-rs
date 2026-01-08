use libloading;
use std::path;

const DWORD_SIZE: u32= 4;
const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
const PROCESS_VM_OPERATION: u32 = 0x0008;
const PROCESS_VM_READ: u32 = 0x0010;
const PROCESS_VM_WRITE: u32 = 0x0020;


#[derive(Debug, Clone)]
pub struct ProcessItem {
  pub pid: u32,
  pub name: String,
  pub(crate) handle: u32,
}

impl ProcessItem {
    pub fn from_pid(pid: u32, psapi_lib: &libloading::Library, kernel32_lib: &libloading::Library) -> Result<ProcessItem, Box<dyn std::error::Error>> {
      unsafe {
        let open_func = kernel32_lib.get::<unsafe extern "system" fn(u32, bool, u32) -> u32>(b"OpenProcess\0").unwrap();
        let handle = open_func(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if handle == 0 {
          return Err(format!("Failed to open process: {}", pid).into());
        }

        let get_name_func = psapi_lib.get::<unsafe extern "system" fn(u32, *mut u32, u32) -> u32>(b"GetProcessImageFileNameA\0").unwrap();
        let mut name_buf: [u8; 2024] = [0; 2024];
        let ret = get_name_func(handle, name_buf.as_mut_ptr() as *mut u32, 2024);

        let name: String = if ret > 0 {
          let name_str = String::from_utf8_lossy(&name_buf[..ret as usize]);
          let exec_name = path::Path::new(name_str.trim());
          exec_name.file_name().unwrap().to_str().unwrap().to_string()
        } else {
          String::new()
        };

        Ok(Self {
          pid,
          name: name,
          handle: handle
        })
      }
    }

    pub fn close(&mut self, kernel32_lib: &libloading::Library) {
      if self.handle == 0 {
        return;
      }
      unsafe {
        let close_func = kernel32_lib.get::<unsafe extern "system" fn(u32) -> u32>(b"CloseHandle\0").unwrap();
        close_func(self.handle);
        self.handle = 0;
      }
    }

    pub fn inject(&mut self, kernel32_lib: &libloading::Library) -> Result<(), Box<dyn std::error::Error>> {
      if self.handle != 0 {
        return Ok(());
      }

      unsafe {
        let open_func = kernel32_lib.get::<unsafe extern "system" fn(u32, bool, u32) -> u32>(b"OpenProcess\0").unwrap();
        let handle = open_func(
          PROCESS_QUERY_INFORMATION|PROCESS_VM_READ|PROCESS_VM_OPERATION|PROCESS_VM_WRITE,
          false,
          self.pid
        );
        if handle == 0 {
          return Err(format!("Failed to open process: {}", self.pid).into());
        }

        self.handle = handle;

        Ok(())
      }
    }
}


#[derive(Debug, Default)]
pub struct ProcessManager {
}

impl ProcessManager {
  pub fn get_process_list(&self, psapi_lib: &libloading::Library, kernel32_lib: &libloading::Library) -> Vec<ProcessItem>{
    let mut list: Vec<ProcessItem> = vec![];
    unsafe {
      let func = psapi_lib.get::<unsafe extern "system" fn(*mut u32, u32, *mut u32) -> u32>(b"EnumProcesses\0").unwrap();
      let mut processes: [u32; 1024] = [0; 1024];
      let mut bytes_returned: u32 = 0;
      let ret: u32 = func(processes.as_mut_ptr(), 1024 * DWORD_SIZE, &mut bytes_returned as *mut u32);
      if ret != 1 {
        return list;
      }

      let num_processes = bytes_returned / DWORD_SIZE;
      for i in 0..num_processes as usize {
        let pid = processes[i];
        if pid == 0 {
          continue;
        }
        let p = ProcessItem::from_pid(pid, psapi_lib, kernel32_lib);
        if p.is_err() {
          continue;
        }
        let mut p = p.unwrap();
        p.close(kernel32_lib);

        list.push(p);
      }
    };

    return list;
  }
}