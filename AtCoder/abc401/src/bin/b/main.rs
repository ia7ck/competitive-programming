use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    };

    let mut error = 0;
    let mut login = false;
    for s in ss {
        if s == "login" {
            login = true;
        } else if s == "logout" {
            login = false;
        } else if s == "public" {

        } else if s == "private" {
            if !login {
                error += 1;
            }
        } else {
            unreachable!();
        }
    }

    println!("{}", error);
}
