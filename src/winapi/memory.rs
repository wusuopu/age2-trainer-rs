use libloading;
use std::ffi::c_void;
use std::io;
use std::mem::{size_of, MaybeUninit};


/*
  通过已经加载了的 windows kernel32.dll lib，调用 ReadProcessMemory 方法读取指定进程内存地址的数据
*/
pub fn read_memory<T>(lib: &libloading::Library, process_handle: u32, addr: u32, size: u32) -> Result<T, Box<dyn std::error::Error>> {
  unsafe {
    type ReadProcessMemoryFn = unsafe extern "system" fn(
      u32,              // handle
      u32,              // address
      *mut u32,         // buffer
      usize,            // buffer size
      *mut usize,       // bytes read
    ) -> i32;

    let read_size = size as usize;
    let expected_size = size_of::<T>();
    if expected_size == 0 {
      return Err("read_memory requires a non zero-sized type".into());
    }
    if read_size != expected_size {
      return Err(format!("size mismatch: expected {} bytes, got {}", expected_size, read_size).into());
    }

    let read_process_memory = lib.get::<ReadProcessMemoryFn>(b"ReadProcessMemory\0")?;

    let mut result = MaybeUninit::<T>::uninit();
    let mut bytes_read: usize = 0;
    let success = read_process_memory(
      process_handle,
      addr,
      result.as_mut_ptr() as *mut u32,
      expected_size,
      &mut bytes_read as *mut usize,
    );

    if success == 0 {
      return Err(io::Error::last_os_error().into());
    }
    if bytes_read != expected_size {
      return Err(format!("ReadProcessMemory copied {} bytes, expected {}", bytes_read, expected_size).into());
    }

    Ok(result.assume_init())
  }
}

/*
  通过已经加载了的 windows kernel32.dll lib，调用 WriteProcessMemory 方法写入指定进程内存地址的数据
*/
pub fn write_memory<T>(lib: libloading::Library, process_handle: u32, addr: u32, data: &T, size: u32) -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
    type WriteProcessMemoryFn = unsafe extern "system" fn(
      u32,              // process handle
      u32,              // address
      u32,              // buffer
      usize,            // buffer size
      *mut usize,       // bytes written
    ) -> i32;

    let write_size = size as usize;
    let expected_size = size_of::<T>();
    if expected_size == 0 {
      return Err("write_memory requires a non zero-sized type".into());
    }
    if write_size != expected_size {
      return Err(format!("size mismatch: expected {} bytes, got {}", expected_size, write_size).into());
    }

    let write_process_memory = lib.get::<WriteProcessMemoryFn>(b"WriteProcessMemory\0")?;

    // let data_buf = MaybeUninit::new(data);
    let mut bytes_written: usize = 0;
    let success = write_process_memory(
      process_handle,
      addr,
      // data_buf.as_ptr() as u32,
      &*(data as *const T) as *const T as u32,
      expected_size,
      &mut bytes_written as *mut usize,
    );

    if success == 0 {
      return Err(io::Error::last_os_error().into());
    }
    if bytes_written != expected_size {
      return Err(format!("WriteProcessMemory wrote {} bytes, expected {}", bytes_written, expected_size).into());
    }

    Ok(())
  }
}
