// TODO use this instead of the tuple in the AssemblyDefs, later
// this type is created to avoid having a tuple with an empty entry incase only one operand
// is present
#[derive(Debug,PartialEq)]
pub enum UnitOrPair<T> {
    Unit(T),
    Pair(T, T),
}
