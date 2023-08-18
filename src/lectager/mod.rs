
pub mod lectager {
    use axum::{
        response::Html,
    };
    use minijinja::{
        context, Environment,
    };
    
    fn get_students() -> Vec<&'static str> {
        let mut students:  Vec<&'static str> = Vec::with_capacity(5);
        students.push("Alice");
        students.push("Beatrix");
        students.push("Charlotte");
        students.push("Diana");
        
        students
    }

    fn get_lectures_of_student(){
        
    }
    pub fn app(env: Environment<'_>) -> Html<String>{
        let students: Vec<&'static str> = get_students();
        let tmpl = env.get_template("lectager/index.html").unwrap();
        
        Html(tmpl.render(context! {
            students => students,
        }).unwrap())
    
    }
}