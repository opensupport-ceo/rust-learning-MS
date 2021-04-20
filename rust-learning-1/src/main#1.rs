/*
변수 만들기 및 사용
완료됨
100 XP
4분
let 키워드를 사용하여 변수에 값을 바인딩할 수 있습니다.
*/
/*
fn main() {
    let a_number = 10;
    let a_boolean = true;
  
    println!("the number is {}.", a_number);
    println!("the boolean is {}.", a_boolean);
  }
*/

/*
  가변성
Rust에서는 기본적으로 변수 바인딩을 변경할 수 없습니다. 변수를 변경할 수 없는 경우 값을 이름에 바인딩한 후에는 해당 값을 변경할 수 없습니다.

예를 들어, 이전 예제의 숫자 값을 변경하려고 하면 컴파일러에서 오류 메시지가 표시됩니다.
*/
/*
fn main() {
    let a_number = 10; // error: cannot assign twice to immutable variable `a_number`
    println!("the number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number);
  }
*/

/*
값을 변경하려면 먼저 mut 키워드를 사용하여 변수 바인딩을 변경할 수 있도록 설정해야 합니다.
*/
/*
fn main() {
    let mut a_number = 10; // notice the `mut` keyword
    println!("the number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number);
}
*/

/*
숫자
Rust의 정수는 비트 크기 및 부호 있는 속성으로 식별할 수 있습니다. 부호 있는 정수는 양수 및 음수를 나타낼 수 있습니다. 부호 없는 정수는 양수만 나타낼 수 있습니다.

숫자
길이	서명	부호 없음
8비트	i8	u8
16비트	i16	u16
32비트	i32	u32
64비트	i64	u64
128비트	i128	u128
아치	isize	usize
또한 isize 및 usize 형식은 프로그램을 실행하는 컴퓨터의 종류에 따라 달라집니다. 64비트 아키텍처에 있으면 64비트, 32비트 아키텍처에 있으면 32비트입니다. 지정하지 않을 때마다 정수에 할당된 기본 형식이 지정됩니다.

Rust의 부동 소수점 형식은 각각 32비트 및 64비트 크기의 f32 및 f64입니다.

최신 CPU에서는 속도가 대략 f32와 같지만 전체 자릿수는 더 클 수 있으므로 기본 형식은 f64입니다.
*/
/*
fn main(){
    let _x = 2.0;      // f64, default type
    let _y: f32 = 3.0; // f32, via type annotation

    //모든 Rust의 기본 숫자 형식은 더하기, 빼기, 곱하기 및 나누기와 같은 수학 연산을 지원합니다.
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // Multiplication
    println!("3 * 6 = {}", 3 * 6)
}
*/

/*
문자 및 문자열
Rust에는 두 개의 문자열 형식과 하나의 문자 형식이 있습니다. 모든 항목은 유효한 UTF-8 표현입니다.

char 형식은 가장 기본적인 형식이며 작은따옴표로 지정합니다.

String과 &str 간의 차이점을 완전히 이해하려면 Rust의 소유권 및 대여 시스템에 대해 알아보아야 합니다. 
그때까지 String 데이터는 프로그램이 실행될 때 변경될 수 있는 문자열 데이터로, &str은 프로그램이 실행될 때 변경되지 않는 
문자열 데이터의 변경할 수 없는 뷰로 생각하면 됩니다.

*/
/*
fn main(){
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart: {}", c, z, heart_eyed_cat);

    let mut hello = String::from("Hello, ");  // create a String from a string literal
    hello.push('w');                          // push a character into our String
    hello.push_str("orld!");                  // push a string literal into our String
    println!("{}", hello)
}
*/

