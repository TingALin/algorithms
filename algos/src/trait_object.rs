// #[warn(dead_code)]
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
 // print_type_of(&x);

trait Echo {
    type Content: std::fmt::Display; // trait 中的关联类型

    fn echo(&self) {
        // 拥有一个默认实现，可以在为其他类型实现时不实现这个方法
        println!(
            "echo content is {}",
            self.echo_content()
        );
    }
    fn echo_content(&self) -> Self::Content;
}
fn test(e: &dyn Echo<Content = i64>) {
    e.echo();
}

fn main(){
    println!(
        "{}",
        std::mem::size_of::<&'static dyn Echo<Content = i64>>()
    );
}




use std::{path::Path, io};

struct Name<T: Display>(T);
enum Result<T,E> {
    Ok(T),
    Err(E),
}

impl <T: Debug + Default, D: Drop + Display> Mytrait<T,D> for u32{}

impl <T,D> Mytrait <T,D> for u32
where
T:Debug + Default,
D:Drop + Display, {}

impl<T> PrintOption for T
where Option<T>:Debug
{}

struct Container(i32, i32);
trait Contains<A, B> {
   fn contains(&self, _: &A, _: &B) -> bool; 
   fn first(&self) -> i32; 
   fn last(&self) -> i32; 
}
impl Contains<i32, i32> for Container{}

fn area<T: Debug+ Display>(t:&T, u:&Display){}
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {}
fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output{}

fn do_something<T: Display>(val: T) { 
    println!("{}", val);
}
fn print(val: impl Display) { 
    println!("{}", val);
}
fn print<S>(val: S) where
S: Display{}
fn bol_then<T>(s:bool, f: impl fnOnce()-> T)->Option<T>{
    if s{
       Some(f()) 
    } else{
        None
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T){}
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

trait Config: frame_system::Config{}
trait Supertrait: Water + Land{}
//前提是Summary这个TRAIT是被两个STRUCT实现了
fn do_something(h: &dyn Summary){}
fn do_something(h: impl Summary){}
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

//associted type 不能用多于一个Impl,generic 可以用多个
struct Foo<T>(T);
struct Bar<T: ?Sized>(T);//允许该类型是确定大小(sized)或者不确定大小(unsized)
//Rust里有两种不确定大小的类类型 ： slice 和 dyn trait/不确定大小类型(unsized type)或者DST，即动态大小类型(Dynamically-Sized Type)。因为不确定大小类型(unsized type)不能存放在栈上，所以它们只能通过传引用(by reference)的方式来传递
//The term "fat pointer" is used to refer to references and raw pointers to dynamically sized types (DSTs) – slices or trait objects
struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
struct BarUse(Bar<[i32]>); // OK

fn func<T: ?Sized>(t: T) {} // compile error
// ...which doesn't compile since t doesn't have
// a known size so we must put it behind a pointer...
fn func<T: ?Sized>(t: &T) {} // compiles
fn func<T: ?Sized>(t: Box<T>) {} // compiles

fn main() {
    
}