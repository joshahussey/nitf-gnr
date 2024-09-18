enum NitfHeader21{
    A,
    B,
    C,
    d,
    FHEV,
}

enum NitfImageSubheader21{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl MyEnum {
    fn values() -> &'static [usize] {
        &[33, 9, 123, 324, 1]
    }

    fn get_offset(target: MyEnum) -> usize {
        let index = target as usize;
        MyEnum::values()[..=index].iter().sum()
    }
}

fn main() {
    let result = MyEnum::get_offset(MyEnum::B);
    println!("Result: {}", result);
}

fn get_header_field(field: MyEnum) -> &str {
    let mut buffer = [0; MyEnum::values()[field as usize]];
    self.seek(SeekFrom::Start(MyEnum::get_offset(field) as u64));
    self.read_exact(&buffer, MyEnum::values()[field as usize]);
}
