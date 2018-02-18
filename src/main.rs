fn main() {
    let x = 2 + 3;
    // x = 8; を書くとエラーになる! immutable!
    println!("{}", x);
    let mut y = 2 + 3; // 警告出るけどmutableに一応出来る。多分あんまりやらない
    y = 8;
    println!("{}", y);

    let mut sokubaku = x; //mutableの変数に束縛出来る
    sokubaku += 2; //ちゃっかり += も出来るんだ！
    println!("{}", sokubaku);

}
