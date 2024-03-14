fn main() {
    {}
}
// 속성형 매크로
// #[route(GET, "/")]
// fn index() {
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// 함수형 매크로
// let sql = sql!(SELECT * FROM posts WHERE id=1);
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {