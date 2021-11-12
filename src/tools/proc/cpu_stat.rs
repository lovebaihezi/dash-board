use std::{str::Split, u64};

use serde::{Deserialize, Serialize};

/// The amount of time, measured in units of USER_HZ
/// (1/100ths of a second on most architectures, use
/// sysconf(_SC_CLK_TCK) to obtain the right value)
pub type UnixTime = u64;
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CoreStat {
    // (1) Time spent in user mode.
    user: Option<UnixTime>,
    // (2) Time spent in user mode with low priority (nice)
    nice: Option<UnixTime>,
    // (3) Time spent in system mode.
    system: Option<UnixTime>,
    /**
        (4) Time spent in the idle task.  This value
        should be USER_HZ times the second entry in
        the /proc/uptime pseudo-file.
    */
    idle_task: Option<UnixTime>,
    /**
        **(since Linux 2.5.41)**
        (5) Time waiting for I/O to complete.  This
        value is not reliable, for the following
        reasons:
            1. The CPU will not wait for I/O to
        complete; io wait is the time that a task
        is waiting for I/O to complete.  When a
        CPU goes into idle state for outstanding
        task I/O, another task will be scheduled
        on this CPU.
            2. On a multi-core CPU, the task waiting for
        I/O to complete is not running on any
        CPU, so the io wait of each CPU is
        difficult to calculate.
            3. The value in this field may decrease in
        certain conditions.
    */
    io_wait_unreliable: Option<UnixTime>,
    /**
        (since Linux 2.6.0)
        (6) Time servicing interrupts.
    */
    interrupt: Option<UnixTime>,
    /**
        **(since Linux 2.6.0)**
        (7) Time servicing softirqs.
    */
    soft_interrupt: Option<UnixTime>,
    /**
        **(since Linux 2.6.11)**
        (8) Stolen time, which is the time spent in
        other operating systems when running in a
        virtualized environment
    */
    virtual_environment_stolen_time: Option<UnixTime>,
    /**
        **(since Linux 2.6.24)**
        (9) Time spent running a virtual CPU for
        guest operating systems under the control of
        the Linux kernel.
    */
    virtual_cpu: Option<UnixTime>,
    /**
        **(since Linux 2.6.33)**
        (10) Time spent running a niced guest
        (virtual CPU for guest operating systems
        under the control of the Linux kernel).
    */
    virtual_cpu_nice: Option<UnixTime>,
}

type DiskIo = ((u64, u64), (u64, u64, u64, u64, u64));

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CpuStat {
    system_cpu: CoreStat,
    specific_cpu: Vec<CoreStat>,
    /*
       The number of pages the system paged in and the
       number that were paged out (from disk).
    */
    page: Option<(u64, u64)>,
    /*
       The number of swap pages that have been brought in
       and out.
    */
    swap: Option<(u64, u64)>,
    /**
        (major,disk_idx):(noinfo, read_io_ops, blks_read, write_io_ops, blks_written)
        **(Linux 2.4 only)**
    */
    disk_io: Option<Vec<DiskIo>>,
    /*
       This line shows counts of interrupts serviced since
       boot time, for each of the possible system
       interrupts.  The first column is the total of all
       interrupts serviced including unnumbered
       architecture specific interrupts; each subsequent
       column is the total for that particular numbered
       interrupt.  Unnumbered interrupts are not shown,
       only summed into the total.
    */
    interrupt: Option<Vec<u64>>,
    // The number of context switches that the system
    //    underwent.
    context_switch: Option<u64>,
    /**
       boot time, in seconds since the Epoch, 1970-01-01
       00:00:00 +0000 (UTC).
    */
    boot_time: Option<u64>,
    // Number of forks since boot.
    process: Option<u64>,
    /***
        Number of processes in runnable state.
        (Linux 2.5.45 onward.)
    */
    process_running: Option<u64>,
    /**
       Number of processes blocked waiting for I/O to
       complete.  (Linux 2.5.45 onward.)
    */
    process_blocked: Option<u64>,
    /**
       This line shows the number of softirq for all CPUs.
       The first column is the total of all softirqs and
       each subsequent column is the total for particular
       softirq.  (Linux 2.6.31 onward.)
    */
    software_interrupt_request: Option<Vec<u64>>,
}

#[inline]
fn take_first(source: &str) -> Option<(u64, u64)> {
    let mut major_disk_idk = source.split(",").map(|s| s.parse::<u64>().unwrap());
    let major = major_disk_idk.next()?;
    let disk_idk = major_disk_idk.next()?;
    Some((major, disk_idk))
}

#[inline]
fn take_last(source: &str) -> Option<(u64, u64, u64, u64, u64)> {
    let mut fives = source.split(",").map(|s| s.parse::<u64>().unwrap());
    let no_info = fives.next()?;
    let read_io_ops = fives.next()?;
    let blks_read = fives.next()?;
    let write_io_ops = fives.next()?;
    let blks_written = fives.next()?;
    Some((no_info, read_io_ops, blks_read, write_io_ops, blks_written))
}
#[inline]
fn take_disk_io(source: &str) -> Option<DiskIo> {
    let mut first_last = source
        .split(":")
        .map(|s| s.trim_matches(|c| c == '(' || c == ')'));
    let first = take_first(first_last.next()?)?;
    let second = take_last(first_last.next()?)?;
    Some((first, second))
}

