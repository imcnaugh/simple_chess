const NEW_BASE: u8 = 26;

/// Returns the name of the column of a chess board from the index of the column
///
/// Note that the index should start at 1, not 0.
///
/// ## Examples
/// ```
/// use chess::base_converter;
/// let col_number: u8 = 1;
/// let col_name = base_converter::get_column_name_from_index(col_number);
///
/// assert_eq!("a", col_name);
/// ```
pub fn get_column_name_from_index(mut num: u8) -> String {
    let mut result = String::new();

    if num == 0 {
        return String::new();
    }

    loop {
        let remainder = (num - 1) % NEW_BASE;
        let devisor = (num - 1) / NEW_BASE;

        result.push((b'a' + remainder) as char);

        if devisor == 0 {
            break;
        }

        num = devisor;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char_response() {
        assert_eq!("a", get_column_name_from_index(1));
        assert_eq!("b", get_column_name_from_index(2));
        assert_eq!("z", get_column_name_from_index(26));
    }

    #[test]
    fn double_char_response() {
        assert_eq!("aa", get_column_name_from_index(27));
        assert_eq!("ab", get_column_name_from_index(28));
        assert_eq!("hq", get_column_name_from_index(225));
    }
}
