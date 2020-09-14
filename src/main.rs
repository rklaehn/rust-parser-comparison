#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate combine;
mod pest_eval;
mod pom_eval;
mod combine_eval;

fn main() -> anyhow::Result<()> {
    let json = pest_eval::parse("{}")?;
    println!("{}", json);

    let json = pom_eval::parse("{}")?;
    println!("{:?}", json);

    let json = combine_eval::parse("{}")?;
    println!("{:?}", json);
    Ok(())
    // println!("Hello, world!");
}
