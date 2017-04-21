///
/// Validates the semantic structure (i.e instruction set compliance) of the instructions
/// this shouldn't be done in any other module to keep the code clean
///
use instruction::Instruction;
use formats::Format;
use instruction_set::*;
use unit_or_pair::*;

pub fn validate_semantics(instr: &mut Instruction) -> Result<(), String> {
    let mut errs: Vec<String> = Vec::new();

    if let Ok(_) = fetch_directive(&instr.mnemonic) {
        // Directives are matched while reading the source code
        return Ok(());
    }

    let def: AssemblyDef;
    match fetch_instruction(&instr.mnemonic) {
        Ok(instr) => def = instr, 
        Err(_) => return Err("Isn't an instruction or a directive".to_owned()),
    }

    if def.has_valid_operands(&instr.operands) == false {
        errs.push("Operands for this mnemonic are invalid".to_owned());
    }

    // Check format correctness
    if def.match_format(&instr.format) == false {
        errs.push("Invalid instruction format".to_owned());
    } else {
        // Format is matched correctly, adjust format 2 instructions with 1 register
        // check the docs of add_reg_a
        if instr.format == Format::Two && instr.unwrap_operands().len() == 1 {
            let op_count = unwrap_to_vec(&def.operands).len();

            if op_count == 1 {
                instr.add_reg_a();
                println!("{:?}", instr);
            }
        }
    }

    if errs.len() > 0 {
        return Err(errs.join(", "));
    }

    Ok(())
}
