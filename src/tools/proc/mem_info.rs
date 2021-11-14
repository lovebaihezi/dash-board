use serde::{Deserialize, Serialize};

use crate::tools::{
    logger::log::{log, LogLevel},
    BitType,
};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[allow(dead_code)]
pub struct MemoInfo {
    /*
       Total usable RAM (i.e., physical RAM minus a few
       reserved bits and the kernel binary code).
    */
    mem_total: Option<BitType>,
    //The sum of LowFree+HighFree.
    mem_free: Option<BitType>,
    /*
        An estimate of how much memory is available for
        starting new applications, without swapping.
    */
    mem_available: Option<BitType>,
    /*
       Relatively temporary storage for raw disk blocks
       that shouldn't get tremendously large (20 MB or so).
    */
    buffers: Option<BitType>,
    /*
        In-memory cache for files read from the disk (the
        page cache).  Doesn't include SwapCached.
    */
    cached: Option<BitType>,
    /*
       Memory that once was swapped out, is swapped back
       in but still also is in the swap file.  (If memory
       pressure is high, these pages don't need to be
       swapped out again because they are already in the
       swap file.  This saves I/O.)
    */
    swap_cached: Option<BitType>,
    /*
       Memory that has been used more recently and usually
       not reclaimed unless absolutely necessary.
    */
    active: Option<BitType>,
    /*
        Memory which has been less recently used.  It is
        more eligible to be reclaimed for other purposes.
    */
    inactive: Option<BitType>,
    /*
       [To Be Documented]
    */
    active_anon: Option<BitType>,
    /*
       [To Be Documented]
    */
    inactive_anon: Option<BitType>,
    /*
       [To Be Documented]
    */
    active_file: Option<BitType>,
    /*
       [To Be Documented]
    */
    inactive_file: Option<BitType>,
    /*
       (From Linux 2.6.28 to 2.6.30,
       CONFIG_UNEVICTABLE_LRU was required.)  [To be
       documented.]
    */
    unevictable: Option<BitType>,
    /*
       (From Linux 2.6.28 to 2.6.30,
       CONFIG_UNEVICTABLE_LRU was required.)  [To be
       documented.]
    */
    mlocked: Option<BitType>,
    /*
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Total amount of highmem.  Highmem is
       all memory above ~860 MB of physical memory.
       Highmem areas are for use by user-space programs,
       or for the page cache.  The kernel must use tricks
       to access this memory, making it slower to access
       than lowmem.
    */
    high_total: Option<BitType>,
    /*
        (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
        required.)  Amount of free highmem.
    */
    high_free: Option<BitType>,
    /*
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Total amount of lowmem.  Lowmem is
       memory which can be used for everything that
       highmem can be used for, but it is also available
       for the kernel's use for its own data structures.
       Among many other things, it is where everything
       from Slab is allocated.  Bad things happen when
       you're out of lowmem.
    */
    low_total: Option<BitType>,
    /*
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Amount of free lowmem.
    */
    low_free: Option<BitType>,
    /*
       (CONFIG_MMU is required.)  [To be documented.]
    */
    mmap_copy: Option<BitType>,
    /*
       Total amount of swap space available.
    */
    swap_total: Option<BitType>,
    /*
       Amount of swap space that is currently unused.
    */
    swap_free: Option<BitType>,
    /*
       Memory which is waiting to get written back to the
       disk.
    */
    dirty: Option<BitType>,
    /*
       Memory which is actively being written back to the
       disk.
    */
    write_back: Option<BitType>,
    /*
       (since Linux 2.6.18)
       Non-file backed pages mapped into user-space page
       tables.
    */
    anon_pages: Option<BitType>,
    /*
       Files which have been mapped into memory (with
       mmap(2)), such as libraries.
    */
    mapped: Option<BitType>,
    /*
       (since Linux 2.6.32)
       Amount of memory consumed in tmpfs(5) filesystems.
    */
    shmem: Option<BitType>,
    /*
       (since Linux 4.20)
       Kernel allocations that the kernel will attempt to
       reclaim under memory pressure.  Includes
       SReclaimable (below), and other direct allocations
       with a shrinker.
    */
    k_reclaimable: Option<BitType>,
    /*
       In-kernel data structures cache.  (See
       slabinfo(5).)
    */
    slab: Option<BitType>,
    /*
       (since Linux 2.6.19)
       Part of Slab, that might be reclaimed, such as
       caches.
    */
    s_reclaimable: Option<BitType>,
    /*
       (since Linux 2.6.19)
       Part of Slab, that cannot be reclaimed on memory
       pressure.
    */
    s_unreclaim: Option<BitType>,
    /*
       (since Linux 2.6.32)
       Amount of memory allocated to kernel stacks.
    */
    kernel_stack: Option<BitType>,
    /*
       (since Linux 2.6.18)
       Amount of memory dedicated to the lowest level of
       page tables.
    */
    page_tables: Option<BitType>,
    /*
       (since Linux 2.6.27)
       (CONFIG_QUICKLIST is required.)  [To be
       documented.]
    */
    quicklists: Option<BitType>,
    /*
        (since Linux 2.6.18)
        NFS pages sent to the server, but not yet committed
        to stable storage.
    */
    n_f_s_unstable: Option<BitType>,
    /*
       (since Linux 2.6.18)
       Memory used for block device "bounce buffers".
    */
    bounce: Option<BitType>,
    /*
       (since Linux 2.6.26)
       Memory used by FUSE for temporary writeback
       buffers.
    */
    writeback_tmp: Option<BitType>,
    /*
        (since Linux 2.6.10)
        This is the total amount of memory currently
        available to be allocated on the system, expressed
        in kilobytes.  This limit is adhered to only if
        strict overcommit accounting is enabled (mode 2 in
        /proc/sys/vm/overcommit_memory).  The limit is
        calculated according to the formula described under
        /proc/sys/vm/overcommit_memory.  For further
        details, see the kernel source file
        Documentation/vm/overcommit-accounting.rst.
    */
    commit_limit: Option<BitType>,
    /*
           The amount of memory presently allocated on the
       system.  The committed memory is a sum of all of
       the memory which has been allocated by processes,
       even if it has not been "used" by them as of yet.
       A process which allocates 1 GB of memory (using
       malloc(3) or similar), but touches only 300 MB of
       that memory will show up as using only 300 MB of
       memory even if it has the address space allocated
       for the entire 1 GB.
       This 1 GB is memory which has been "committed" to
       by the VM and can be used at any time by the
       allocating application.  With strict overcommit
       enabled on the system (mode 2 in
       /proc/sys/vm/overcommit_memory), allocations which
       would exceed the CommitLimit will not be permitted.
       This is useful if one needs to guarantee that
       processes will not fail due to lack of memory once
       that memory has been successfully allocated.
    */
    committed_a_s: Option<BitType>,
    //Total size of vmalloc memory area.
    vmalloc_total: Option<BitType>,
    /*
       Amount of vmalloc area which is used.  Since Linux
       4.4, this field is no longer calculated, and is
       hard coded as 0.  See /proc/vmallocinfo.
    */
    vmalloc_used: Option<BitType>,
    /*
        Largest contiguous block of vmalloc area which is
        free.  Since Linux 4.4, this field is no longer
        calculated and is hard coded as 0.  See
        /proc/vmallocinfo.
    */
    vmalloc_chunk: Option<BitType>,
    /*
       unknown
    */
    percpu: Option<BitType>,
    /*
       (since Linux 2.6.32)
       (CONFIG_MEMORY_FAILURE is required.)  [To be
       documented.]
    */
    hardware_corrupted: Option<BitType>,
    // Shows the amount of memory marked by madvise(2)
    //   MADV_FREE.
    lazy_free: Option<BitType>,
    /*
       (since Linux 4.12)
       Shows the amount of memory marked by madvise(2)
       MADV_FREE.
    */
    anon_huge_pages: Option<BitType>,
    /*
       (since Linux 2.6.38)
       (CONFIG_TRANSPARENT_HUGEPAGE is required.)  Non-
       file backed huge pages mapped into user-space page
       tables.
    */
    shmem_huge_pages: Option<BitType>,
    /*
       (since Linux 4.8)
       (CONFIG_TRANSPARENT_HUGEPAGE is required.)  Shared
       memory mapped into user space with huge pages.
    */
    shmem_pmd_mapped: Option<BitType>,
    /*
     */
    file_huge_pages: Option<BitType>,
    /*
     */
    file_pmd_mapped: Option<BitType>,
    /*
       (since Linux 3.1)
       Total CMA (Contiguous Memory Allocator) pages.
       (CONFIG_CMA is required.)
    */
    cma_total: Option<BitType>,
    /*
       (since Linux 3.1)
       Free CMA (Contiguous Memory Allocator) pages.
       (CONFIG_CMA is required.)
    */
    cma_free: Option<BitType>,
    /*
       (CONFIG_HUGETLB_PAGE is required.)  The size of the
       pool of huge pages.
    */
    huge_pages_total: Option<BitType>,
    /*
        (CONFIG_HUGETLB_PAGE is required.)  The number of
        huge pages in the pool that are not yet allocated.
    */
    huge_pages_free: Option<BitType>,
    /*
       (since Linux 2.6.17)
       (CONFIG_HUGETLB_PAGE is required.)  This is the
       number of huge pages for which a commitment to
       allocate from the pool has been made, but no
       allocation has yet been made.  These reserved huge
       pages guarantee that an application will be able to
       allocate a huge page from the pool of huge pages at
       fault time.
    */
    huge_pages_rsvd: Option<BitType>,
    /*
       (since Linux 2.6.24)
       (CONFIG_HUGETLB_PAGE is required.)  This is the
       number of huge pages in the pool above the value in
       /proc/sys/vm/nr_hugepages.  The maximum number of
       surplus huge pages is controlled by
       /proc/sys/vm/nr_overcommit_hugepages.
    */
    huge_pages_surp: Option<BitType>,
    /*
       (CONFIG_HUGETLB_PAGE is required.)  The size of
       huge pages.
    */
    huge_page_size: Option<BitType>,
    /*
       (since Linux 2.6.27)
       Number of bytes of RAM linearly mapped by kernel in
       4 kB pages.  (x86.)
    */
    hugetlb: Option<BitType>,
    /*
       (since Linux 2.6.27)
       Number of bytes of RAM linearly mapped by kernel in
       4 MB pages.  (x86 with CONFIG_X86_64 or
       CONFIG_X86_PAE enabled.)
    */
    direct_map4k: Option<BitType>,
    /*
     Number of bytes of RAM linearly mapped by kernel in
     4 MB pages.  (x86 with CONFIG_X86_64 or
     CONFIG_X86_PAE enabled.)
    */
    direct_map4m: Option<BitType>,
    /*
       (since Linux 2.6.27)
       Number of bytes of RAM linearly mapped by kernel in
       2 MB pages.  (x86 with neither CONFIG_X86_64 nor
       CONFIG_X86_PAE enabled.)
    */
    direct_map2_m: Option<BitType>,
    /*
       (since Linux 2.6.27)
       (x86 with CONFIG_X86_64 and
       CONFIG_X86_DIRECT_GBPAGES enabled.)
    */
    direct_map1_g: Option<BitType>,
}

