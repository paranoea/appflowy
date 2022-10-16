use crate::core::{AttributeHashMap, DeltaOperation, DeltaOperations, OperationBuilder};

pub type TextOperations = DeltaOperations<AttributeHashMap>;
pub type TextOperationBuilder = OperationBuilder<AttributeHashMap>;
pub type TextOperation = DeltaOperation<AttributeHashMap>;

// pub trait TextOperation2: Default + Debug + OperationTransform {}
//
// impl TextOperation2 for TextOperations {}