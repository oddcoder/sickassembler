// TODO use this instead of the tuple in the AssemblyDefs, later
// this type is created to avoid having a tuple with an empty entry incase only one operand
// is present
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
#[derive(Debug,PartialEq,Clone)]
pub enum UnitOrPair<T> {
    Unit(T),
    Pair(T, T),
    None,
=======
#[derive(Debug,PartialEq)]
pub enum UnitOrPair<T> {
    Unit(T),
    Pair(T, T),
>>>>>>> Pass 2 preparation
}
