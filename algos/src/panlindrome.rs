
// pub fn panlindrome(num: i32) -> bool{
//     let num_string = num.to_string();
//     let mut left = 0;
//     let mut right = num_string.len() - 1;

//     while left < right {
//         if num_string.get(left..=left) != num_string.get(right..=right){
//             return false;             
//         }
//         left +=1;
//         right -=1;
   
//     }   
//     true
// }

// pub fn panlindrome(num: i32) -> bool{
//     num.to_string() == num.to_string().chars().rev().collect::<String>()
// }

pub fn panlindrome(x: i32) -> bool{
    if x < 0 {
        return false;
    }
    let mut rev = 0;
    let mut xx = x;
    while xx > 0 {
        rev = 10 * rev + xx % 10;
        xx /= 10;
    }
    rev == x

}