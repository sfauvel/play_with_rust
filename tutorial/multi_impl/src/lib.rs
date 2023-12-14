
#[allow(dead_code)]
fn func_box_report(report: &Box<dyn Report>) -> String {
    report.report()
}

#[allow(dead_code)]
fn func_dyn_report(report: &dyn Report) -> String {
    report.report()
}
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
        let article: NewsArticle = NewsArticle::new("Seb", "Something");
        assert_eq!("Something, by Seb", article.report());
        let box_article: Box<dyn Report> = Box::new(article);
        assert_eq!("Something, by Seb", func_box_report(&box_article));


        let article: AnonymousArticle = AnonymousArticle::new("Something by X");
        assert_eq!("Something by X", article.report());
        let box_article: Box<dyn Report> = Box::new(article);
        assert_eq!("Something by X", func_box_report(&box_article));
        
        let article: Box<dyn Report> = create(0);
        assert_eq!("Something, by Seb", article.report());
        assert_eq!("Something, by Seb", func_box_report(&article));

        let article: Box<dyn Report> = create(1);
        assert_eq!("Something, by Bob", article.report());
        assert_eq!("Something, by Bob", func_box_report(&article));
        
        let article: Box<dyn Report> = create(2);
        assert_eq!("Something by X", article.report());
        assert_eq!("Something by X", func_box_report(&article));
    }
    #[test]
    fn my_test_report_with_dyn_report() {
        let article: NewsArticle = NewsArticle::new("Seb", "Something");
        assert_eq!("Something, by Seb", func_dyn_report(&article));

        let article: AnonymousArticle = AnonymousArticle::new("Something by X");
        assert_eq!("Something by X", func_dyn_report(&article));

        let article: Box<dyn Report> = create(0);
        // With `as_ref` we transform `Box<R>` to `&dyn R`
        article.as_ref();
        assert_eq!("Something, by Seb", func_dyn_report(article.as_ref()));
        
        let article: Box<dyn Report> = create(1);
        assert_eq!("Something, by Bob", func_dyn_report(article.as_ref()));
        
        let article: Box<dyn Report> = create(2);
        assert_eq!("Something by X", func_dyn_report(article.as_ref()));
    }
    #[test]
    fn my_test_report_with_generic() {
        let article: NewsArticle = NewsArticle::new("Seb", "Something");
        assert_eq!("Something, by Seb", func_generic_report(&article));

        let article: AnonymousArticle = AnonymousArticle::new("Something by X");
        assert_eq!("Something by X", func_generic_report(&article));

        // As we return a `Box`, we need to store it in a variable before calling `as_ref`. 
        // Otherwise, the value is freed immediately and we could not get a reference.
        let article: Box<dyn Report> = create(0);
        assert_eq!("Something, by Seb", func_generic_report(article.as_ref()));
        
        let article: Box<dyn Report> = create(1);
        assert_eq!("Something, by Bob", func_generic_report(article.as_ref()));
        
        let article: Box<dyn Report> = create(2);
        assert_eq!("Something by X", func_generic_report(article.as_ref()));
    }
}