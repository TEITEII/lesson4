pub fn main() {
    let data:[u32;6] = [6458,68689,8787,8426,7359779,97]; 
    total(&data);
}    

pub fn total(item:&[u32]) -> Option<u32>{
    let _s:u32 = item.iter().sum();
    println!("The sum of data is {}",&_s);
    return Some(_s);
}