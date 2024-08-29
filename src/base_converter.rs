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

/// Does a bad job of converting a chess board column name to a int number
/// I say its bad in that it should verify the input string is 'a'..='z'
/// //TODO run validation on the input
/// but it will return a int for the colum number with a 1 index, so column 'a' is 1
///
/// ## Examples
/// ```
/// use chess::base_converter;
///
/// let column_name = "a";
/// let column_number = base_converter::get_index_from_column_name(column_name);
///
/// assert_eq!(1, column_number);
/// ```
pub fn get_index_from_column_name(s: &str) -> u8 {
    let mut result = 0;

    for (i, c) in s.chars().rev().enumerate() {

        let idk = u32::pow(26, i as u32);
        let next = (c as u8 - b'a' + 1) as u32;

        result += idk * next;
    }

    result as u8
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
        assert_eq!("iu", get_column_name_from_index(255));
    }

    #[test]
    fn col_to_id() {
        assert_eq!(1, get_index_from_column_name("a"));
        assert_eq!(2, get_index_from_column_name("b"));
        assert_eq!(26, get_index_from_column_name("z"));
        assert_eq!(27, get_index_from_column_name("aa"));
        assert_eq!(28, get_index_from_column_name("ab"));
        assert_eq!(225, get_index_from_column_name("hq"));
        assert_eq!(255, get_index_from_column_name("iu"));
    }
}
