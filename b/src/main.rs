use std::{ops::Deref, sync::OnceLock};

use a::*;

#[derive(Debug, Copy, Clone)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SwizzlesExprData {
    x: Expr<f32>,
    y: Expr<f32>,
}
impl FromNode for SwizzlesExprData {
    fn from_node(_node: NodeRef) -> Self {
        SwizzlesExprData {
            x: Expr::from_node(NodeRef::new()),
            y: Expr::from_node(NodeRef::new()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SwizzlesVarData {
    x: Var<f32>,
    y: Var<f32>,
}
impl FromNode for SwizzlesVarData {
    fn from_node(_node: NodeRef) -> Self {
        SwizzlesVarData {
            x: Var::from_node(NodeRef::new()),
            y: Var::from_node(NodeRef::new()),
        }
    }
}

impl Value for Float2 {
    type ExprData = SwizzlesExprData;
    type VarData = SwizzlesVarData;
    type Expr = Float2ExprProxy;
    type Var = Float2VarProxy;
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Float2ExprProxy {
    node: NodeRef,
    pub x: Expr<f32>,
    pub y: Expr<f32>,
}
unsafe impl HasExprLayout<SwizzlesExprData> for Float2ExprProxy {}
impl ExprProxy for Float2ExprProxy {
    type Value = Float2;
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Float2VarProxy {
    node: NodeRef,
    pub x: Var<f32>,
    pub y: Var<f32>,
}
unsafe impl HasVarLayout<SwizzlesVarData> for Float2VarProxy {}
impl Deref for Float2VarProxy {
    type Target = Expr<Float2>;
    fn deref(&self) -> &Self::Target {
        Var::<Float2>::from_proxy(self)._deref()
    }
}
impl VarProxy for Float2VarProxy {
    type Value = Float2;
}

fn main() {
    let f = Float2 { x: 1.0, y: 2.0 }.as_expr();
    f.x.do_float_thing();
    let v = Var::<Float2>::from_node(NodeRef::new());
    v.y.do_float_thing();
}
