pub trait Reportable {
    // Returns a summary of the reportable item.
    fn summary(&self) -> String;
    // Returns a detailed description of the reportable item.
    fn detail(&self) -> String;

    fn print_summary(&self) {
        println!("{}", self.summary());
    }

    fn print_detail(&self) {
        println!("{}", self.detail());
    }
}
