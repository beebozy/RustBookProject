
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results =Vec::new();

    
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_sensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();

    for line in contents.lines(){
        if line.contains(query) && line.chars().any(|c| c.is_lowercase()){
            results.push(line);
        }
    }

    results
}