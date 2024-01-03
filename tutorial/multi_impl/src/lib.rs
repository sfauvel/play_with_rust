// https://bennett.dev/dont-use-boxed-trait-objects-for-struct-internals/
// https://www.possiblerust.com/pattern/3-things-to-try-when-you-can-t-make-a-trait-object
// https://oswalt.dev/2021/06/polymorphism-in-rust/

// A function that use a reference to a Box.
#[allow(dead_code)]
fn func_box_report(report: &Box<dyn Report>) -> String {
    report.report()
}

// Function that use a dynamic trait.
#[allow(dead_code)]
fn func_dyn_report(report: &dyn Report) -> String {
    report.report()
}

// Function that use a generic type implementing a trait.
#[allow(dead_code)]
fn func_generic_report<R: Report + ?Sized>(report: &R) -> String {
    report.report()
}

#[allow(dead_code)]
pub trait Report {
    fn report(&self) -> String;

}

/// We create several kinds of objects so we must return a `Box`.
#[allow(dead_code)]
fn create(x: u32) -> Box<dyn Report> {
    if x == 0 {
        Box::new(NewsArticle::new("Seb", "Something"))
    } else if x == 1 {
        Box::new(NewsArticle::new("Bob", "Something"))
    } else {
        Box::new(AnonymousArticle::new("Something by X"))
    }
}



pub struct NewsArticle {
    pub author: String,
    pub content: String,
}

impl  NewsArticle {
    pub fn new(author: &str, content: &str) -> Self {
        Self {
            author:author.to_string(),
            content:content.to_string()
        }
    }
}
impl Report for NewsArticle {
    
    fn report(&self) -> String {
        format!("{}, by {}", self.content, self.author)
    }
}
pub struct AnonymousArticle {
    pub content: String,
}

impl  AnonymousArticle {
    pub fn new(content: &str) -> Self {
        Self {
            content:content.to_string()
        }
    }
}

impl Report for AnonymousArticle {    
    fn report(&self) -> String {
        format!("{}", self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test_report() {
        let article: Box<dyn Report> = create(0);
        assert_eq!("Something, by Seb", article.report());

        assert_eq!("Something, by Seb", create(0).report());
        assert_eq!("Something, by Bob", create(1).report());
        assert_eq!("Something by X", create(2).report());
    }

    #[test]
    fn my_test_report_with_box_dyn_function() {
        // Need to put it in a local variable.
        let box_article: Box<dyn Report> = Box::new(NewsArticle::new("Seb", "Something"));
        assert_eq!("Something, by Seb", func_box_report(&box_article));
 
        let box_article: Box<dyn Report> = Box::new(AnonymousArticle::new("Something by X"));
        assert_eq!("Something by X", func_box_report(&box_article));
        
        assert_eq!("Something, by Seb", func_box_report(&create(0)));
        assert_eq!("Something, by Bob", func_box_report(&create(1)));
        assert_eq!("Something by X", func_box_report(&create(2)));
    }
    #[test]
    fn my_test_report_with_dyn_report() {
        assert_eq!("Something, by Seb", func_dyn_report(&NewsArticle::new("Seb", "Something")));
        assert_eq!("Something by X", func_dyn_report(&AnonymousArticle::new("Something by X")));

        // With `as_ref` we transform `Box<R>` to `&dyn R`
        assert_eq!("Something, by Seb", func_dyn_report(create(0).as_ref()));
        assert_eq!("Something, by Bob", func_dyn_report(create(1).as_ref()));
        assert_eq!("Something by X", func_dyn_report(create(2).as_ref()));
    }
    #[test]
    fn my_test_report_with_generic() {
        assert_eq!("Something, by Seb", func_generic_report(&NewsArticle::new("Seb", "Something")));
        assert_eq!("Something by X", func_generic_report(&AnonymousArticle::new("Something by X")));

        // As we return a `Box`, we need to store it in a variable before calling `as_ref`. 
        // Otherwise, the value is freed immediately and we could not get a reference.
        assert_eq!("Something, by Seb", func_generic_report(create(0).as_ref()));
        assert_eq!("Something, by Bob", func_generic_report(create(1).as_ref()));
        assert_eq!("Something by X", func_generic_report(create(2).as_ref()));
    }
}