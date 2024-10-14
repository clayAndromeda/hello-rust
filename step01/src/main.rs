fn main() {
    let x: u32 = 5;
    let y = 20;
    let z = mul(x, y);

    println!("The result is: {}", z);

    println!("a || b = {}", bool_a() || bool_b());
    println!("a | b = {}",  bool_a() |  bool_b());

    println!("===== 参照型 =====");
    let mut n: u64 = 100; // 破壊的代入可能な変数を宣言してみる
    let a: &u64 = &n; // nの参照をaに束縛
    // *a = 200; // 破壊的代入はできない
    println!("a = {}, addr = {:p}", a, a);

    let b: &mut u64 = &mut n; // nの参照をbに束縛
    *b = 200; // bの指す先 = nに200を破壊的代入
    println!("b = {}, addr = {:p}", b, b);
    println!("n = {n}");

    println!("===== 配列, スライス =====");
    let arr : [u32; 4] = [1, 2, 3, 4];
    println!("arr = {:?}", arr);
    let slice = &arr[1..3];
    println!("slice = {:?}", slice);
    // slice[5]; 実行時のパニック（スライスは動的検査のため実行時までエラーがでない）

    println!("===== 文字列 =====");
    // Rustでは、文字と文字列は別の型である。
    let a: &str = " Hello"; // &strは文字列スライス
    // a += ", world!"; // 文字列スライスは不変なので、+=はできない
    let mut b: String = a.to_string(); // Stringに変換（ヒープに確保される）
    b += ", world!   "; // Stringは可変なので、+=ができる
    let c: &str = b.trim(); // trim()は文字列の前後の空白を取り除く
    println!("c = {}", c);

    println!("===== 関数ポインタ =====");
    do_it(add, 10, 20);
    do_it(mul, 10, 20);

    println!("===== 構造体 / enum/variant =====");
    enum Storage {
        HDD { size: u32, rpm: u32 },
        SSD (u32),
    }
    let _hdd = Storage::HDD { size: 1024, rpm: 7200 };

    struct PCSpec {
        cpus: u16, // CPU Core数
        memory: u32, // メモリ容量[GiB]
        storage: Storage, // ストレージの種別と容量
    }
    let spec = PCSpec {
        cpus: 4,
        memory: 16,
        storage: Storage::SSD(1024),
    };
    println!("spec = {}", spec.cpus);
}

fn mul(x: u32, y: u32) -> u32 {
    // セミコロンが最後にないことに注意。Rustでは、関数の最後の値が返り値となる
    x * y
}

fn bool_a() -> bool { println!("a"); true }
fn bool_b() -> bool { println!("b"); false }

fn do_it(f: fn(u32, u32) -> u32, x: u32, y: u32) {
    println!("do_it! {}", f(x, y));
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}
