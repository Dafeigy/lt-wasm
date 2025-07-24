mod generator;

fn main() {
    let mut gene = generator::LTGenerator::new(
        vec![
            "abcd".to_string(), 
            "efgh".to_string(), 
            "ijkl".to_string(), 
            "mnop".to_string()]
        );

    // for i in &mut gene {
    //     println!("Next value: {}", i);
    // }
    let mut i:i32 = 0;
    while i < 20 {
        println!("Next value: {:?}", generator::get_next_value(&mut gene));
        i += 1;
    }

}

#[test]
fn test_generator(){
    let mut gene = generator::LTGenerator::new( 
        vec![
            "abcd".to_string(), 
            "efgh".to_string(), 
            "ijkl".to_string(), 
            "mnop".to_string()
            ]);

    // for i in &mut gene {
    //     println!("Next value: {}", i);
    // }
    let mut i:i32 = 0;
    while i < 20 {
        println!("Next value: {:?}", generator::get_next_value(&mut gene));
        i +=1;
    }
}
