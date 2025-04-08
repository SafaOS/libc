pub const lconv = extern struct {
    decimal_point: [3]u8,
    thousands_sep: [3]u8,
    grouping: [3]u8,
    int_curr_symbol: [3]u8,
    currency_symbol: [3]u8,
    mon_decimal_point: [3]u8,
    mon_thousands_sep: [3]u8,
    mon_grouping: [3]u8,
    positive_sign: [3]u8,
    negative_sign: [3]u8,
    int_frac_digits: u8,
    frac_digits: u8,
    p_cs_precedes: u8,
    p_sep_by_space: u8,
    n_cs_precedes: u8,
    n_sep_by_space: u8,
    p_sign_posn: u8,
    n_sign_posn: u8,
    int_p_cs_precedes: u8,
    int_p_sep_by_space: u8,
    int_n_cs_precedes: u8,
    int_n_sep_by_space: u8,
    int_p_sign_posn: u8,
    int_n_sign_posn: u8,
};

// STUB
pub export fn localeconv() *lconv {
    @panic("localeconv: unimplemented");
}

// STUB
pub export fn setlocale(category: c_int, locale: [*:0]const c_char) ?[*:0]const u8 {
    _ = category;
    _ = locale;
    @panic("setlocale: unimplemented");
}
