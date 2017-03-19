#[cfg(test)]
mod instuction_set_tests {

    use basic_types::instruction_set;
<<<<<<< ff98c4d9a1ef854a737bc0b12d7e563e0af6f1d5
}
=======

    #[test]
    fn test_existing_instruction() {
        assert!(instruction_set::exists("ADD"));
    }

    #[test]
    fn flag_usability_from_outer_module() {
        assert!(instruction_set::OP_M.bits() > 0); // Convert the flag to a number
    }
}
>>>>>>> Add instruction set file, tests
