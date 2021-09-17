use std::{mem::size_of, slice};

use libc::{
    mach_host_self, mach_msg_type_number_t, mach_port_t, natural_t, processor_cpu_load_info,
    processor_cpu_load_info_t, processor_flavor_t, processor_info_array_t, CPU_STATE_IDLE,
    CPU_STATE_NICE, CPU_STATE_SYSTEM, CPU_STATE_USER, PROCESSOR_CPU_LOAD_INFO,
};

use super::Platform;

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

pub struct MacOS {
    info_size: mach_msg_type_number_t,
    cpu_count: u32,
    ptr: *mut i32,
    host: mach_port_t,
    pub core_status: Vec<i32>,
    prev_vec: Vec<Prev>,
}

impl MacOS {
    fn new() -> Self {
        let info_size: mach_msg_type_number_t = size_of::<processor_cpu_load_info_t>() as u32;
        let cpu_count = 0;
        let ptr = std::ptr::null_mut();
        let host = unsafe { mach_host_self() };
        let mut core_status: Vec<i32> = Vec::with_capacity(0);
        let mut prev_vec: Vec<Prev> = Vec::with_capacity(0);
        unsafe {
            host_processor_info(host, PROCESSOR_CPU_LOAD_INFO, &cpu_count, &ptr, &info_size);
        }
        if core_status.capacity() != cpu_count as usize {
            core_status.resize(cpu_count as usize, 0);
            prev_vec.resize(cpu_count as usize, Prev { total: 0, idle: 0 })
        }
        let cpu_load: *mut processor_cpu_load_info = ptr.cast();
        let slice: &[processor_cpu_load_info] =
            unsafe { slice::from_raw_parts(cpu_load, cpu_count as usize) };
        for core in 0..cpu_count as usize {
            let ticks = &(slice[core].cpu_ticks);
            let prev = &mut prev_vec[core];
            let total = ticks[CPU_STATE_USER as usize]
                + ticks[CPU_STATE_SYSTEM as usize]
                + ticks[CPU_STATE_NICE as usize]
                + ticks[CPU_STATE_IDLE as usize];
            prev.total = total;
            prev.idle = ticks[CPU_STATE_IDLE as usize];
        }
        Self {
            info_size,
            cpu_count,
            ptr,
            host,
            prev_vec,
            core_status,
        }
    }
    fn _update(&mut self) {
        unsafe {
            host_processor_info(
                self.host,
                PROCESSOR_CPU_LOAD_INFO,
                &(self.cpu_count),
                &(self.ptr),
                &(self.info_size),
            );
        }
        let cpu_count = self.cpu_count;
        if self.core_status.capacity() != cpu_count as usize {
            self.core_status.resize(cpu_count as usize, 0);
            self.prev_vec
                .resize(cpu_count as usize, Prev { total: 0, idle: 0 })
        }
        let cpu_load: *mut processor_cpu_load_info = self.ptr.cast();
        let slice: &[processor_cpu_load_info] =
            unsafe { slice::from_raw_parts(cpu_load, cpu_count as usize) };
        for core in 0..cpu_count as usize {
            self.core_status[core] = {
                let ticks = &(slice[core].cpu_ticks);
                let prev = &mut self.prev_vec[core];
                let total = ticks[CPU_STATE_USER as usize]
                    + ticks[CPU_STATE_SYSTEM as usize]
                    + ticks[CPU_STATE_NICE as usize]
                    + ticks[CPU_STATE_IDLE as usize];
                let new_idle = ticks[CPU_STATE_IDLE as usize] - prev.idle;
                let new_total = total - prev.total;
                let re = ((1f32 - new_idle as f32 / new_total as f32) * 100f32) as i32;
                prev.total = total;
                prev.idle = ticks[CPU_STATE_IDLE as usize];
                re
            }
        }
    }
}

impl Platform for MacOS {
    fn init() -> Self {
        Self::new()
    }

    fn update(&mut self) {
        self._update();
    }
    fn get_all(&self) -> &'_ [i32] {
        &(self.core_status)
    }

    fn get(&self, core: usize) -> i32 {
        self.core_status[core]
    }
}
