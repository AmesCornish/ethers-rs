/// Macro that expands to a struct with common AST node fields.
macro_rules! ast_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub id: usize,
            #[serde(with = "serde_helpers::display_from_str")]
            pub src: SourceLocation,
            $(
                $(#[$field_meta])*
                pub $field: $ty
            ),*
        }
        impl NodeTrait for $name {
            fn node_data(&self) -> (usize, SourceLocation) {
                (self.id, self.src.clone())
            }
        }
        impl NodeIterable for $name {
            fn iter(&self) -> NodeIterator {
                NodeVector::new()
                    $(
                        .extend(&self.$field)
                    )*
                        .iter()
            }
        }
    }
}

/// A macro that expands to a struct with common expression node fields.
macro_rules! expr_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)*
        }
    ) => {
        ast_node!(
            $(#[$struct_meta])*
            struct $name {
                #[serde(default, deserialize_with = "serde_helpers::default_for_null")]
                argument_types: Vec<TypeDescriptions>,
                #[serde(default)]
                is_constant: bool,
                #[serde(default)]
                is_l_value: bool,
                #[serde(default)]
                is_pure: bool,
                #[serde(default)]
                l_value_requested: bool,
                type_descriptions: TypeDescriptions,
                $(
                    $(#[$field_meta])*
                    $field: $ty
                ),*
            }
        );
        impl ExpressionTrait for $name {
            fn get_type(&self) -> &Option<String> {
                &self.type_descriptions.type_string
            }
        }
    }
}

/// A macro that expands to a struct with common statement node fields.
macro_rules! stmt_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)*
        }
    ) => {
        ast_node!(
            $(#[$struct_meta])*
            struct $name {
                // TODO
                documentation: Option<String>,
                $(
                    $(#[$field_meta])*
                    $field: $ty
                ),*
            }
        );
    }
}

/// A macro that expands to an enum where each variant also contains a struct of the same name.
///
/// The inner value of each variant is boxed since AST types are inherently recursive.
macro_rules! node_group {
    ($group:ident$(: $trait:ident)?; $( $name:ident ),* $(,)*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Delegate)]
        $(#[delegate($trait)])?
        #[delegate(NodeTrait)]
        #[delegate(NodeIterable)]
        #[serde(tag = "nodeType")]
        pub enum $group {
            $(
                $name(Box<$name>),
            )*
        }
    };
}

macro_rules! node_group_yul {
    ($group:ident; $( $name:ident ),* $(,)*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(tag = "nodeType")]
        pub enum $group {
            $(
                $name(Box<$name>),
            )*
        }
    };
}

pub(crate) use ast_node;
pub(crate) use expr_node;
pub(crate) use node_group;
pub(crate) use node_group_yul;
pub(crate) use stmt_node;
