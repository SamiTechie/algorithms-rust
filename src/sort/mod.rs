pub fn quick_sort(list: &mut [u32]){
    if list.len() < 2{
        return ;
    }
    let pivot_index = partition(list);
    quick_sort(&mut list[0..pivot_index]);
    quick_sort(&mut list[pivot_index+1..]);
}
fn partition(list: &mut[u32]) -> usize{
   let len = list.len();
   let pivot_index: usize = len / 2;
   list.swap(pivot_index, len - 1);
   let mut  i = 0;
   for j in 0..len {
       if list[j] < list[len -1]{
            list.swap(i, j);
            i +=1;
       }
   }
   list.swap(i, len -1);
   i
}
