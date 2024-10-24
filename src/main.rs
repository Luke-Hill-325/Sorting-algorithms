use rand::{thread_rng, Rng};
use std::time::Instant;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rng = thread_rng();
    //will segfault if stack limit is less than 89MB * 5
    let mut i = 0;
    for mut sample in [core::array::from_fn::<u64, 11111110, _>(|_| rng.gen()); 5]
    {
        match i{
            0=>println!("Quick Sort:"),
            1=>println!("Merge Sort:"),
            2=>println!("Heap Sort:"),
            3=>println!("Insertion Sort:"),
            4=>println!("Selection Sort:"),
            _=>(),
        }
        (1..8).fold(&mut sample as &mut[u64], |s, e| -> &mut[u64] {
            let sample_size = 10_usize.pow(e);
            print!("\t{:>8}: ", sample_size);
            let (to_sort, remainder) = s.split_at_mut(sample_size);
            let guard = interrupts::disable();
            let now = Instant::now();
            match i{
                0=> quick_sort(to_sort),
                1=> merge_sort(to_sort),
                2=> heap_sort(to_sort),
                3=> insertion_sort(to_sort),
                4=> selection_sort(to_sort),
                _=>(), 
            }
            let benchmark = now.elapsed();
            drop(guard);
            assert!(to_sort.is_sorted());
            println!("{:?}", benchmark);
            return remainder;
        });
        i += 1;
    }
}

#[inline]
fn selection_sort(arr: &mut[u64]){
    for i in 0..arr.len(){
        arr.swap(i, arr.iter().enumerate().skip(i).reduce(|min, (j, b)| {
            if b < min.1 {
                return (j, b);
            }
            else {return min}
        }).unwrap().0);
    }
}

#[inline]
fn insertion_sort(arr: &mut[u64]){
    for i in 1..arr.len() {
        let insertion = arr[i];
        let mut j = i;
        while j > 0 && insertion < arr[j-1] {
            arr[j] = arr[j-1];
            j -= 1;
        }
        arr[j] = insertion;
    }
}

#[inline]
fn heap_sort(arr: &mut[u64]){
   for i in (0..(arr.len() / 2)).rev() {
       heapify(arr, i);
   }
   for i in (1..arr.len()).rev() {
       arr.swap(0, i);
       heapify(&mut arr[..i], 0);
   }
}
fn heapify(arr: &mut[u64], i: usize){
    let first_child = i * 2 + 1;
    if first_child >= arr.len() {return};
    let second_child = first_child + 1;
    let largest_child = if arr.len() > second_child && arr[second_child] > arr[first_child] {second_child} else {first_child};
    if arr[largest_child] > arr[i] {
        arr.swap(i, largest_child);
        heapify(arr, largest_child);
    }
}
    

fn merge_sort(arr: &mut[u64]){
    if arr.len() <= 1 {return};
    let (left, right) = arr.split_at_mut(arr.len()/2);
    merge_sort(left);
    merge_sort(right);
    let mut l = left.to_owned().into_iter().peekable();
    let mut r = right.to_owned().into_iter().peekable();
    for i in 0..arr.len() {
        if r.peek() == None || (l.peek() != None && l.peek().unwrap() < r.peek().unwrap()) {
            arr[i] = l.next().unwrap();
        } else {
            arr[i] = r.next().unwrap();
        }
    }
}

fn quick_sort(arr: &mut[u64]){
    let part = partition(arr);
    if part.0.len() > 1 {
        quick_sort(part.0);
    }
    if part.1.len() > 2 {
        quick_sort(&mut part.1[1..])
    }
}

#[inline]
fn partition(arr: &mut[u64]) -> (&mut[u64], &mut[u64]) {
    let pivot = arr[0];
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        while arr[i] < pivot {
            i += 1;
        };
        while arr[j] > pivot {
            j -= 1;
        };
        if i < j {
            arr.swap(i,j);
        };
    };
    return arr.split_at_mut(j);
}
