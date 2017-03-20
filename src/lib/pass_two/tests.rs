
#[cfg(test)]
mod pass_two_tests {
    use pass_two::object_code_generator::generate_object_code;
    use basic_types::instruction::Instruction;
    use basic_types::operands::Operand;
    use basic_types::formats::Formats;
    use basic_types::flags::Flags;

    #[test]
    fn flag_resolution() {
        Operand::Immediate(6);
        //pass_two::object_code_generator::object_code_gen::generate_object_code

        let mut instr = Instruction::new(String::new(),
                                         "load".to_owned(),
                                         Operand::Immediate(5),
                                         Operand::Immediate(1));

        instr.set_format(Formats::Four);
        instr.set_flag(Flags::Immediate);
        instr.set_flag(Flags::Extended);

        generate_object_code(instr);
    }
}
