use std::io;

fn main() {
    boat();
}

fn boat() {
    let stdin = io::stdin();
    let mut temp = String::new();
    stdin.read_line(&mut temp).expect("Failed read line.");
    let n: usize = temp.trim().parse().expect("Failed to parse.");
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut t = String::new();
        stdin.read_line(&mut t).expect("Failed read line.");
        a.push(t.trim().parse().expect("Failed to parse."));
    }

    choose_sort(a.as_mut_slice());
    let short_time = sum(n, a.as_mut_slice());
    println!("The shortest time is {}", short_time);
}

fn choose_sort(list: &mut [i32]) {
    let l = list.len();
    for i in 0..l {
        let mut min_index = i;
        for j in i..l {
            if list[min_index] > list[j] {
                min_index = j;
            }
        }

        let temp = list[i];
        list[i] = list[min_index];
        list[min_index] = temp;
    }
}

fn sum(size: usize, list: &[i32]) -> i32 {
    let mut s = 0;
    let mut n = size;
    loop {
        if n < 4 {
            break;
        }

        let temp1 = list[n-1] + list[n-2] + list[0]*2;
        let temp2 = list[n-1] + list[0] + list[1] * 2;
        s += min_num(temp1, temp2);
        n -= 2;
    }

    if n == 3 {
        s += list[0] + list[1] + list[2];
    } else if n == 2 {
        s += list[1];
    } else {
        s += list[0];
    }

    s
}

fn min_num(a: i32, b: i32) -> i32 {
    if a > b {
        return b;
    }
    a
}
