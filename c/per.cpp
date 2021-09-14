#include <stdio.h>
#include <unistd.h>
#include <mach/mach_host.h>
#include <mach/processor_info.h>
#include <iostream>

using namespace std;

class CpuUsage
{
public:
    CpuUsage(int core) : core_(core)
    {
        prev = updated_ticks_(core);
    }

    float get()
    {
        Ticks t = updated_ticks_(core_);
        unsigned long long int used = t.used() - prev.used();
        unsigned long long int total = t.total() - prev.total();
        prev = t;
        return (float)used / (float)total * 100.0f;
    }

private:
    struct Ticks
    {
        unsigned long long int usertime;
        unsigned long long int nicetime;
        unsigned long long int systemtime;
        unsigned long long int idletime;

        unsigned long long int used() { return usertime + nicetime + systemtime; }
        unsigned long long int total() { return usertime + nicetime + systemtime + idletime; }
    } prev;

    int core_;

    Ticks updated_ticks_(int core)
    {
        unsigned int cpu_count;
        processor_cpu_load_info_t cpu_load;
        mach_msg_type_number_t cpu_msg_count;

        int rc = host_processor_info(
            mach_host_self(),
            PROCESSOR_CPU_LOAD_INFO,
            &cpu_count,
            (processor_info_array_t *)&cpu_load,
            &cpu_msg_count);
        if (rc != 0)
        {
            printf("Error: failed to scan processor info (rc=%d)\n", rc);
            exit(1);
        }

        if (core < 0 || cpu_count <= core)
        {
            printf("Error: invalid core number: %d\n", core);
            exit(1);
        }
        unsigned long long int usertime = cpu_load[core].cpu_ticks[CPU_STATE_USER];
        unsigned long long int nicetime = cpu_load[core].cpu_ticks[CPU_STATE_NICE];
        unsigned long long int systemtime = cpu_load[core].cpu_ticks[CPU_STATE_SYSTEM];
        unsigned long long int idletime = cpu_load[core].cpu_ticks[CPU_STATE_IDLE];

        Ticks t = {usertime, nicetime, systemtime, idletime};
        return t;
    }
};

int main()
{
    CpuUsage a(0), b(1), c(2), d(3);
    sleep(1);
    printf("%6.2f, %6.2f, %6.2f, %6.2f\n", a.get(), b.get(), c.get(), d.get());
    sleep(1);
    printf("%6.2f, %6.2f, %6.2f, %6.2f\n", a.get(), b.get(), c.get(), d.get());
    sleep(1);
    printf("%6.2f, %6.2f, %6.2f, %6.2f\n", a.get(), b.get(), c.get(), d.get());
    return 0;
}