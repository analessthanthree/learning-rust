use std::io;
use std::io::Write;

fn f_to_c() {
    let mut temp = String::new();

    print!("Convert temperature in F to C!\nF: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = temp.trim().parse().expect("Failed to parse");
    let temp_c = 5.0/9.0*(temp - 32.0);

    println!("{temp}F is {temp_c}C");

}

fn fibonacci() {
    let mut n = String::new();

    print!("Generate the n'th fibonacci number!\nn: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u64 = n.trim().parse().expect("Failed to parse");
    
    let mut i: u64 = 1;

    let mut fib_minus: u64 = 0;
    let mut fib: u64 = 1;
    let mut fib_plus: u64;
    
    loop {
        if n == 0 {
            fib = fib_minus;
        }
        else if n == 1 {
            break
        }
        else if i < n {
            fib_plus = fib_minus + fib;
            fib_minus = fib;
            fib = fib_plus;
            i +=1;
        }
        if i >= n {
            break
        }
    }
    println!("{fib}")
}

fn days_of_christmas() {

    // Array of str reference tupels?
    
    const DAYS_AND_GIFTS: [(&str, &str); 12] = [
        ("first","A partridge in a pear tree"),
        ("second","Two turtle doves"),
        ("third","Three French hens"),
        ("fourth","Four calling birds"),
        ("fifth","Five golden rings"),
        ("sixth","Six geese a-laying"),
        ("seventh","Seven swans a-swimming"),
        ("eighth","Eight maids a-milking"),
        ("ninth","Nine ladies dancing"),
        ("tenth","Ten lords a-leaping"),
        ("eleventh","Eleven pipers piping"),
        ("twelth","Twelve drummers drumming")
    ];

    let mut christmas_spirit = String::new();

    print!("Are you in the Christmas spirit today? [y/N]: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut christmas_spirit)
        .expect("Failed to read line");

    let christmas_spirit: bool = if christmas_spirit.trim() == "y" { true } else { false };

    if christmas_spirit {
        for (i,day) in DAYS_AND_GIFTS.iter().enumerate() {
            println!("On the {0} day of Christmas my true love gave to me", day.0);
            for j in (1..=i).rev() {
                println!("{0},", DAYS_AND_GIFTS[j].1);
            }
            if i == 0 {
                println!("{0}!", DAYS_AND_GIFTS[0].1);
            }
            else {
                println!("And {0}!", DAYS_AND_GIFTS[0].1.to_lowercase());
            }
            println!("");
        }
    }
}

fn main() {
    f_to_c();
    fibonacci();
    days_of_christmas();
}
