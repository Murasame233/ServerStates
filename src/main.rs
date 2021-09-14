use std::mem::size_of;

use libc::{PROCESSOR_CPU_LOAD_INFO, kern_return_t, mach_host_self, mach_msg_type_number_t, mach_port_t, natural_t, processor_cpu_load_info_t, processor_flavor_t, processor_info_array_t};

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
    let mut cpuinfo: processor_cpu_load_info_t;
    let mut info_size: mach_msg_type_number_t = size_of::<processor_cpu_load_info_t>() as u32;
    let mut cpu_count: usize;
    unsafe{
        host_processor_info(mach_host_self(),PROCESSOR_CPU_LOAD_INFO,cpu_count,cpuinfo,&info_size);
    }
    cpuinfo[1]
}
