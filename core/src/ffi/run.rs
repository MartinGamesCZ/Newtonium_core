#[no_mangle]
// Function for running the GTK application
pub extern "C" fn run() -> () {
  // Run the GTK application
  gtk::main();

  ()
}
