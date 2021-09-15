use std::{mem::size_of, slice};

use libc::{CPU_STATE_IDLE, PROCESSOR_CPU_LOAD_INFO, mach_host_self, mach_msg_type_number_t, mach_port_t, natural_t, processor_cpu_load_info, processor_cpu_load_info_t, processor_flavor_t, processor_info_array_t};

extern "C" {
    fn host_processor_info(
        host: mach_port_t,
        flavor: processor_flavor_t,
        out_processor_count: &natural_t,
        out_processor_info: &processor_info_array_t,
        out_processor_infoCnt: &mach_msg_type_number_t,
    ) -> i32;
}
fn main() {
    let _cpuinfo: processor_cpu_load_info_t;
    let info_size: mach_msg_type_number_t = size_of::<processor_cpu_load_info_t>() as u32;
    let cpu_count = 0;
    let ptr = std::ptr::null_mut();
    unsafe{
        host_processor_info(mach_host_self(),PROCESSOR_CPU_LOAD_INFO,&cpu_count,&ptr,&info_size);
    }
    println!("{}",cpu_count);
    let cpu_load:*mut processor_cpu_load_info = ptr.cast();
    let slice:&[processor_cpu_load_info] = unsafe {slice::from_raw_parts(cpu_load, cpu_count as usize)};
    println!("{}",&slice[2].cpu_ticks[CPU_STATE_IDLE as usize]);
}
