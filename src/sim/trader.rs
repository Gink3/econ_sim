
#[derive(Debug)]
pub struct Trader {
    age: u8,
} 

impl Trader {
    pub fn new(a: u8) -> Trader {
        Trader {
            age: a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}