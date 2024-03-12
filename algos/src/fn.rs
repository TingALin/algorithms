fn for_each_planet<F>(f: F)
where
    F: Fn(&'static str) + 'static,
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn foobar<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("{}", f(f(2)));
}

fn foobar_mut<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    let temp = f(2);
    println!("{}", f(temp));
}

fn foobar_once<F>(f: F)
where
    F: FnOnce() -> String,
{
    println!("{}", f());
}

fn foobar_two<F>(x: i32, y: i32, is_greater: F)
where
    F: Fn(i32, i32) -> bool,
{
    let (greater, smaller) = if is_greater(x, y) { (x, y) } else { (y, x) };
    println!("{} is greater than {}", greater, smaller);
}

fn countdown<F>(count: usize, tick: F)
where
    F: Fn(usize),
{
    for i in (1..=count).rev() {
        tick(i);
    }
}

fn make_tester<'a>(answer:&'a str) -> impl Fn(&str)->bool + 'a{
    move |c| {c == answer}
}

fn is_palindrome(number: i32) -> bool{
    // let n= number.to_string();
    // let n_rev = n.as_str().chars().rev().collect::<String>();
    // n == n_rev

    // x.to_string().chars().rev().eq(x.to_string().chars())
    
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
        }
        let mut n = 0;
        let mut num = x;

        while num > 0 {
            n = n * 10 + num % 10;
            num /= 10;
        }
        match x == n {
            true => return true,
            false => return false,
    }
}

fn main() {
    // print!("{:?}", is_palindrome(121));
    // print!("{:?}", is_palindrome(123));

   
    //     let greeting = String::from("Good to see you");
    //     for_each_planet(move |planet| {println!("{:?}, {:?}", greeting, planet)});

    //    foobar(|x| x*2);

    //    foobar_mut(|x| x*2);
    //    let mut acc = 2;
    //    foobar_mut(|x| {acc +=1; x*acc});

    //    let s = String::from("alright");

    //     // foobar_once(move || s);
    //     foobar_once(|| s.clone());
    //     foobar_once(move || s);

    //     foobar_two(3, 64, |x,y| x>y);
    //     foobar_two(32, 64, |_, _| panic!("Comparing is futile!"));

    // countdown(3, |t| println!("{:?}", t));
    // countdown(3, |_| ());//a toilet closure

    // for c in "SuRPRISE INbOUND"
    //     .chars()
    //     .filter(|x| x.is_lowercase())
    //     .flat_map(|c| c.to_uppercase())
    // {
    //     print!("{}", c);
    // }
    // println!(); //加了就没%

    // let test = make_tester("hunter".into());
    // println!("{}", test("xxx"));
    // println!("{}", test("hunter"));

    // let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
    // .iter()
    // .map(|x| x + 3)
    // .fold(0, |x, y| x + y);
    // println!("{}", x);
}
