// rustはスネークケース
fn square_sum(n: isize) -> isize { //　いまだわからないisizeとか
    (0..n).  // この書き方出来るのは美しい rubyみたい
        filter(|i| i % 2 == 0) // なんでこういう書き方になったのかわからん。。。
        .map(|i| i * i) // scala でいう .map(i => i * i) みたいな感じ
        .sum() //return いらない
}

fn main() {
    println!("{}", square_sum(20))
}
