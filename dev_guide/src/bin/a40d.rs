// Smart pointers & interior mutability

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![ItemOrder {
            item: MenuItem::Drink,
            quantity: 1,
        }],
    }
}

type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
struct Chef(Order);
#[derive(Debug)]
struct WaitStaff(Order);
#[derive(Debug)]
struct Accounting(Order);

fn main() {
    let orders: Order = Rc::new(RefCell::new(vec![]));
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = WaitStaff(Rc::clone(&orders));
    let account = Accounting(Rc::clone(&orders));

    let order = new_table_order();
    {
        orders.borrow_mut().push(order);
    }
    dbg!(chef.0.borrow());
    drop(chef);
    dbg!(wait_staff.0.borrow());
    dbg!(account.0.borrow());
}
