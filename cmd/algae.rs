use lsystem::{LSystemBuilder, LSystemError};
use lsystem::token::TokenId;

fn main() -> Result<(), LSystemError> {
    let mut builder = LSystemBuilder::new();


    let token = TokenId::new(119, true);
    println!("token: {:?} {} {}", token.0, token.value(), token.has_param());

    let a = builder.token("A")?;
    let b = builder.token("B")?;

    builder.axiom(vec![a])?;

    builder.production_rule(a, vec![a, b])?; // A -> AB
    builder.production_rule(b, vec![a])?;   // B -> A

    let mut system = builder.finish()?;

    for _ in 0..25 {
        system.reset();
        system.step_by(30);
    }
    Ok(())
}
