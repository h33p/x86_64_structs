//! Representations of various x86 specific structures and descriptor tables.

// idt needs `feature(abi_x86_interrupt)`, which is not available on stable rust
#[cfg(feature = "abi_x86_interrupt")]
pub mod idt;

pub mod paging;
pub mod port;
pub mod tss;

/// A struct describing a pointer to a descriptor table (GDT / IDT).
/// This is in a format suitable for giving to 'lgdt' or 'lidt'.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}
