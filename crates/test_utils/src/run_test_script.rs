use crate::script_instructions;
use koto_bytecode::{Chunk, CompilerSettings, ModuleLoader};
use koto_runtime::{Ptr, Result, prelude::*};

/// Compiles the script with default settings
pub fn compile_test_script(script: &str, script_path: Option<KString>) -> Result<Ptr<Chunk>> {
    let mut loader = ModuleLoader::default();
    match loader.compile_script(script, script_path, CompilerSettings::default()) {
        Ok(chunk) => Ok(chunk),
        Err(error) => {
            println!("{script}\n");
            Err(format!("Error while compiling script: {error}").into())
        }
    }
}

/// Runs a script using the provided Vm, optionally checking its output
pub fn run_test_script(
    mut vm: KotoVm,
    script: &str,
    script_path: Option<KString>,
    expected_output: Option<KValue>,
) -> Result<()> {
    let chunk = compile_test_script(script, script_path)?;

    match vm.run(chunk) {
        Ok(result) => {
            if let Some(expected_output) = expected_output {
                match vm.run_binary_op(BinaryOp::Equal, result.clone(), expected_output.clone()) {
                    Ok(KValue::Bool(true)) => {}
                    Ok(KValue::Bool(false)) => {
                        return Err(format!(
                            "{}\nUnexpected result - expected: {}, result: {}",
                            script_instructions(script, vm.chunk()),
                            vm.value_to_string(&expected_output).unwrap(),
                            vm.value_to_string(&result).unwrap(),
                        )
                        .into());
                    }
                    Ok(other) => {
                        return Err(format!(
                            "{}\nExpected bool from equality comparison, found '{}'",
                            script_instructions(script, vm.chunk()),
                            vm.value_to_string(&other).unwrap()
                        )
                        .into());
                    }
                    Err(e) => {
                        return Err(format!(
                            "{}\nError while comparing output value: ({e})",
                            script_instructions(script, vm.chunk()),
                        )
                        .into());
                    }
                }
            }

            match vm.run_tests(vm.exports().clone()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{}\n {e}", script_instructions(script, vm.chunk())).into()),
            }
        }

        Err(e) => Err(format!(
            "{}\nError while running script: {e}",
            script_instructions(script, vm.chunk())
        )
        .into()),
    }
}
