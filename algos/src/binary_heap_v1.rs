use std::marker::PhantomData;
// 下整个REPO？
// use support::StorageValue;
// https://github.com/yz89/substrate-heap/blob/master/runtime/src/heap.rs
// 参考整体用法和结构, 没写是MIN/MAX
// https://www.cnblogs.com/741162830qq/p/16994193.html

pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug + TypeInfo { }

// 用trait还是直接impl?
pub trait Compare {
    type A;
    // calculate the position without knowing the root没有实现
    fn close_than(x: &Self::A, y: &Self::A) -> bool;
}

// Min-heap
struct Heap<T,C,S>(PhantomData<(T,C,S)>);

impl<T,C,S> Heap<T,C,S>
where T: Parameter, C: Compare<A=T>, S: StorageValue<Vec<T>, Query=Vec<T>>,
{
     /// Push a value into heap and update storage.
     pub fn push(item: T) {
        let mut store = S::get();
        Self::push_into_store(&mut store, item);
        S::put(store);
    }

    /// Push a vector into heap and update the storage.
    pub fn push_vec(items: Vec<T>) {
        let mut store = S::get();
        for item in items {
            Self::push_into_store(&mut store, item);
        }
        S::put(store);
    }

    /// Pop the top element of heap and update the storage.
    pub fn pop() -> Option<T> {
        let mut store = S::get();
        let top = Self::pop_from_store(&mut store);
        S::put(store);
        top
    }

    fn push_into_store(store: &mut Vec<T>, item: T) {
        store.push(item);
        let last = store.len() - 1;
        Self::shift_up(store, last);
    }

    pub fn pop_vec(stake: &T) -> Vec<T> {
        let mut store = S::get();
        let vec = Self::pop_by_stake(&mut store, stake);
        S::put(store);
        vec
    }

    //??
    fn pop_by_stake(store: &mut Vec<T>, stack: &T) -> Vec<T> {
        let mut vec = Vec::new();
        let peek_top = store.get(0);
        match peek_top {
            None => vec,
            Some(peek_top) => {
                if C::closer_than(peek_top, stack) {
                    let top = Self::pop_from_store(store);
                    match top {
                        None => vec,
                        Some(top) => {
                            vec.push(top);
                            vec.append(&mut Self::pop_by_stake(store, stack));
                            vec
                        }
                    }
                } else {
                    vec
                }
            }
        }
    }

    fn pop_from_store(store: &mut Vec<T>) -> Option<T> {
        match store.len() {
            0 => None,
            1 => store.pop(),
            _ => {
                let last = store.len() - 1;
                store.swap(0, last);
                let top = store.pop();
                Self::shift_down(store, 0);
                top
            }
        }
    }

    // close_than没实现, 考虑下为什么用C
    fn shift_up(store: &mut [T], idx: usize) {
        match Self::parent_idx(idx) {
            None => {},
            Some(node) => {
                if C::close_than(&store[left], &store[node]){
                    store.swap(idx, node);
                    Self::shift_up(store, node);
                }
            }
        }
    }

    // better use case on this?
    fn shift_down(store: &mut [T], idx: usize) {
        match Self::left_idx(store, idx) {
            None => {},
            Some(left) => {
                // 为什么？
                match Self::right_idx(store, idx) {
                    None => {
                        if C::close_than(&store[left], &store[idx]){
                            store.swap(idx, left);
                            Self::shift_down(store, left);
                        }
                    },
                    Some(right) => {
                        let closer =
                            if C::closer_than(&store[left], &store[right]) {
                                left
                            } else {
                                right
                            };
                        if C::closer_than(&store[closer], &store[idx]) {
                            store.swap(idx, closer);
                            Self::shift_down(store, closer);
                        }
                    },
                }
            }
        }
    }

    fn parent_idx(child: usize) -> Option<usize> {
        match child {
            0 => None,
            1..2 => Some(0),
            _ => {
                if child % 2 == 1{
                    Some((child -1)/2)
                } else {
                    Some((child -2)/2)
                }
            }
        }
    }

    // &[T]
    fn left_idx(store: &[T], parent: usize) -> Option<usize> {
        let left: usize = parent * 2 + 1;
        if left < store.len() {
            Some(left)
        } else {
            None
        }
    }

    fn right_idx(store: &[T], parent: usize) -> Option<usize> {
        let right: usize = parent * 2 + 2;
        if right < store.len() {
            Some(right)
        } else {
            None
        }
    }
}