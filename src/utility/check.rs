pub fn is_space(letter: char) -> bool {
    letter == ' '
    || letter == '\t'
    || letter == '\n'
    || letter == '\r'
}

pub fn is_letter(letter: char) -> bool {
    ('a' <= letter && letter <= 'z')
    || ('A' <= letter && letter <= 'Z')
    || (letter == '_')
}

pub fn is_digit(letter: char) -> bool {
    '0' <= letter && letter <= '9'
}