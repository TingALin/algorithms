use std::collections::HashMap;

// O(n) & Hashmap
#[allow(dead_code)]
fn sum_of_two_hashmap(input: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..input.len(){
    //     let wanted = map.get(&(target - input[i]));
    //     match wanted {
    //         Some(index) => return vec![i as i32, *index],
    //         None => map.insert(input[i], i as i32),
    //     };
        if let Some(k) = map.get(&(target - input[i])){
            if *k != i{
                return vec![*k as i32, i as i32]
            }
        }
    map.insert(input[i], i);
    }
    panic!("Not found")
    // Vec::new()

    // for (index, value) in input.iter().enumerate(){
    //     if let Some(k) = map.get(&(target - value)){
    //         return vec![index as i32, *k as i32];
    //     }
    //     map.insert(*value, index);
    // }
    // unreachable!()
}

// O(nlogn) & Two pointers
#[allow(dead_code)]
fn sum_of_two_two_pointers(input: &mut Vec<i32>, target: i32) -> Vec<i32> {
    input.sort();
    let mut left = 0;
    let mut right = input.len()-1;
    while left < right {
        if input[left] + input[right] == target {
            return vec![input[left], input[right]];
        } else if input[left] + input[right] > target{
            right -= 1
        } else {
            left += 1
        }
    }
    unreachable!()
}

// O(n^2) & Array
#[allow(dead_code)]
fn sum_of_three_array(arr: Vec<i32>, target: i32) -> (i32, i32, i32) {
    for fist_layer in 0..arr.len(){
        for second_layer in 1..arr.len(){
            for third_layer in 2..arr.len(){
                if arr[fist_layer] + arr[second_layer] + arr[third_layer] == target{
                    return (fist_layer as i32, second_layer as i32, third_layer as i32)
                }
            }
        }
    }
    // unreachable!()
    (0, 0, 0)
}

// Two pointers
#[allow(dead_code)]
fn sum_of_three_two_pointer(arr: Vec<i32>, target: i32) -> Vec<i32> {
    assert!(arr.len() >= 3);
    let arr = &mut {arr};
    arr.sort();

    let mut i = 0;
    while i < arr.len(){
        if i > 0 && arr[i] == arr[i -1] {
            i +=1;
            continue;
        }
        let mut j = i + 1;
        let mut k = arr.len() - 1;
        while j < k {
            if j > i + 1 && arr[j] == arr[j-1]{
                j += 1;
                continue;
            }
            if k < arr.len() - 1 && arr[k] == arr[k+1]{
                k -=1;
                continue;
            }
            let sum = arr[i] as i64 + arr[j] as i64 + arr[k] as i64;
            if sum == target as i64 {
                return vec![arr[i], arr[j], arr[k]];
            } else {
                if sum < target as i64 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        i+=1
    }
    unreachable!()
}

#[cfg(test)]
mod sum_tests {
    use super::*;

    #[test]
    fn sum_array() {
        let nums = vec![2,3,4,5];
        let expected = (0, 1, 2);
        assert_eq!(sum_of_three_array(nums, 9), expected);

        assert_eq!(sum_of_three_array(vec![0,0,0], 9), (0, 0, 0));
    }
}