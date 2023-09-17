// TODO: Force the ExprProxy to impl Deref<ExprRaw>, and also have VarProxy which impls Deref<VarRaw> which then impls Deref<Expr>.

use std::marker::{PhantomData, PhantomPinned};
use std::mem::MaybeUninit;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct NodeRef;

pub trait Tracked {
    type Type: TrackingType;
}
pub trait TrackingType {}
pub struct ValueType;
impl TrackingType for ValueType {}
pub struct ExprType;
impl TrackingType for ExprType {}
pub struct VarType;
impl TrackingType for VarType {}

pub trait Value: Copy + Tracked<Type = ValueType> + 'static {
    type Expr<'a>: ExprProxy<'a, Value = Self>;
    // type Var: VarProxy<Value = Self>;
}
pub trait ExprProxy<'a>: Copy + 'a {
    type Value: Value<Expr<'a> = Self>;
    fn _wrap(expr: &'a Expr<'a, Self::Value>) -> Self;
}
// pub trait VarProxy: Copy {
//     type Value: Value<Var = Self>;
//     fn _wrap(var: &Var<Self::Value>) -> Self;
// }

// #[derive(Clone, Copy, Debug)]
// pub struct ExprRaw<T: Value> {
//     pub(crate) node: NodeRef,
//     _phantom: PhantomData<T>,
// }

#[derive(Clone, Copy, Debug)]
pub struct Expr<'a, T: Value> {
    node: NodeRef,
    _phantom: PhantomData<(T, &'a ())>,
    _pin: PhantomPinned,
    proxy: T::Expr<'a>,
}
impl<T: Value> Tracked for Expr<'_, T> {
    type Type = ExprType;
}
impl<'a, T: Value> Expr<'a, T> {
    pub fn from_node(node: NodeRef) -> Self {
        let mut res: Expr<'a, T> = Expr {
            node,
            _phantom: PhantomData,
            _pin: PhantomPinned,
            proxy: unsafe { MaybeUninit::uninit().assume_init() },
        };
        let proxy = T::Expr::_wrap(unsafe { std::mem::transmute::<_, &'a Expr<'a, T>>(&res) });
        res.proxy = proxy;
        res
    }
    pub fn print_node(&self) {
        println!("{:?}", self.node);
    }
}

impl<'a, T: Value> Deref for Expr<'a, T> {
    type Target = T::Expr<'a>;
    fn deref(&self) -> &Self::Target {
        &self.proxy
    }
}

impl Tracked for f32 {
    type Type = ValueType;
}
impl Value for f32 {
    type Expr<'a> = F32ExprProxy<'a>;
}
#[derive(Debug, Copy, Clone)]
pub struct F32ExprProxy<'a>(&'a Expr<'a, f32>);
impl<'a> ExprProxy<'a> for F32ExprProxy<'a> {
    type Value = f32;
    fn _wrap(expr: &'a Expr<'a, Self::Value>) -> Self {
        F32ExprProxy(expr)
    }
}
impl F32ExprProxy<'_> {
    pub fn do_thing(&self) {
        println!("Hi!");
    }
}
/*
#[derive(Clone, Copy, Debug)]
pub struct Var<T: Value> {
    pub(crate) node: NodeRef,
    _phantom: PhantomData<T>,
    proxy: T::Var,
}

impl<T: Value> Deref for Var<T> {
    type Target = Expr<T>;
    fn deref(&self) -> &Self::Target {}
}
*/
