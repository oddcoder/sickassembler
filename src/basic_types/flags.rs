#[allow(dead_code)]

pub enum Flags {
    /**
     * Each enum value will represent the bit number to be set calculated from left to
     * write to avoid conflict with format 3 and 4 bit locations
     */
    Indirect,
    Immediate,
    Indexed,
    Extended,
    BaseRelative,
    PcRelative,
}
