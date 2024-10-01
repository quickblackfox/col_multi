use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;

fn col_steps(mut n: i64) -> i64 {
    let mut c = 0;
    while n > 1 {
        n = match n % 2 {
            0 => n / 2,
            _ => 3 * n + 1
        };
        c += 1;
    }
    c
}

fn main() {
    let t1 = thread::spawn(move || {
        let path = Path::new("t1.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 2;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t2 = thread::spawn(move || {
        let path = Path::new("t2.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 3;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t3 = thread::spawn(move || {
        let path = Path::new("t3.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 4;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t4 = thread::spawn(move || {
        let path = Path::new("t4.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 5;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t5 = thread::spawn(move || {
        let path = Path::new("t5.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 6;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t6 = thread::spawn(move || {
        let path = Path::new("t6.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 7;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t7 = thread::spawn(move || {
        let path = Path::new("t7.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 8;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t8 = thread::spawn(move || {
        let path = Path::new("t8.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 9;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t9 = thread::spawn(move || {
        let path = Path::new("t9.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 10;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    let t10 = thread::spawn(move || {
        let path = Path::new("t10.txt");
        let mut f = File::create(&path).unwrap();

        let mut biggest_n = 0;
        let mut biggest_steps = 0;
        let mut i = 11;
        loop {
            let next = col_steps(i);
            if next >= biggest_steps {
                biggest_n = i;
                biggest_steps = next;
                let output = format!("biggest n = {}, steps = {}\n", biggest_n, biggest_steps);
                f.write_all(output.as_ref()).unwrap();
            }
            i += 10;
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
    t6.join().unwrap();
    t7.join().unwrap();
    t8.join().unwrap();
    t9.join().unwrap();
    t10.join().unwrap();
}
