///
/// # 参照とスコープとライフタイム
///
/// - 参照は変数を借用することによってのみ存在する
/// - したがって、関数が参照を返す場合、借用元の変数が必ずどこかに存在する
/// - 関数内で宣言された変数は、関数の終了時点でスコープが終了し、破棄されるため、関数内の変数の参照を返すことはできない
/// - よって、関数が参照を返せるのは以下のいずれかの場合のみ
///     - 関数が引数として参照を受け取っている場合
///     - 関数の返り値となる参照が静的なライフタイムを持っている場合
///
fn main() {
    practice_lifetime_parameter();
    practice_static_lifetime_parameter();
}

fn practice_lifetime_parameter() {
    let vec = vec![2, 4, 7, 8, 6, 3, 5];
    let slice = &vec[..];
    let mut increasing_slice = slice;
    for i in 0..slice.len() - 1 {
        if slice[i] >= slice[i + 1] {
            increasing_slice = &slice[..=i];
            break;
        }
    }
    assert_eq!(increasing_slice, &[2, 4, 7, 8]);

    // 同じライフタイムパラメータが付いた参照は、全て同じライフタイムを持つ
    // 「返り値は引数 `slice` と同じライフタイムを持つ」ことを明示することで、返り値のライフタイムを明確にできる
    fn increasing<'a>(slice: &'a [i32]) -> &'a [i32] {
        let mut ret: &'a [i32] = slice;
        for i in 0..slice.len() - 1 {
            if slice[i] >= slice[i + 1] {
                ret = &slice[..=i];
                break;
            }
        }
        ret
    }

    // 引数として &vec を渡し、返り値を result に代入する
    // increasing の引数と返り値のライフタイムは同じ → result のライフタイムは vec のスコープ終了まで
    let vec = vec![2, 4, 7, 8, 6, 3, 5];
    let result = increasing(&vec);
    assert_eq!(result, &[2, 4, 7, 8]);

    // 以下はコンパイルエラー
    // let result;
    // {
    //     let vec = vec![2, 4, 7, 8, 6, 3, 5];
    //     result = increasing(&vec);
    // }
    // assert_eq!(result, &[2, 4, 7, 8]);

    // 参照型の変数のスコープが終了しても、参照型が指している値のライフタイムは存続できる
    let vec = vec![2, 4, 7, 8, 6, 3, 5];
    let result;
    {
        let slice = &vec[..];
        result = increasing(slice);
    }
    assert_eq!(result, &[2, 4, 7, 8]);
}

fn practice_static_lifetime_parameter() {
    fn ordinal_suffix(number: u32) -> &'static str {
        match (number % 10, number % 100) {
            (_, 11..=13) => "th",
            (1, _) => "st",
            (2, _) => "nd",
            (3, _) => "rd",
            _ => "th",
        }
    }

    println!("{}{}", 100, ordinal_suffix(100));
    println!("{}{}", 33, ordinal_suffix(33));
}
