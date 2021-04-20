/*
컬렉션 형식 사용

튜플 외에도 Rust에는 여러 값을 하나의 단일 형식으로 그룹화할 수 있는 여러 다른 ‘복합 형식’이 있습니다.
*/
/*
배열
배열은 메모리에 순차적으로 저장되는 동일한 형식의 개체 컬렉션입니다. 
*/
/*
fn main() {
    // a comma-separated list inside of brackets
    let _weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
  
    // initialize an array of 512 elements where every element is a zero
    let _byte_buffer = [0_u8; 512];

    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array: {}", letters[0]);  // prints 'a'
    println!("second element of the array: {}", letters[1]); // prints 'b'
}
*/

/*
벡터

배열과 마찬가지로 Vec<T> 형식으로 벡터를 사용하여 동일한 형식의 여러 값을 저장할 수 있습니다. 
배열과 달리 벡터는 언제든지 커지거나 작아질 수 있습니다. 이 기능은 컴파일 타임에 크기가 알려지지 않음을 의미하므로 
Rust에서는 벡터의 잘못된 위치에 액세스하는 문제가 발생할 수 없습니다.

 참고

<T> 구문은 Rust에서 자주 확인할 수 있습니다. 이 항목은 제네릭 형식 매개 변수입니다. Vec<T>를 작성하는 경우 
T 형식으로 구성된 Vec 형식을 나타내는 것입니다. 일반적으로 namewT는 아직 알 수 없는 형식의 형식 이름으로 사용됩니다. 
실제로 벡터를 만들 때는 Vec<u32> 또는 Vec<String> 같은 구체적인 형식을 갖습니다.

*/
/*
fn main() {
    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_numbers);  // prints "[1, 2, 3]"

    // the vec! macro also accepts the same syntax as the array constructor
    let ten_zeroes = vec![0; 10];
    println!("Ten zeroes: {:?}", ten_zeroes); // prints [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    let mut v = Vec::new();  // creates an empty vector,
    v.push(5);               // pushes the number five into it...
    v.push(6);               // ... an then six, and so on
    v.push(7);
    v.push(8);
    println!("{:?}", v); // prints [5, 6, 7, 8]
}
*/

/*
해시 맵
가장 일반적인 컬렉션 중 마지막 컬렉션은 해시 맵입니다. HashMap<K, V> 형식은 K 형식의 키와 V 형식의 값 간 매핑을 저장합니다. 벡터가 정수 인덱스로 값을 저장하는 경우 해시 맵은 키로 값을 저장합니다.

많은 프로그래밍 언어에서 이러한 종류의 데이터 구조를 지원합니다. 이러한 구조는 해시, 맵, 개체, 해시 테이블, 사전 또는 결합형 배열 등의 다양한 이름을 사용합니다.
*/
/*
use std::collections::HashMap;


fn main(){
    let mut book_reviews: HashMap<String, String> = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
        book_reviews.len());
    }

    // Searching for an existing key returns the value associated to it
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // But searching for a nonexisting key will cause a panic
    //println!("Review for Herman: {}", book_reviews["Moby Dick"]);  // panics!

    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);

}
*/




