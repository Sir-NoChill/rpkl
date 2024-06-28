#[cfg(test)]
mod tests {

    #[cfg(feature = "codegen")]
    use std::path::PathBuf;
    #[cfg(feature = "codegen")]
    use pkl_rs;

    #[test]
    #[cfg(feature = "codegen")]
    fn test_appconfig() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/appconfig.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_nested_config() {
        eprintln!("Running codegen tests");
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/nesting.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }
}
