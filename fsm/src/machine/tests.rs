#[cfg(test)]
mod test {
    use super::Machine;

    #[test]
    fn parse_string() -> Result<(), String> {
        let machine = Machine::from_path(Path::new().string("Hello"))?;
    }
}
