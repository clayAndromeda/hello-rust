use std::error::Error;
/* 第4章 トレイト trait */
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::path::Path;
use std::{fs::File, io::prelude::*};

use serde::{Deserialize, Serialize};

// 虚数を表す型
struct ImiginaryNumber {
    real: f64,
    img: f64,
}

//ImaginaryNumber型にDisplayトレイトを実装
impl Display for ImiginaryNumber {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

// リストを表す型
#[derive(Debug, Clone, Serialize, Deserialize)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}

impl<T> List<T> {
    fn new() -> Self {
        List::Nil
    }

    // リストを消費して、そのリストの先頭に引数dataを追加したリストを返す
    fn cons(self, data: T) -> Self {
        List::Node {
            data,
            next: Box::new(self),
        }
    }
    // 不変イテレータを返す
    // 'aはライフタイムを表す型
    fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter { elm: self }
    }
}

// 不変イテレータを表す型
struct ListIter<'a, T> {
    elm: &'a List<T>,
}

// std::iter::Iterator traitを実装する
impl<'a, T> Iterator for ListIter<'a, T> {
    // typeキーワードを使い関連型を定義している
    // Itemはイテレータが指す要素の型
    // &'a Tは以下を意味する
    // イテレータが指す要素の型はTの不変参照で、ライフタイムは'aである
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elm {
            List::Node { data, next } => {
                self.elm = next;
                Some(data)
            }
            List::Nil => None,
        }
    }
}

// trait制約を持つ関数
// Mulトレイトを持つ型Tは、乗算演算子*を使える
// Copyトレイトを持つ型Tは、代入がメモリコピーで行われる
fn square<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}

// 静的ディスパッチ・動的ディスパッチの検証
trait Foo {
    fn foo(&self);
}

struct Bar;
// Bar型にFooトレイトを実装
impl Foo for Bar {
    fn foo(&self) {
        println!("Bar::Foo");
    }
}

struct Buzz;
impl Foo for Buzz {
    fn foo(&self) {
        println!("Buzz::Foo");
    }
}

fn main() {
    let num = ImiginaryNumber {
        real: 1.0,
        img: 2.0,
    };
    println!("{num}");

    println!("====================");
    // イテレータの利用例
    let list = List::new().cons(1).cons(2).cons(3);
    for x in list.iter() {
        println!("{}", x);
    }

    println!("====================");
    // jsonにシリアライズ
    let json = serde_json::to_string(&list).unwrap();
    println!("JSON: {} bytes", json.len());
    println!("{}", json);

    let yaml = serde_yaml::to_string(&list).unwrap();
    println!("YAML: {} bytes", yaml.len());
    println!("{}", yaml);

    // MessagePackにシリアライズ
    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes", msgpack.len());
    println!("{:?}", msgpack);

    println!("====================");
    // それぞれデシリアライズ
    let list: List<i32> = serde_json::from_str(&json).unwrap();
    println!("{:?}", list);
    let list: List<i32> = serde_yaml::from_str(&yaml).unwrap();
    println!("{:?}", list);
    let list: List<i32> = rmp_serde::from_read(msgpack.as_slice()).unwrap();
    println!("{:?}", list);

    // シリアライズしたYAMLをファイルに出力してみる
    let path = Path::new("list.yaml");
    let mut f = File::create(path).unwrap();
    f.write_all(yaml.as_bytes()).unwrap();

    // シリアライズしたファイルを読み出して、デシリアライズする
    let mut f = File::open(path).unwrap();
    let mut yaml = String::new();
    f.read_to_string(&mut yaml).unwrap();
    let list: List<i32> = serde_yaml::from_str(&yaml).unwrap();
    println!("{:?}", list);

    println!("====================");
    println!("2^2 = {}", square(2));
    println!("15.5^2 = {}", square(15.5));

    println!("====================");
    // コンパイル時にTが決定する
    fn call_foo_static<T: Foo>(arg: &T) {
        arg.foo();
    }
    // 実行時に呼び出し先が決定する
    fn call_foo_dynamic(arg: &dyn Foo) {
        arg.foo();
    }

    let bar = Bar;
    let buzz = Buzz;

    // 静的ディスパッチ
    call_foo_static(&bar);
    call_foo_static(&buzz);
    // 動的ディスパッチ
    call_foo_dynamic(&bar);
    call_foo_dynamic(&buzz);

    // 動的ディスパッチの実用的な使用例：Errorトレイト
    println!("====================");
    #[derive(Debug)]
    struct ErrorA;
    impl Display for ErrorA {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ErrorA")
        }
    }
    impl Error for ErrorA {}

    #[derive(Debug)]
    struct ErrorB;
    impl Display for ErrorB {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ErrorB")
        }
    }
    impl Error for ErrorB {}

    fn error_a() -> Result<(), ErrorA> {
        Err(ErrorA)
    }
    fn error_b() -> Result<(), ErrorB> {
        Err(ErrorB)
    }

    fn error_ab() -> Result<(), Box<dyn Error>> {
        error_a()?;
        error_b()?;
        Ok(())
    }
    let result = error_ab();
    println!("{:?}", result);
}
