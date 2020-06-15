use super::{
    ast,
    context::Context,
    structure::{Node, StructureGraph},
};
use crate::errors;
use petgraph::graph::{EdgeIndex, NodeIndex};

/// Implements convience functions to build commonly used ast nodes and
/// add them to the structure graph.
pub trait ASTBuilder {
    /// Abstract representation for the  indexing types used by the underlying
    /// graph representation.
    type ComponentHandle;
    type ConnectionHandle;

    /// Representation of values representing the port.
    type PortRep;

    /// Construct a new primitive of type `prim` with paramters `params`.
    /// The identifier for this component uses the prefix `name_prefix`.
    /// Uses the `ctx` to check the well-formedness of the primitive
    /// instantiation.
    ///
    /// Returns a handle to the component that can be used by the underlying
    /// graph representation to access this new components internal
    /// representation.
    fn new_primitive<S: AsRef<str>>(
        &mut self,
        ctx: &Context,
        name_prefix: S,
        prim: S,
        params: &[u64],
    ) -> errors::Result<Self::ComponentHandle>;

    /// Create a new constant with value `val` and width `width` and add
    /// it to the structure graph. All numbers are represented using
    /// NumType::Decimal.
    ///
    /// Returns a handle to the component for the constant and the default
    /// port on which the constant component outputs values.
    fn new_constant(
        &mut self,
        val: u64,
        width: u64,
    ) -> errors::Result<(Self::ComponentHandle, Self::PortRep)>;

}

impl ASTBuilder for StructureGraph {
    type ComponentHandle = NodeIndex;
    type ConnectionHandle = EdgeIndex;
    type PortRep = ast::Id;

    fn new_primitive<S: AsRef<str>>(
        &mut self,
        ctx: &Context,
        name_prefix: S,
        prim: S,
        params: &[u64],
    ) -> errors::Result<NodeIndex> {
        let prim_name = self.namegen.gen_name(name_prefix.as_ref());
        let prim_comp = ctx.instantiate_primitive(
            prim_name.clone(),
            &prim.as_ref().into(),
            params,
        )?;
        Ok(self.add_primitive(
            prim_name.into(),
            prim.as_ref(),
            &prim_comp,
            params,
        ))
    }

    fn new_constant(
        &mut self,
        val: u64,
        width: u64,
    ) -> errors::Result<(NodeIndex, ast::Id)> {
        let bitnum = ast::BitNum {
            width,
            num_type: ast::NumType::Decimal,
            val,
            span: None,
        };
        let (name, node) = Node::new_constant(&mut self.namegen, &bitnum);
        let idx = self.add_node(name, node);
        Ok((idx, ast::Id::new("out", None)))
    }
}
