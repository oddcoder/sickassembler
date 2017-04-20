// TODO use this instead of the tuple in the AssemblyDefs, later
// this type is created to avoid having a tuple with an empty entry incase only one operand
// is present
#[derive(Debug,PartialEq,Clone)]
pub enum UnitOrPair<T> {
    Unit(T),
    Pair(T, T),
    None,
}

pub fn unwrap_to_vec<T>(u_or_p: &UnitOrPair<T>) -> Vec<T>
    where T: Clone
{

    let operands = match u_or_p {
        &UnitOrPair::None => vec![],
        &UnitOrPair::Unit(ref o1) => vec![o1.clone()],
        &UnitOrPair::Pair(ref o1, ref o2) => vec![o1.clone(), o2.clone()],
    };

    operands.to_owned()
}
