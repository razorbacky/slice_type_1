[Rust] Slice Type 1
======

## 슬라이스(Slice)

슬라이스는 컬렉션(Collection)을 통째로 참조하는 것이 아닌, 컬렉션의 연속된 일련의 요소를 참조하도록 해준다.

슬라이스는 참조자의 일종으로서 소유권을 갖지 않는다.

-----

공백 문자로 구분된 단어들의 문자열을 입력 받아 해당 문자열의 첫번째 단어를 변환하는 함수 만들기
공백 문자를 찾을 수 없으면 문자열 전체가 하나의 단어라는 뜻이니 전체 문자열을 반환

```rust
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes,iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s); // word 는 값 5를 받는다.

  s.clear(); // 이 코드는 String을 비워서 "" 으로 만든다.

  // 여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는 문자열이 없다. word는 이제 유효하지 않다.
}

```

**이해를 돕기 위한 설명 (내가 추가한 것이므로 정확하지 않을 수 있음)**

first_word 함수는 문자열에 대해 단순히 정수(usize) 값으로 변환하여 반환하였으므로
s.clear() 를 통해 문자열(String)을 비워도, 정수 변환으로 반환 받은 "5" 라는 값은 여전히 남기 때문에
word를 출력하면 단순히 "5" 만 출력됨.

그러나, clear를 통해 문자열은 이미 비어있기 때문에 인덱스 5 는 유요하지 않음. (함께 비워져야 하나, 정수값을 반환 받았기 때문으로 추정)

:: 결론, first_word로 String을 가져와서 변환했는데, 그 이후에 String 값이 바뀐 것! 이것이 "논리적 오류" 이다.

* **s.as_byes() -> &[u8] 바이트 배열을 가져옴**
* **.iter().enumerate() -> 바이트 배열을 (index, value) 쌍으로 반복**
* **item == b' ' -> ASCII 공백(스페이스)인지 확인 (b' ' 는 바이트 리터럴)**
* **return i; -> 공백을 찾으면 해당 인덱스 반환**
* **공백이 없는 경우 문자열의 전체 길이(s.len()) 반환**
