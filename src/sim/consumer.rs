
pub struct Consumer<'a> {
    pop: u64,
    //rate is a tuple of per person per day
    rates: Vec<(u64,String)<'a>>,
    inv: Vec<(String,u64)<'a>,
}