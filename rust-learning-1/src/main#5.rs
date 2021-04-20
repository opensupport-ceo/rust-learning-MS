/*
제어 흐름을 사용하여 코드 반복
*/

fn main() {
    //예제에서 if의1 == 2 조건은 값이 false 입니다.
    /*
    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }
    */
    

    //대부분의 언어와 달리 if 블록은 식으로 동작할 수도 있습니다. 
    //코드가 컴파일되려면 모든 분기는 동일한 형식을 반환해야 합니다.
    /*
    let formal = true;
    let greeting = if formal {
        "Good evening."
    } else {
        "Hello, friend!"
    };
    println!("{}",greeting); // prints "Good evening."
    */


    //else if 식에서 if와 else를 결합하여 여러 조건을 사용할 수 있습니다
    /*
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    */

}