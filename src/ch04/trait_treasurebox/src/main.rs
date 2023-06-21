trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelryBox {
    key_no: i32,
    price: i32,
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }

    fn check(&self) {
        println!("price is {}", self.price);
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key_no: i32) -> bool {
        true
    }

    fn check(&self) {
        println!("damage is {}", self.damage);
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("cannot open");
        return;
    }
    tbox.check();
}

trait TreasureBoxBasicMethod {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }
    fn get_key_no(&self) -> i32;
    fn check(&self);
}

fn main() {
    let box1 = JewelryBox {
        key_no: 1,
        price: 100,
    };
    let box2 = TrapBox { damage: 100 };

    open_box(&box1, 1);
    open_box(&box2, 10);
}
