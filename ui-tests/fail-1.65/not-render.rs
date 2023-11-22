struct NotRender;

markup::define! {
    Layout(name: NotRender) {
        @markup::doctype()
        html["attr-user"=name] {
            body {
                strong { "Hello " @name "!" }
            }
        }
    }
}

fn main() {

}
