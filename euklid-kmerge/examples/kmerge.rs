use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq)]
struct Item<'a> {
    arr: &'a Vec<i32>,
    idx: usize,
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_item() == other.get_item()
    }
}

impl<'a> PartialOrd for Item<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_item().partial_cmp(&other.get_item())
    }
}

impl<'a> Ord for Item<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_item().cmp(&other.get_item())
    }
}

impl<'a> Item<'a> {
    fn new(arr: &'a Vec<i32>, idx: usize) -> Self {
        Self { arr, idx }
    }

    fn get_item(&self) -> i32 {
        self.arr[self.idx]
    }
}

fn merge(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sorted = vec![];

    let mut heap = BinaryHeap::with_capacity(arrays.len());

    // From each vector we build an item which
    // stores the array and the current index.
    for arr in &arrays {
        let item = Item::new(arr, 0);
        println!("item={:?}", item);
        heap.push(Reverse(item));
    }

    while !heap.is_empty() {
        // Get the item with the smallest head.
        let mut it = heap.pop().unwrap();
        println!("poped: {:?}", it.0);
        sorted.push(it.0.get_item());

        // Increment the index
        it.0.idx += 1;

        // If there are still elements in the array
        // we push back the item in the heap, so the
        // heap is reordered.
        if it.0.idx < it.0.arr.len() {
            heap.push(it)
        }
    }

    sorted
}

fn main() {
    let a = vec![1, 5, 7];
    let b = vec![-2, 3, 4];
    let v = vec![a, b];
    dbg!(merge(v));
}
