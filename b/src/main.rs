use a::*;

#[derive(Debug, Clone, Copy)]
struct OtherValue;
impl Tracked for OtherValue {
    type Type = ValueType;
}
impl Value for OtherValue {
    type Expr<'a> = OtherProxy<'a>;
}

#[derive(Debug, Clone, Copy)]
struct OtherProxy<'a>(&'a Expr<'a, OtherValue>);

impl<'a> ExprProxy<'a> for OtherProxy<'a> {
    type Value = OtherValue;
    fn _wrap(expr: &'a Expr<'a, Self::Value>) -> Self {
        OtherProxy(expr)
    }
}
impl OtherProxy<'_> {
    fn print_node_with_marking(&self) {
        println!("This is an Expr<OtherValue>! {:?}", self.0.print_node());
    }
}

fn main() {
    let x = Expr::<f32>::from_node(NodeRef);
    x.do_thing();
    let y = Expr::<OtherValue>::from_node(NodeRef);
    y.print_node_with_marking();
}
