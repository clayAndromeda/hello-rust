use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, RwLock},
    thread::sleep,
    time::Duration,
};

fn main() {
    println!("===== 所有権についての説明 =====");
    struct H2O {} // 水分子
    struct O2 {} // 酸素分子
    struct H2 {} // 水素分子

    // 水素分子2つと酸素分子1つを組み合わせて水分子を作る
    fn burn(_h2_h1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
        (H2O {}, H2O {})
    }

    let h2_1 = H2 {};
    let h2_2 = H2 {};
    let o2 = O2 {};
    let (_h2o_1, _h2o_2) = burn(h2_1, h2_2, o2); // 燃焼

    // コンパイルエラー！既に消費した分子は使えない
    // let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);

    println!("===== ライフタイムについての説明 =====");
    // let a;
    // {
    //     let b = 10;
    //     a = &b;
    // }
    // println!("{}", a); // コンパイルエラー！bのライフタイムが終わっている

    let a;
    {
        let b = 10;
        a = &b;
        println!("{}", a); // これは問題ない。非字句ライフタイムが生存しているので
    }

    println!("===== ライフタイム型 =====");
    let a: i32 = 10;
    let b: &i32 = &a;

    // ライフタイムを取る関数
    fn square<'a>(x: &'a i32) -> i32 {
        x * x
    }
    square(b);

    // 参照を持つ構造体
    struct Foo<'a> {
        _x: &'a i32, // 構造体のフィールドに参照を持つ場合は、'aのようにライフタイムを指定する必要がある
    }
    Foo { _x: &a };

    println!("===== 借用 =====");
    let a = 10; // 不変参照としての借用
    {
        let b = &a;
        let _c = &a;
        let _d = b;
    }
    // Rustは参照カウントではなくライフタイムを用いて、コンパイル時に無効な参照がないかをチェックする

    let mut a = 10;
    let b = &mut a; // 可変参照
    let c = b; // ムーブ（bの所有権がcに移動する）
    *c = 20; // 参照先の値を変更
             // *b = 30; ※コンパイルエラー！bの所有権がcに移動しているため
             // a = 40;  ※コンパイルエラーになる。今&mut借用しているから、書き込み口はbだけ。

    // 基本的に、&借用はコピーセマンティクスで、&mut借用はムーブセマンティクス

    println!("===== スマートポインタを使った排他制御 =====");
    // ミューテックスをArcで包むのは、Rustのイディオム
    let x = Arc::new(Mutex::new(100_000)); // 口座に10万入れる
    let x2 = x.clone(); // 参照カウンタをインクリメント
    let x3 = x.clone(); // 参照カウンタをインクリメント

    // スレッドを生成
    let h1 = std::thread::spawn(move || {
        let mut guard = x.lock().unwrap();
        *guard -= 20_000; // 2万引き出す
    });
    // 別でスレッド生成
    let h2 = std::thread::spawn(move || {
        let mut guard = x2.lock().unwrap();
        *guard -= 30_000; // 3万引き出す
    });

    h1.join().unwrap();
    h2.join().unwrap();

    // 残高をprintln!()する
    println!("{}", x3.lock().unwrap());

    println!("===== ReadWriteロック =====");
    read_write_lock();
}

fn read_write_lock() {
    // 美術館を例に、ReadWriteロックを実装してみる
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "富嶽三十六景");
    gallery.insert("ミュシャ", "黄道十二宮");

    // RwLockとArcを利用して共有可能に
    let gallery = Arc::new(RwLock::new(gallery));
    let mut hdls = Vec::new(); // joinハンドラ
    for n in 0..4 {
        // 客を表すスレッドを生成
        let gallery = gallery.clone();
        let hdl = std::thread::spawn(move || {
            for _ in 0..8 {
                {
                    let guard = gallery.read().unwrap(); // Readロック
                    if n == 0 {
                        for (key, value) in guard.iter() {
                            print!("{}: {}, ", key, value);
                        }
                        println!("");
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });

        hdls.push(hdl);
    }

    // 美術館スタッフ
    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            // 展示内容を入れ替える
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap(); // Writeロック
                guard.clear();
                guard.insert("ゴッホ", "ひまわり");
                guard.insert("ミロ", "アポロンの誕生");
            } else {
                let mut guard = gallery.write().unwrap(); // Writeロック
                guard.clear();
                guard.insert("モネ", "睡蓮");
                guard.insert("ミケランジェロ", "ダビデ像");
            }
            sleep(Duration::from_secs(2));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();
}
