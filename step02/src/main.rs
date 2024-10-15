use std::{
    collections::{BTreeMap, LinkedList},
    time,
};

fn main() {
    println!("===== generics =====");
    enum List<T> {
        Node { data: T, next: Box<List<T>> },
        Nil,
    }
    let n1 = List::<u32>::Nil;
    let n2 = List::<u32>::Node {
        data: 10,
        next: Box::<List<u32>>::new(n1),
    };
    let n3 = List::Node {
        data: 40,
        next: Box::new(n2),
    };

    println!("===== Closure =====");
    let f = |a, b| a + b;
    let n = f(10, 20);
    let mut s = Storage::SSD(512);
    let mut f = || match &mut s {
        Storage::HDD { size: s, .. } => *s += 64,
        _ => (),
    };

    println!("===== Methods =====");
    impl Storage {
        fn get_size(&self) -> u32 {
            match self {
                Storage::HDD { size, .. } => *size,
                Storage::SSD(size) => *size,
            }
        }
    }

    println!("===== sequence =====");
    let mut list1 = LinkedList::new();
    list1.push_back(0);
    list1.push_back(1);
    list1.push_back(2);

    println!("===== map =====");
    let mut m = BTreeMap::new(); // B木で実装されたマップ
    m.insert(1, "apple");
    m.insert(2, "orange");
    m.insert(3, "banana");
    m.insert(4, "grape");
    m.insert(5, "peach");
    if let Some(x) = m.get(&2) {
        println!("m[2] = {}", x);
    }
    for (n, s) in m.iter() {
        println!("n = {}, s = {}", n, s);
    }

    println!("===== 並列ソートの実装例 =====");
    let (mut v1, mut v2) = randomized_vec();
    let start = std::time::Instant::now(); // 現在時刻
    v1.sort();
    v2.sort();
    let end = start.elapsed(); // 経過時間
    println!("elapsed = {:?}", end);

    let (mut v1, mut v2) = randomized_vec();
    let start = std::time::Instant::now(); // 現在時刻
    let handler1 = std::thread::spawn(move || {
        v1.sort();
        v1
    });
    let handler2 = std::thread::spawn(move || {
        v2.sort();
        v2
    });
    let _v1 = handler1.join().unwrap(); // unwrap()はOKで帰ってくることを前提に値を取り出す。Errならpanic!
    let _v2 = handler2.join().unwrap();
    let end = start.elapsed(); // 経過時間
    println!("elapsed = {:?}", end);
}

enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

struct XOR64 {
    x: u64,
} // xorshiftで疑似乱数を生成する
impl XOR64 {
    fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88172645463325252,
        }
    }
    fn next(&mut self) -> u64 {
        self.x ^= self.x << 13;
        self.x ^= self.x >> 7;
        self.x ^= self.x << 17;
        self.x
    }
}

fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut generator = XOR64::new(1234);
    for _ in 0..50000000 {
        let x1 = generator.next();
        let x2 = generator.next();
        v1.push(x1);
        v2.push(x2);
    }
    (v1, v2)
}
