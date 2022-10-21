use candid_gen::generate_candid_method;

fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
