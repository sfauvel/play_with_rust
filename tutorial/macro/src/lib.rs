
// Because it s use only in tests
#[allow(dead_code)]
#[macro_export]
macro_rules! adoc_table {
    ( $( $line:expr ),* ) => {
        {
            let result = "|====\n"; 
            $(
                let text: Vec<String> = $line.iter().map(|x| x.to_string()).collect();
                let result = result.to_string() + "|" + &text.join("|") + "\n"; 
            )*
            let result = result.to_string() + "|===="; 
            
            println!("{}", result);
            result

        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_table_with_string() {
        let result = adoc_table!(vec!("toto", "titi"));
        let expected = "|====\n|toto|titi\n|====";
        assert_eq!(expected, result);
    }    
    
    #[test]
    fn format_table_with_string_and_integer() {        
        let result = adoc_table!(vec!("toto", "titi"), vec!(42,45));
        let expected = "|====\n|toto|titi\n|42|45\n|====";
        assert_eq!(expected, result);

        

    }


}