pub fn mem_info() -> std::io::Result<MemoInfo> {
    let path = "/proc/meminfo";
    let mut mem_info = MemoInfo::default();
    let file_content = std::fs::read_to_string(path)?;
    file_content
        .split('\n')
        .map(|s| s.split(':'))
        .map(|mut s| -> Option<(&str, &str)> { Some((s.next()?.trim(), s.next()?.trim())) })
        .flatten()
        .for_each(|(s1, s): (&str, &str)| match s1 {
            "MemTotal" => mem_info.mem_total = BitType::new(s),
            "MemFree" => mem_info.mem_free = BitType::new(s),
            "MemAvailable" => mem_info.mem_available = BitType::new(s),
            "Buffers" => mem_info.buffers = BitType::new(s),
            "Cached" => mem_info.cached = BitType::new(s),
            "SwapCached" => mem_info.swap_cached = BitType::new(s),
            "Active" => mem_info.active = BitType::new(s),
            "Inactive" => mem_info.inactive = BitType::new(s),
            "Active(anon)" => mem_info.active_anon = BitType::new(s),
            "Inactive(anon)" => mem_info.inactive = BitType::new(s),
            "Active(file)" => mem_info.active_file = BitType::new(s),
            "Inactive(file)" => mem_info.inactive_file = BitType::new(s),
            "Unevictable" => mem_info.unevictable = BitType::new(s),
            "Mlocked" => mem_info.mlocked = BitType::new(s),
            "HighTotal" => mem_info.high_total = BitType::new(s),
            "HighFree" => mem_info.high_free = BitType::new(s),
            "LowTotal" => mem_info.low_total = BitType::new(s),
            "LowFree" => mem_info.low_free = BitType::new(s),
            "MmapCopy" => mem_info.mmap_copy = BitType::new(s),
            "SwapTotal" => mem_info.swap_total = BitType::new(s),
            "SwapFree" => mem_info.swap_free = BitType::new(s),
            "Dirty" => mem_info.dirty = BitType::new(s),
            "Writeback" => mem_info.write_back = BitType::new(s),
            "AnonPages" => mem_info.anon_pages = BitType::new(s),
            "Mapped" => mem_info.mapped = BitType::new(s),
            "Shmem" => mem_info.shmem = BitType::new(s),
            "KReclaimable" => mem_info.k_reclaimable = BitType::new(s),
            "Slab" => mem_info.slab = BitType::new(s),
            "SReclaimable" => mem_info.s_reclaimable = BitType::new(s),
            "SUnreclaim" => mem_info.s_unreclaim = BitType::new(s),
            "KernelStack" => mem_info.kernel_stack = BitType::new(s),
            "PageTables" => mem_info.page_tables = BitType::new(s),
            "Quicklists" => mem_info.quicklists = BitType::new(s),
            "NFS_Unstable" => mem_info.n_f_s_unstable = BitType::new(s),
            "Bounce" => mem_info.bounce = BitType::new(s),
            "WritebackTmp" => mem_info.writeback_tmp = BitType::new(s),
            "CommitLimit" => mem_info.commit_limit = BitType::new(s),
            "HardwareCorrupted" => mem_info.hardware_corrupted = BitType::new(s),
            "Committed_AS" => mem_info.committed_a_s = BitType::new(s),
            "VmallocTotal" => mem_info.vmalloc_total = BitType::new(s),
            "VmallocUsed" => mem_info.vmalloc_used = BitType::new(s),
            "VmallocChunk" => mem_info.vmalloc_chunk = BitType::new(s),
            "LazyFree" => mem_info.lazy_free = BitType::new(s),
            "AnonHugePages" => mem_info.anon_huge_pages = BitType::new(s),
            "ShmemHugePages" => mem_info.shmem_huge_pages = BitType::new(s),
            "ShmemPmdMapped" => mem_info.shmem_pmd_mapped = BitType::new(s),
            "CmaTotal" => mem_info.cma_total = BitType::new(s),
            "CmaFree" => mem_info.cma_free = BitType::new(s),
            "HugePages_Total" => mem_info.huge_pages_total = BitType::new(s),
            "HugePages_Free" => mem_info.huge_pages_free = BitType::new(s),
            "HugePages_Rsvd" => mem_info.huge_pages_rsvd = BitType::new(s),
            "HugePages_Surp" => mem_info.huge_pages_surp = BitType::new(s),
            "Hugepagesize" => mem_info.huge_page_size = BitType::new(s),
            "DirectMap4k" => mem_info.direct_map4k = BitType::new(s),
            "DirectMap4M" => mem_info.direct_map4m = BitType::new(s),
            "DirectMap2M" => mem_info.direct_map2_m = BitType::new(s),
            "DirectMap1G" => mem_info.direct_map1_g = BitType::new(s),
            "Hugetlb" => mem_info.hugetlb = BitType::new(s),
            "Precpu" => mem_info.percpu = BitType::new(s),
            "FileHugePages" => mem_info.file_huge_pages = BitType::new(s),
            "FilePmdMapped" => mem_info.file_pmd_mapped = BitType::new(s),
            _ => {
                log(LogLevel::Warn(
                    std::format!("{} not support, value : {}", s1, s).as_str(),
                ));
            }
        });
    Ok(mem_info)
}

#[test]
fn test_mem_info() {
    assert!(mem_info().is_ok());
}
