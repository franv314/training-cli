mod api;
mod error;

const USERNAME: &str = "username";
const PASSWROD: &str = "password";

fn main() -> error::Result<()> {
    let token = api::login::login(USERNAME, PASSWROD)?;

    dbg!(token);

    Ok(())
}
