// Genericsとは型と関数の機能をより汎用的に使えるようにするための機能
// コードの重複を避けるのに役立つが構文が複雑になる
// Generic型を使いこなすにはジェネリクス型がきちんと機能するかに細心の注意が必要

// Generic型には鍵括弧とアッパーキャメルケースが使われる <Aaa, Bbb, ...>
// Generic型のパラメータはたいていの場合、<T>で示される
// Rustの場合、ジェネリクスには「1つ以上のジェネリックな型パラメータ<T>を受ける物」という意味がある
// ジェネリックな型パラメータが指定された場合、それは必ずジェネリック型になる
//    そうでなければ必ず、非ジェネリック型 = 具象型(concrete)になる

// 例 あらゆる型の引数Tを取るジェネリック関数fooを定義する
#[allow(dead_code)]
fn foo<T>(_arg: T) {}
// Tはジェネリックな型パラメータ<T>として指定されているので(_arg:T)のように指定されると
// ジェネリック型として扱われる。これ以前にT構造体が定義されていても同様となる

// Aは具象型
struct A;

// <A>という指定が無く、A自身も具象型のため
// Singleは具象型
struct Single(A);

// <T>が指定されていて始めのTよりも先に来ているので
// SingleGenはジェネリック型
// Tはどんな型にもなりえる、上で指定したAやSingleも受け取れる
struct SingleGen<T>(T);

fn main() {
    // Singleは具象型で型Aを受け取る
    let _s = Single(A);

    // _charにSingleGen<char>型の値を束縛する
    // 値はSingleGen('a')である。
    // SingleGen<T>に具体的な型パラメータ(T=char)が指定されている
    let _char: SingleGen<char> = SingleGen('a');

    // SingleGen型の変数には明示的にパラメータを与えなくても良い
    let _t = SingleGen(A);
    let _t_i32 = SingleGen(6);
    let _t_char = SingleGen('a');
    // こんなのもいける
    let _t_nested = SingleGen(SingleGen(SingleGen(A)));
}
