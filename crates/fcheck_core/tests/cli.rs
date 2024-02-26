// TODO: No clue why I need this, as this is supposedly unnecessary and actually bad practice since Edition 2015, but otherwise the build breaks. Fix.
extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*; // Add methods on commands

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = std::process::Command::cargo_bin("fcheck")?;

        cmd.arg("../../samples/basic.py");
        cmd.assert().failure();

        Ok(())
    }
}
