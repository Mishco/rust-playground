fn main() {
    println!("Hello, world!");

    let x = plus_one(5);

    println!("The value of x is: {x}");

    print_labeled_measurement(5, 'h');

    let number = 3;

    if number != 0 {
        println!("number was three");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is : {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result} ");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    println!("0F   \t {} °C", convert_from_fahreinheit_to_celsius(0));
    println!("37F  \t {} °C", convert_from_fahreinheit_to_celsius(37));
    println!("99F \t {} °C", convert_from_fahreinheit_to_celsius(99));
    println!("100F \t {} °C", convert_from_fahreinheit_to_celsius(100));

    println!(
        "100°C \t=> {} °C",
        convert_from_celsius_to_fahrenteit(100.0)
    );
    println!("37°C  \t=> {} °C", convert_from_celsius_to_fahrenteit(37.0));
    println!("0°C   \t=> {} °C", convert_from_celsius_to_fahrenteit(0.0));

    println!("{}", fibb(5));
    println!("{}", fibb(10));
    println!("{}", fibb(11));
    println!("{}", fibb(12));



    let mut s = String::from("hello");
    
    change(&mut s);
    println!("{}", s);

    let r1 = &s;
    let r2 = &s;

    println!("{} , {}", r1, r2);


}

fn change(some_string: &mut String) {
    some_string.push_str(", svet");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measuremenet is : {x}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Fahrenheit to Celsius
fn convert_from_fahreinheit_to_celsius(x: i32) -> i32 {
    // F = (°C × (9/5)) + 32
    ((f64::from(x) - 32.0) / (9.0 / 5.0)) as i32
}

// Celsius to Fahrenheit
fn convert_from_celsius_to_fahrenteit(x: f64) -> f64 {
    // °F = °C * 9/5 + 32
    (f64::from(x) * (9.0 / 5.0)) + 32.0
}

// Generate the nth Fibonacci number.
fn fibb(x: i64) -> i64 {
    if x < 0 {
        println!("not valid input");
        return -1;
    } else if x == 1 {
        return 0;
    } else if x == 2 {
        return 1;
    } else {
        return fibb(x - 1) + fibb(x - 2);
    }
}

#[test]
fn test() {
    assert_eq!(convert_from_celsius_to_fahrenteit(21.5), 70.7);
}

#[test]
fn test2() {
    assert_eq!(convert_from_fahreinheit_to_celsius(100), 37);
}

#[test]
fn test_fibb() {
    assert_eq!(fibb(13), 144);
}
