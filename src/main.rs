use postgres::{Client, Error, NoTls};
use std::time::Duration;
fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    client.is_valid(Duration::from_secs(1))?;

    Ok(())
}
