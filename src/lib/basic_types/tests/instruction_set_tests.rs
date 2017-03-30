#[cfg(test)]
mod instuction_set_tests {

    use basic_types::instruction_set;
    use basic_types::formats;
    #[test]
    fn check_op_code() {
        let op = instruction_set::fetch_instruction(&"add".to_owned()).unwrap().op_code;
        assert_eq!(op, 0x18);
    }

    fn check_format() {
        let instr = instruction_set::fetch_instruction(&"comp".to_owned()).unwrap();
        assert_eq!(instr.match_format(&formats::Format::Four), true);
        assert_eq!(instr.match_format(&formats::Format::Three), true);
        assert_eq!(instr.match_format(&formats::Format::Two), false);
        assert_eq!(instr.match_format(&formats::Format::One), false);
    }
}
