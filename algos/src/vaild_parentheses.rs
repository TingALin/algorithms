use std::collections::HashMap;
#[allow(dead_code)]
pub fn is_vaild(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let match_map = {
        let mut m = HashMap::new();
        m.insert(')', '(');
        m.insert('}', '{');
        m.insert(']', '[');
        m
    };

    let mut stack = vec![];
    for c in s.chars() {
        if let Some(&val) = match_map.get(&c){
            // println!("val: {:?}", val);
            match stack.pop(){
                None => return false,
                Some(pop) => {
                    // println!("pop: {:?}", pop);
                    if pop == val{
                        continue;
                    } else {
                        return false;
                    }
                },
            }
        } else {
            stack.push(c);
            // println!("stack: {:?}", stack);
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn parentheses_test() {
        let s_one = "()".to_string();
        assert_eq!(is_vaild(s_one), true);

        let s_two = "(){}[]".to_string();
        assert_eq!(is_vaild(s_two), true);

        let s_three = "(]".to_string();
        assert_eq!(is_vaild(s_three), false);
    }
}