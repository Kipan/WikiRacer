use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};
use std::{collections::HashSet, ops::Sub};

static SITE: &str = "https://en.wikipedia.org/wiki/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);

        App::new().wrap(Cors::permissive()).service(submit)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await


}



#[derive(Serialize, Deserialize)]
struct Submission {
    start: String,
    target: String,
}

#[derive(Serialize)]
struct Results {
    time: u64,
    path: Vec<String>,
}

impl Submission {
    fn new(start: String, target: String) -> Self {
        Submission { start, target }
    }
}

#[post("/submit")]
async fn submit(user_data: web::Query<Submission>) -> impl Responder {
    //HttpResponse::Ok().body("the start is ".to_string() + &user_data.into_inner().start)
    let start = user_data.start.clone();
    let target = user_data.target.clone();
    let mut path = Vec::new();

    let now = SystemTime::now();
    path = find_path(&start, &target);
    let mut time = 0;
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            time = elapsed.as_secs();
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    HttpResponse::Created().json(Results { time, path })
}

/*
Converts an HTML page to a String
Pass in Wiki title
Returns String-ified version of HTML
*/
fn html_to_str(url_str: &str) -> String {
    // First write everything into a `Vec<u8>`
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut body = String::new();
    handle.url(&format!("{SITE}{url_str}")).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    // Convert it to `String`
    body.push_str(&String::from_utf8(data).expect("body is not valid UTF8!"));

    body
}

/*
Returns all valid Wiki titles taking in an HTML string
Pass in String-ified version of Wiki HTML
Return a HashSet containing Strings of titles without # or :
*/
fn find_wiki_links(input: &str) -> HashSet<String> {
    struct HTMLIterator<'a> {
        input: &'a str,
        curr: usize,
    }

    impl<'a> HTMLIterator<'a> {
        fn new(input: &'a str) -> Self {
            Self { input, curr: 0 }
        }
    }

    impl<'a> Iterator for HTMLIterator<'a> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(start) = self.input[self.curr..].find("/wiki/") {
                let start = self.curr + start;
                let end = self.input[start..].find(r#"""#).map(|e| start + e);
                if let Some(end) = end {
                    let tryme = &self.input[start + 6..end];
                    self.curr = end + 1;
                    if !tryme.contains(':') && !tryme.contains('#') {
                        return Some(tryme.to_string());
                    }
                } else {
                    break;
                }
            }
            None
        }
    }

    let iter = HTMLIterator::new(input);
    let link_set: HashSet<String> = iter.collect();

    link_set
}

/*
Finds a path between two Wiki pages
Pass in the start and target Wiki title
Returns a Vector containing Wiki titles in order of path from start to target
*/
fn find_path(start: &str, target: &str) -> Vec<String> {
    // Empty priority queue of ladders
    let mut pq: VecDeque<Vec<String>> = VecDeque::new();
    pq.push_back(vec![start.to_string()]);

    // Empty hashset of visited pages
    let mut visited_set: HashSet<String> = HashSet::new();

    // Define target info
    let body_target = html_to_str(target);
    let link_set_target = find_wiki_links(&body_target);

    // Keep popping off ladders until it's empty
    while let Some(mut path) = pq.pop_front() {
        //println!("{:?}", pq);
        let curr_page = path.last().unwrap().clone();
        let curr_page_set = find_wiki_links(&html_to_str(&curr_page));
        visited_set.insert(curr_page.clone());

        // If the current page contains target, add it to the ladder and return!
        if curr_page_set.contains(&target.to_string()) {
            path.push(target.to_string());
            return path;
        }

        // for each page within the current page, check if its been visited.
        // If not visited, push the page onto the current path
        // Find the priority, then push
        for page in curr_page_set {
            if !visited_set.contains(&page) {
                //visited_set.insert(curr_page.clone());
                let mut new_path = path.clone();
                new_path.push(page.clone());
                //let priority =
                //    num_pages_in_common(&find_wiki_links(&html_to_str(&page)), &link_set_target);
                pq.push_back(new_path);
                //println!("{}",page);
            }
        }
        //println!("{:?}", pq);
    }

    Vec::new()
}

/*
Find number of pages in common between two pages to determine priority
Pass in two titles
Return number of common pages
*/
fn num_pages_in_common(set1: &HashSet<String>, set2: &HashSet<String>) -> usize {
    return set1.intersection(set2).count();
}
