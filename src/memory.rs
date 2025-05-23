use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};
/// 返回一个对活动的4级表的可变引用。
///
/// 这个函数是不安全的，因为调用者必须保证完整的物理内存在传递的
/// `physical_memory_offset`处被映射到虚拟内存。另外，这个函数
/// 必须只被调用一次，以避免别名"&mut "引用（这是未定义的行为）。
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
                                   -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    unsafe { &mut *page_table_ptr }
}