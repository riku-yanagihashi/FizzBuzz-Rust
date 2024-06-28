pub fn fizzbuzz1() {
        let mut num = 1; // numに1という値を格納
        while num <= 30 { // numが100以下なら実行する
            if num % 15 == 0 { // numが3で割り切れるなら
                println!("FizzBuzz") // Fizzと出力
            } else if num % 5 == 0 { // 3で割り切れないが、5で割り切れる場合
                println!("Buzz") // Buzzと出力
            } else if num % 3 == 0 {
                println!("Fizz")
            } else { // もしどれにも当てはまらないなら
                println!("{}", num) // numの中身を表示
            }
            num += 1 // numに+1したあとに代入
        }
}
