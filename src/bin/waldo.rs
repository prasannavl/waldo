extern crate waldo;

fn main() {
    waldo::windows::api::console_set_vterm();
    waldo::run();
}

#[cfg(test)]
mod test {

}
