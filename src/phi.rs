use value::Value;
use block::BasicBlock;
use ffi::core;

/// A PHI node represents a value which is selected based on the predecessor of the current block.
pub type PhiNode = Value;

impl PhiNode {
    /// Adds an incoming value to the end of this PHI node.
    pub fn add_incoming(&self, val: &Value, block: &BasicBlock) {
        let mut values = vec![val.into()];
        let mut blocks = vec![block.into()];
        unsafe { core::LLVMAddIncoming(self.into(), values.as_mut_ptr(), blocks.as_mut_ptr(), 1) }.into()
    }

    /// Counts the number of incoming values attached to this PHI node.
    pub fn count_incoming(&self) -> u32 {
        unsafe { core::LLVMCountIncoming(self.into()) }
    }

    /// Gets an incoming value from this PHI node from a specific index.
    pub fn get_incoming_value(&self, index: u32) -> &Value {
        unsafe { core::LLVMGetIncomingValue(self.into(), index) }.into()
    }

    /// Gets an incoming basic block from this PHI node from a specific index.
    pub fn get_incoming_block(&self, index: u32) -> &BasicBlock {
        unsafe { core::LLVMGetIncomingBlock(self.into(), index) }.into()
    }
}
