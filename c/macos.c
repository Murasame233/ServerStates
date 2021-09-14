#include <mach/mach_init.h>
#include <mach/mach_error.h>
#include <mach/mach_host.h>
#include <mach/vm_map.h>

// static unsigned long long _previousTotalTicks = 0;
// static unsigned long long _previousIdleTicks = 0;

// Returns 1.0f for "CPU fully pinned", 0.0f for "CPU idle", or somewhere in between
// You'll need to call this at regular intervals, since it measures the load between
// the previous call and the current one.
// float GetCPULoad()
// {
//    host_cpu_load_info_data_t cpuinfo;
//    mach_msg_type_number_t count = HOST_CPU_LOAD_INFO_COUNT;
//    if (host_statistics(mach_host_self(), HOST_CPU_LOAD_INFO, (host_info_t)&cpuinfo, &count) == KERN_SUCCESS)
//    {
//       unsigned long long totalTicks = 0;
//       for(int i=0; i<CPU_STATE_MAX; i++) totalTicks += cpuinfo.cpu_ticks[i];
//       return CalculateCPULoad(cpuinfo.cpu_ticks[CPU_STATE_IDLE], totalTicks);
//    }
//    else return -1.0f;
// }

// float CalculateCPULoad(unsigned long long idleTicks, unsigned long long totalTicks)
// {
//    unsigned long long totalTicksSinceLastTime = totalTicks-_previousTotalTicks;
//    unsigned long long idleTicksSinceLastTime  = idleTicks-_previousIdleTicks;
//    float ret = 1.0f-((totalTicksSinceLastTime > 0) ? ((float)idleTicksSinceLastTime)/totalTicksSinceLastTime : 0);
//    _previousTotalTicks = totalTicks;
//    _previousIdleTicks  = idleTicks;
//    return ret;
// }

// static unsigned allocateCPULoadInfo(processor_cpu_load_info_t* p) {
//    mach_msg_type_number_t info_size = sizeof(processor_cpu_load_info_t);
//    unsigned cpu_count;

//    // Use host_processor_info
//    if (0 != host_processor_info(mach_host_self(), PROCESSOR_CPU_LOAD_INFO, &cpu_count, (processor_info_array_t*)p, &info_size)) {
//    }

//    return cpu_count;
// }

// unsigned int get_count(){
//    host_cpu_load_info_data_t;
//    processor_cpu_load_info_t p;
//    allocateCPULoadInfo(&p);
//    return p[1].cpu_ticks[CPU_STATE_IDLE];
//    // return allocateCPULoadInfo(&p);
// }
// host_processor_info()