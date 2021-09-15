use std::{io::Write, mem::size_of, slice, thread::sleep, time::Duration};

use libc::{
    mach_host_self, mach_msg_type_number_t, mach_port_t, natural_t, processor_cpu_load_info,
    processor_cpu_load_info_t, processor_flavor_t, processor_info_array_t, CPU_STATE_IDLE,
    CPU_STATE_NICE, CPU_STATE_SYSTEM, CPU_STATE_USER, PROCESSOR_CPU_LOAD_INFO,
};

extern "C" {
    fn host_processor_info(
        host: mach_port_t,
        flavor: processor_flavor_t,
        out_processor_count: &natural_t,
        out_processor_info: &processor_info_array_t,
        out_processor_infoCnt: &mach_msg_type_number_t,
    ) -> i32;
}

#[derive(Clone)]
struct Prev {
    total: u32,
    idle: u32,
}
fn main() {
    let info_size: mach_msg_type_number_t = size_of::<processor_cpu_load_info_t>() as u32;
    let cpu_count = 0;
    let ptr = std::ptr::null_mut();
    let host = unsafe { mach_host_self() };
    let mut vec: Vec<f32> = Vec::with_capacity(0);
    let mut prev_vec: Vec<Prev> = Vec::with_capacity(0);
    loop {
        unsafe {
            host_processor_info(host, PROCESSOR_CPU_LOAD_INFO, &cpu_count, &ptr, &info_size);
        }
        if vec.capacity() != cpu_count as usize {
            vec.resize(cpu_count as usize, 0.0);
            prev_vec.resize(cpu_count as usize, Prev { total: 0, idle: 0 })
        }
        let cpu_load: *mut processor_cpu_load_info = ptr.cast();
        let slice: &[processor_cpu_load_info] =
            unsafe { slice::from_raw_parts(cpu_load, cpu_count as usize) };
        for core in 0..cpu_count as usize {
            vec[core] = {
                let ticks = &(slice[core].cpu_ticks);
                let prev = &mut prev_vec[core];
                let total = ticks[CPU_STATE_USER as usize]
                    + ticks[CPU_STATE_SYSTEM as usize]
                    + ticks[CPU_STATE_NICE as usize]
                    + ticks[CPU_STATE_IDLE as usize];
                let re = 1f32
                    - (ticks[CPU_STATE_IDLE as usize] - prev.idle) as f32
                        / (total - prev.total) as f32;
                prev.total = total;
                prev.idle = ticks[CPU_STATE_IDLE as usize];
                re
            }
        }
        print!("\r");
        for p in &vec {
            print!("{}% ", (p * 100f32) as i32);
        }
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
