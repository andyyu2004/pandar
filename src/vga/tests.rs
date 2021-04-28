use super::*;

#[test_case]
fn test_println_simple() {
    println!("test println simple output")
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test println simple output")
    }
}

#[test_case]
fn test_println_output() {
    let s = "some single line string";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(char.ascii_code), c);
    }
}
