// Slice Type 1 . 25.02.14
// 슬라이스 타입

/* 슬라이스(Slice)는 컬렉션(Collection)을 통째로 참조하는 것이 아니라,
컬렉션의 연속된 일련의 요소를 참조하도록 해준다. 슬라이스는 참조자의 일종으로서 소유권을 갖지 않는다.
*/

// 공백문자로 구분된 단어들의 문자열을 입력 받아 첫 번째 단어를 반환하는 함수

// String 매개변수의 바이트 인덱스 값을 반환하는 first_word 함수
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // String을 각각 분리하여 공백 값인지 확인을 위해 as_byes 매서드로 바이트 배열로 변환

    for (i, &item) in bytes.iter().enumerate() {
        // 바이트 배열에 사용할 반복자 (iterator)를 iter 매서드로 생성
        // 반복자에 대한 내용은 공식 문서 13장에서 확인
        // iter 매서드는 컬렉션의 각 요소를 반환하고
        // enumerate 매서드는 iter 의 각 결괏값을 튜플로 감싸준다.
        // 이 때 반환하는 튜플의 첫 번째 요소는 인덱스, 두 번째 요소가 해당 요소의 참조자이다.

        // enumerate 매서드가 반환한 튜플은 패턴을 이용해 해제한다.
        // 패턴은 공식 문서 6장

        // for 루프 내에서 i 는 튜플 요소 중에 인데스에 대응하고, &item 은 바이트에 대응
        // 이 때 패턴에 &를 사용하는 이유는 iter().enumerate() 에서 얻은 요소의 참조가 필요.

        if item == b' ' {
            return i;
        }
    }
    s.len()
    /* for 반복문 내에서 바이트 리터럴 문법으로 공백 문자를 나타내느 바이트를 찾고,
    찾으면 해당 위치를 반화한다. 찾지 못한 경우, s.len() 으로 문자열의 길이를 반환한다. */
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 값 5를 받는다.

    s.clear(); // 이 코드는 String을 비워서 "" 으로 만든다.

    // 여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는 문자열은 더 이상 없다.
    // word는 이제 전혀 유효하지 않다.

    println!("{s}");
    println!("{word}");
}
