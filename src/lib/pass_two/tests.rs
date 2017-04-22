#[cfg(test)]
mod tests {
    use pass_two::translator::*;
    use instruction::{Instruction, AsmOperand};
    use operands::{OperandType, Value};
    use formats::Format;
    use flags::Flags;
    use unit_or_pair::UnitOrPair;
    use register::Register;
    #[test]
    fn flag_resolution() {
        let mut instr: Instruction =
            Instruction::new(String::new(),
                             "JSUB".to_owned(),
                             UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                              Value::SignedInt(0x1036))));

        // Format should be set first before adding any operands
        // +JSUB WRREC 4B10105D
        instr.set_format(Format::Four);
        // n i x b p e | 20-addr - 0 - indexed
        // 0 1 0 0 0 1
        assert_eq!(instr.get_flags_value().unwrap(), (1 << 20) + (1 << 20 + 4));
    }
}
