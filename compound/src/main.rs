fn main() {
    let array: [u32; 3] = [1u32, 2, 3];
    let first_element = array[0];

    let length = "Some text".len();
    [1][length];

    let tuple: (u32, u8, bool) = (1u32, 2, true);
    let first_element = (1, 2, true).0;
}
