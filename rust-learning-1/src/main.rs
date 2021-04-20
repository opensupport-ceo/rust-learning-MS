/*
while 루프에서 조건이 충족될 때까지 반복
while 식은 조건자가 false가 될 때까지 반복됩니다.

while 루프는 먼저 부울 루프 조건식을 평가합니다. 루프 조건식이 true로 평가되면 루프 본문 블록이 실행됩니다. 그런 다음, 루프 조건식으로 제어 권한이 돌아갑니다. 루프 조건식이 false로 평가되면 while 식이 완료됩니다.
*/
/*
fn main(){
    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }
}
*/

/*
for 루프로 반복
for 식은 반복기에서 값을 추출합니다. 반복기가 빈 상태가 될 때까지 루프를 반복합니다.

Rust에서 반복기는 값을 반복할 수 있는 모든 형식입니다. 직접 반복할 수 있는 값도 있고, .iter()와 같은 메서드를 호출하여 반복기를 생성할 수 있는 값도 있습니다.
*/

fn main(){
    /*
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    */

    /*
    반복기를 만드는 또 다른 쉬운 방법은 a..b 범위 표기법을 사용하는 것입니다. 이 표기법은 a(포함)에서 b(제외)까지의 값을 한 단계로 생성합니다.
    */
    /*
    for item in 0..5 {
        println!("{}", item * 2);
    }
    */
    
}