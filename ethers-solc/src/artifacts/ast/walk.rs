use ambassador::{delegatable_trait, Delegate};
use derive_more::From;
use std::fmt::Display;

use super::*;
use traits::{ambassador_impl_NodeTrait, NodeTrait};

#[derive(Debug, Delegate, From)]
#[delegate(NodeTrait)]
#[delegate(NodeIterable)]
pub enum Node<'a> {
    Block(&'a Block),
    BlockOrStatement(&'a BlockOrStatement),
    ContractDefinitionPart(&'a ContractDefinitionPart),
    ElementaryTypeName(&'a ElementaryTypeName),
    EnumValue(&'a EnumValue),
    Expression(&'a Expression),
    ExpressionOrVariableDeclarationStatement(&'a ExpressionOrVariableDeclarationStatement),
    ExpressionStatement(&'a ExpressionStatement),
    FunctionCall(&'a FunctionCall),
    FunctionIdentifierPath(&'a FunctionIdentifierPath),
    IdentifierOrIdentifierPath(&'a IdentifierOrIdentifierPath),
    IdentifierPath(&'a IdentifierPath),
    InheritanceSpecifier(&'a InheritanceSpecifier),
    ModifierInvocation(&'a ModifierInvocation),
    OverrideSpecifier(&'a OverrideSpecifier),
    ParameterList(&'a ParameterList),
    SourceUnit(&'a SourceUnit),
    SourceUnitPart(&'a SourceUnitPart),
    Statement(&'a Statement),
    TryCatchClause(&'a TryCatchClause),
    TypeName(&'a TypeName),
    UserDefinedTypeNameOrIdentifierPath(&'a UserDefinedTypeNameOrIdentifierPath),
    VariableDeclaration(&'a VariableDeclaration),
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

pub type NodeIterator<'a> = <Vec<Node<'a>> as IntoIterator>::IntoIter;

pub struct NodeVector<'a>(Vec<Node<'a>>);

impl<'a> NodeVector<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn extend<T>(mut self, value: &'a T) -> Self
    where
        &'a T: ToNodeVector<'a>,
    {
        let vec = value.node_vector();
        self.0.extend(vec.0);
        self
    }

    pub fn iter(self) -> NodeIterator<'a> {
        self.0.into_iter()
    }
}

pub trait ToNodeVector<'a>: Sized {
    fn node_vector(self) -> NodeVector<'a> {
        NodeVector::new()
    }
}

impl<'a, T: 'a> ToNodeVector<'a> for &'a Vec<T>
where
    Node<'a>: From<&'a T>,
{
    fn node_vector(self) -> NodeVector<'a> {
        NodeVector(self.iter().map(|v| v.node_vector().iter()).flatten().collect())
    }
}

impl<'a, T: 'a> ToNodeVector<'a> for &'a Option<T>
where
    Node<'a>: From<&'a T>,
{
    fn node_vector(self) -> NodeVector<'a> {
        match self {
            None => NodeVector::new(),
            Some(v) => v.node_vector(),
        }
    }
}

impl<'a, T: 'a> ToNodeVector<'a> for &'a Vec<Option<T>>
where
    Node<'a>: From<&'a T>,
{
    fn node_vector(self) -> NodeVector<'a> {
        NodeVector(self.iter().map(|v| v.node_vector().iter()).flatten().collect())
    }
}

impl<'a, T> ToNodeVector<'a> for &'a T
where
    Node<'a>: From<&'a T>,
    T: 'a,
{
    fn node_vector(self) -> NodeVector<'a> {
        let n = Node::from(self);
        let v = vec![n];
        NodeVector(v)
    }
}

macro_rules! not_node {
    ($($name:ty)*)
    => { $(
        impl<'a> ToNodeVector<'a> for &$name {}
        impl<'a> ToNodeVector<'a> for &Vec<$name> {}
        impl<'a> ToNodeVector<'a> for &Option<$name> {}
        impl<'a> ToNodeVector<'a> for &Vec<Option<$name>> {}
    )*
    }
}

not_node! {
    bool
    BTreeMap<String, Vec<usize>>
    BTreeMap<usize, usize>
    isize
    SourceLocation
    String
    usize

    AssemblyReferenceSuffix
    AssignmentOperator
    BinaryOperator
    ContractKind
    ElementaryOrRawTypeName
    ExternalInlineAssemblyReference
    ExternalReferences
    FunctionCallKind
    FunctionKind
    InlineAssemblyFlag
    LiteralKind
    ModifierInvocationKind
    Mutability
    StateMutability
    StorageLocation
    StructuredDocumentation
    SymbolAlias
    TypeDescriptions
    UnaryOperator
    Visibility
    YulBlock
}

// impl<'a, T: 'a> ToNodeVector<'a> for &Option<T> where T: NotNode {}  // conlicting implementation

#[delegatable_trait]
pub trait NodeIterable {
    fn iter(&self) -> NodeIterator {
        Vec::new().into_iter()
    }
}

impl<T: NodeIterable> NodeIterable for &T {
    fn iter(&self) -> NodeIterator {
        (*self).iter()
    }
}

impl<T: NodeIterable> NodeIterable for Box<T> {
    fn iter(&self) -> NodeIterator {
        self.as_ref().iter()
    }
}

pub(crate) use ambassador_impl_NodeIterable;
