use std::ops::{Add, Sub};

#[derive(Debug)]
struct Account {
    money: u32,
}

impl Account {
    fn add(&mut self, money: u32) {
        self.money = self.money.add(money);
    }

    fn subtract(&mut self, money: u32) {
        self.money = self.money.sub(money);
    }
}

macro_rules! exchange {
    (Give $amount:literal to $name:ident) => {
        $name.add($amount);
    };
    (Take $amount:literal from $name:ident) => {
        $name.subtract($amount);
    };
    (Give $amount:literal from $giver:ident to $receiver:ident) => {
        $giver.subtract($amount);
        $receiver.add($amount);
    };
}

macro_rules! give_money_to_the_poor {
    (Give $example:literal) => {
        println!("How generous");
    };
    (Give 0) => {
        println!("Cheapskate");
    };
}

fn main() {
    let mut the_poor = Account { money: 0 };
    let mut the_rich = Account { money: 1000 };

    exchange!(Give 100 to the_poor);
    exchange!(Take 100 from the_rich);
    exchange!(Give 100 from the_rich to the_poor);

    println!("The poor has: {}", the_poor.money);
    println!("The rich has: {}", the_rich.money);

    give_money_to_the_poor!(Give 0);
}
