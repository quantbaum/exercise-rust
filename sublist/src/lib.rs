#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn equallist<T:PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut eq = 0;
    for (x, y) in _first_list.iter().zip(_second_list.iter()){
        if x == y{
            eq += 1;
        }
    }
    if _first_list.len() == eq{
        Comparison::Equal
    }
    else{
        Comparison::Unequal
    }
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    let mut eq = Comparison::Unequal;
    if _first_list.len() == _second_list.len(){
        eq = equallist(_first_list, _second_list);
    }
    else if _first_list.len() > _second_list.len(){
        let mut tmp_eq1 = sublist(&_first_list[1..], _second_list);
        let mut tmp_eq2 = sublist(&_first_list[.._first_list.len() - 1], _second_list);
        eq = match (tmp_eq1, tmp_eq2){
            (Comparison::Unequal, Comparison::Unequal) => Comparison::Unequal,
            _ => Comparison::Superlist
        }
    }
    else if _first_list.len() < _second_list.len(){
        let mut tmp_eq1 = sublist(_first_list, &_second_list[1..]);
        let mut tmp_eq2 = sublist(_first_list, &_second_list[.._second_list.len() - 1]);
        eq = match (tmp_eq1, tmp_eq2){
            (Comparison::Unequal, Comparison::Unequal) => Comparison::Unequal,
            _ => Comparison::Sublist
        }
    }
    eq
}
