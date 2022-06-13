fn main() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,8];
    let mut i = 0;
    let mut j = 0;
    while i < org_arr.len() && j < sub_arr.len(){
        if org_arr[i] == sub_arr[j] {
            i += 1;
            j += 1;
            if j == sub_arr.len(){
                println!("found");
                break;
            }
        } else {
            i = i - j +1;
            j = 0;
        }
        
    }
    if j != sub_arr.len() {
        println!("false");
    }

}

