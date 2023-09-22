use ambassador::delegatable_trait;

pub use crate::artifacts::ast::{NodeType, SourceLocation};

#[delegatable_trait]
pub trait NodeTrait {
    fn node_data(&self) -> (usize, SourceLocation);

    fn id(&self) -> usize {
        self.node_data().0
    }

    fn src(&self) -> SourceLocation {
        self.node_data().1
    }
}

impl<T: NodeTrait> NodeTrait for &T {
    fn node_data(&self) -> (usize, SourceLocation) {
        (*self).node_data()
    }
}

impl<T: NodeTrait> NodeTrait for Box<T> {
    fn node_data(&self) -> (usize, SourceLocation) {
        self.as_ref().node_data()
    }
}

#[delegatable_trait]
pub trait ExpressionTrait {
    fn get_type(&self) -> &Option<String>;
}

impl<T> ExpressionTrait for Box<T>
where
    T: ExpressionTrait,
{
    fn get_type(&self) -> &Option<String> {
        self.as_ref().get_type()
    }
}

pub(crate) use ambassador_impl_ExpressionTrait;
pub(crate) use ambassador_impl_NodeTrait;
