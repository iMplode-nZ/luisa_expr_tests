use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;

static NEXT_NODE: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct NodeRef(usize);
impl NodeRef {
    pub fn new() -> Self {
        NodeRef(NEXT_NODE.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

pub trait Tracked {
    type Type: TrackingType;
    type Value: Value;
}
pub trait FromNode {
    fn from_node(node: NodeRef) -> Self;
}
pub trait TrackingType {}
pub struct ValueType;
impl TrackingType for ValueType {}
pub struct ExprType;
impl TrackingType for ExprType {}
pub struct VarType;
impl TrackingType for VarType {}

impl<T: Value> Tracked for T {
    type Type = ValueType;
    type Value = T;
}

pub trait AsExpr: Tracked {
    fn as_expr(&self) -> Expr<Self::Value>;
}

impl<T: Value> AsExpr for T {
    fn as_expr(&self) -> Expr<Self::Value> {
        Expr::from_node(NodeRef(0))
    }
}
impl<T: Value> AsExpr for Var<T> {
    fn as_expr(&self) -> Expr<T> {
        Expr::from_node(self.node)
    }
}
impl<T: Value> AsExpr for Expr<T> {
    fn as_expr(&self) -> Expr<T> {
        *self
    }
}

pub trait Value: Copy + 'static {
    type Expr: ExprProxy<Value = Self>;
    type Var: VarProxy<Value = Self>;
    type ExprData: Copy + FromNode + 'static;
    type VarData: Copy + FromNode + 'static;
}
pub unsafe trait HasExprLayout<X: Copy + FromNode> {}
pub unsafe trait HasVarLayout<X: Copy + FromNode> {}
pub trait ExprProxy: Copy + HasExprLayout<<Self::Value as Value>::ExprData> + 'static {
    type Value: Value<Expr = Self>;
}
pub trait VarProxy:
    Copy + HasVarLayout<<Self::Value as Value>::VarData> + Deref<Target = Expr<Self::Value>> + 'static
{
    type Value: Value<Var = Self>;
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Expr<T: Value> {
    node: NodeRef,
    _phantom: PhantomData<T>,
    data: T::ExprData,
}
impl<T: Value> Tracked for Expr<T> {
    type Type = ExprType;
    type Value = T;
}
impl<T: Value> Deref for Expr<T> {
    type Target = T::Expr;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: Value> FromNode for Expr<T> {
    fn from_node(node: NodeRef) -> Self {
        Expr {
            node,
            _phantom: PhantomData,
            data: T::ExprData::from_node(node),
        }
    }
}
impl<T: Value> Expr<T> {
    pub fn from_proxy(proxy: &T::Var) -> &Self {
        unsafe { std::mem::transmute(proxy) }
    }
    pub fn print_node(&self) {
        println!("{:?}", self.node);
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Var<T: Value> {
    node: NodeRef,
    _phantom: PhantomData<T>,
    data: T::VarData,
}
impl<T: Value> Tracked for Var<T> {
    type Type = VarType;
    type Value = T;
}
impl<T: Value> Deref for Var<T> {
    type Target = T::Var;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: Value> FromNode for Var<T> {
    fn from_node(node: NodeRef) -> Self {
        Var {
            node,
            _phantom: PhantomData,
            data: T::VarData::from_node(node),
        }
    }
}
impl<T: Value> Var<T> {
    pub fn print_node(&self) {
        println!("{:?}", self.node);
    }
    pub fn from_proxy(proxy: &T::Var) -> &Self {
        unsafe { std::mem::transmute(proxy) }
    }
    pub fn _deref(&self) -> &Expr<T> {
        todo!();
    }
}

impl<T: Default> FromNode for T {
    fn from_node(_node: NodeRef) -> Self {
        Self::default()
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct DefaultVarProxy<T: Value<VarData = ()>>(Var<T>);
unsafe impl<T: Value<VarData = ()>> HasVarLayout<()> for DefaultVarProxy<T> {}
impl<T: Value<VarData = ()>> Deref for DefaultVarProxy<T> {
    type Target = Expr<T>;
    fn deref(&self) -> &Self::Target {
        self.0._deref()
    }
}
impl<T: Value<VarData = (), Var = DefaultVarProxy<T>>> VarProxy for DefaultVarProxy<T> {
    type Value = T;
}

impl Value for f32 {
    type Expr = F32ExprProxy;
    type Var = DefaultVarProxy<Self>;
    type ExprData = ();
    type VarData = ();
}
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct F32ExprProxy(Expr<f32>);
unsafe impl HasExprLayout<()> for F32ExprProxy {}
impl ExprProxy for F32ExprProxy {
    type Value = f32;
}
impl F32ExprProxy {
    pub fn do_float_thing(&self) {
        println!("Doing float thing");
    }
}