/*
튜플
튜플은 하나의 복합으로 수집되는 다양한 형식의 값을 그룹화한 것입니다. 고정 길이를 가집니다. 
즉, 선언된 후 크기를 늘리거나 줄일 수 없습니다. 튜플의 형식은 각 멤버의 형식 시퀀스로 정의됩니다.

assert_eq! 매크로는 두 식이 서로 같은지 확인합니다.

튜플은 여러 형식을 단일 값으로 결합하려는 경우에 유용합니다. 예를 들어, 튜플은 많은 값을 포함할 수 있으므로 함수에서 
튜플을 사용하여 여러 값을 반환할 수 있습니다.

*/
/*
fn main() {
    let tuple = ("hello", 5, 'c');
  
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
  }
  */

  /*
  구조체 및 열거형에 대해 알아보기
완료됨
100 XP
4분
구조체는 다른 형식으로 구성된 형식입니다. 튜플처럼 구조체의 부분은 여러 다른 형식일 수 있지만, 값의 의미를 명확하게 하기 위해 각 데이터 부분에 이름을 지정할 수 있습니다.

Rust의 구조체는 3가지 버전으로 제공됩니다. 클래식 구조체, 튜플 구조체 및 단위 구조체가 있습니다.
*/
/*
// A struct with named fields
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

// A tuple struct
struct Point2D(u32, u32);

// A unit struct
struct Unit;

fn main() {
    // Instantiate a classic struct, with named fields. Order does not matter.
    let _person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    // Instantiate a tuple struct by passing the values in the same order as defined.
    let _origin = Point2D(0, 0);

    // Instantiate a unit struct.
    let _unit = Unit;

    //Error!
    //println!("_person: {:?}, _origin: {:?}, _unit: {:#?}", _person, _origin, _unit)//
}
*/
/*
클래식 C 구조체 가 가장 일반적으로 사용됩니다. 이러한 필드 내에 정의된 각 필드에는 이름 및 형식이 있습니다. 정의된 후에는 example_struct.field 구문을 사용하여 액세스할 수 있습니다.
튜플 구조체 는 클래식 구조체와 유사하지만 해당 필드에는 이름이 없습니다. 개별 변수에 액세스하는 경우 일반적인 튜플을 사용할 때와 동일한 구문, 즉 0부터 시작하는 foo.0, foo.1 등과 같은 구문이 사용됩니다.
단위 구조체 는 표식으로 가장 일반적으로 사용됩니다. Rust의 특성 기능에 대해 알아볼 때 단위 구조체가 유용할 수 있는 이유를 자세히 알아보겠습니다.
*/

/*
열거형
열거형은 여러 변형 중 하나가 될 수 있는 형식입니다.

함수형 프로그래밍 배경에서 가져온 경우 Rust 호출 열거형은 일반적으로 대수 데이터 형식으로 알려져 있습니다. 중요한 세부 정보는 각 열거형 변형에 데이터를 포함할 수 있다는 것입니다.

enum 키워드를 사용하면 몇 가지 다른 변형 중 하나에 해당할 수 있는 형식을 만들 수 있습니다. 구조체와 마찬가지로 열거형 변형은 이름이 있는 필드 또는 이름이 없는 필드를 포함하거나 필드를 포함하지 않을 수 있습니다.

다음 예제에서는 웹 이벤트를 분류하는 열거형을 정의합니다. 각 변형은 독립적이며 다른 크기와 유형의 값을 저장합니다.
*/
/*
struct Click { 
    x: i64, 
    y: i64 
}

struct KeyPress(char);

enum WebEvent {
    /*
    이 열거형에는 다음과 같은 다양한 형식의 4가지 변형이 있습니다.

    PageLoad 및 PageUnload에는 연결된 데이터가 없습니다.
    Keypress에는 단일 문자가 포함됩니다.
    Paste에는 단일 문자열이 포함됩니다.
    Click의 내부에는 익명 구조체가 있습니다.
    */

    // An enum may either be unit-like,
    PageLoad,
    PageUnload,
    // An enum can include characters and strings
    KeyPress(char),
    Paste(String),
    // or include tuple structs
    Click { x: i64, y: i64 },
}
*/

/*
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car{color, transmission, convertible, mileage: 0};

    // Factory's Quality Control Department says that new cars must always have zero mileage!
    assert_eq!(car.mileage, 0);
    return car;
}

fn main() {
    let client_request_1 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Manual);
    assert_eq!(client_request_1.convertible, false);

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);

    let client_request_2 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_request_2.color, "Yellow");
    assert_eq!(client_request_2.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_2.convertible, false);
}
*/

