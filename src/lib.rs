fn char_transform_ar(c:char)-> char{
    match c{
        '1'=>'١',
        '2'=>'٢',
        '3'=>'٣',
        '4'=>'٤',
        '5'=>'٥',
        '6'=>'٦',
        '7'=>'٧',
        '8'=>'٨',
        '9'=>'٩',
        '0'=>'٠',
        other=>c
    }
}
pub fn transformer_ar(input: String) -> String {
    let mut output = String::from("");
    for c in input.chars() {
        output.push(char_transform_ar(c));
        //output.insert(0,c);
    }
    output
}
#[test]
fn transform_test(){
    let expected = "١٢٣٤٥٦٧٨٩٠".to_string();
    let input = "1234567890".to_string();
    assert_eq!(expected,transformer_ar(input) );
    let expected = "أهلا hello hi  ١٢٣٤٥٦٧٨٩٠ ".to_string();
    let input = "أهلا hello hi  1234567890 ".to_string();
    assert_eq!(expected,transformer_ar(input) );
}
