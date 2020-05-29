fn main() {
    let logical: bool = false;
    let a_float: f32 = 1.0;
    let an_integer = 5i32; // i32
    let def_float = 3.0; // f64
    let def_integer = 7; // i32

    let mut inferred_type = 12; // inferred 下の行から推論される
    inferred_type = 4294967296i64;

    let mut mutable = 12; //変更可能
    mutable = 21;

    // mutable = true; // 型の変更は不可

    let mutable: bool = false; // shadowingで型を変えることはできる（内部では別物扱い）
}