fn fill_field(mut stat: &mut CoreStat, s: Split<&str>) {
    let mut i = s
        .map(|s| s.trim().parse::<u64>())
        .filter(|s| s.is_ok())
        .map(|n| n.unwrap());
    stat.user = i.next();
    stat.nice = i.next();
    stat.system = i.next();
    stat.idle_task = i.next();
    stat.io_wait_unreliable = i.next();
    stat.interrupt = i.next();
    stat.soft_interrupt = i.next();
    stat.virtual_environment_stolen_time = i.next();
    stat.virtual_cpu = i.next();
    stat.virtual_cpu_nice = i.next();
}

fn convert(source: String) -> std::io::Result<CpuStat> {
    let mut result = CpuStat::default();
    let name_splits = source.split("\n").filter(|s| !s.is_empty()).map(
        |s| -> (Option<&str>, std::str::Split<&str>) {
            let mut splits = s.split(" ");
            (splits.next(), splits)
        },
    );
    for (name_or_none, mut tail) in name_splits {
        if let Some(name) = name_or_none {
            if name.starts_with("cpu") {
                if name == "cpu" {
                    fill_field(&mut result.system_cpu, tail);
                } else {
                    let mut x = CoreStat::default();
                    fill_field(&mut x, tail);
                    result.specific_cpu.push(x);
                }
            } else {
                match name {
                    "page" => {
                        result.page = Some((
                            tail.next().unwrap().parse::<u64>().unwrap(),
                            tail.next().unwrap().parse::<u64>().unwrap(),
                        ))
                    }
                    "swap" => {
                        result.swap = Some((
                            tail.next().unwrap().parse::<u64>().unwrap(),
                            tail.next().unwrap().parse::<u64>().unwrap(),
                        ))
                    }
                    // "intr" => {
                    //     result.interrupt = Some(
                    //         tail.map(move |s| s.parse::<u64>().unwrap())
                    //             .collect::<Vec<u64>>(),
                    //     )
                    // }
                    "disk_io" => {
                        result.disk_io = {
                            Some({
                                tail.map(take_disk_io)
                                    .filter(|s| s.is_some())
                                    .map(|s| s.unwrap())
                                    .collect::<Vec<DiskIo>>()
                            })
                        }
                    }
                    "ctxt" => {
                        result.context_switch = Some(tail.next().unwrap().parse::<u64>().unwrap())
                    }
                    "btime" => {
                        result.boot_time = Some(tail.next().unwrap().parse::<u64>().unwrap())
                    }
                    "processes" => {
                        result.process = Some(tail.next().unwrap().parse::<u64>().unwrap())
                    }
                    "procs_running" => {
                        result.process_running = Some(tail.next().unwrap().parse::<u64>().unwrap())
                    }
                    "procs_blocked" => {
                        result.process_blocked = Some(tail.next().unwrap().parse::<u64>().unwrap())
                    }
                    "softirq" => {
                        result.software_interrupt_request = Some(
                            tail.map(|s| s.trim().parse::<u64>())
                                .filter(|x| x.is_ok())
                                .map(|x| x.unwrap())
                                .collect::<Vec<u64>>(),
                        )
                    }
                    _ => {
                        // println!("unknown {}", name);
                    }
                }
            }
        }
    }
    Ok(result)
}

pub fn cpu_stat() -> std::io::Result<CpuStat> {
    let cpu_stat_file_content = std::fs::read_to_string("/proc/stat")?;
    let result = convert(cpu_stat_file_content)?;
    Ok(result)
}

#[test]
fn cpu_stat_test() {
    let source = r#"cpu  5084991 2795 1085705 40383952 18125 174103 73464 0 0 0
cpu0 438729 275 89317 3354427 1473 9555 6997 0 0 0
cpu1 435747 232 91054 3354235 1435 10797 8436 0 0 0
cpu2 445009 244 89437 3353894 1561 8931 5046 0 0 0
cpu3 448033 258 90341 3350264 1354 9009 5547 0 0 0
cpu4 487699 191 92730 3307656 1378 9091 4582 0 0 0
cpu5 379496 298 107676 3378028 1594 30345 4125 0 0 0
cpu6 396344 237 81178 3407082 1542 9724 5042 0 0 0
cpu7 418367 197 83983 3387487 1613 8686 3721 0 0 0
cpu8 437704 227 85564 3367230 1629 8479 3720 0 0 0
cpu9 436287 218 87243 3366701 1538 8757 4287 0 0 0
cpu10 313255 229 97422 3405689 1478 52090 18157 0 0 0
cpu11 448315 183 89754 3351254 1522 8632 3798 0 0 0
intr 197172306 0 473 0 0 0 0 0 0 0 2330 0 0 85 0 41 0 2138 415 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 27 20 72 245 97 92 149 62 22 20 20318 17901 22430 23707 18611 28004 23351 23596 17580 16194 17612 22740 7 1476098 0 0 41 0 282196 60881 82528 103504 165567 209 39 30393529 961 3188681 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
ctxt 562378222
btime 1623002973
processes 45099
procs_running 4
procs_blocked 0
softirq 159081905 47839458 11367052 19 849814 8102 0 749747 59052904 17481 39197328"#;
    assert!(convert(source.to_string()).is_ok());
}
