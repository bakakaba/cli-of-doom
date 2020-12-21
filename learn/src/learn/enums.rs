use colored::*;

pub fn enums() {
    println!("\n{}", "Enums".green().bold());

    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn callz(&self) {
            match self {
                Message::Write(value) => println!("message={}", value),
                _ => println!("unsupported")
            }

        }
    }

    let m = Message::Write(String::from("hello"));
    m.callz();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;

    enum Coin {
        Penny,
        _Nickel,
        _Dime,
        _Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::_Nickel => 5,
            Coin::_Dime => 10,
            Coin::_Quarter => 25,
        }
    }

    value_in_cents(Coin::Penny);

    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}