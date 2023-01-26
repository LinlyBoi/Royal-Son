use yew::prelude::*;
#[function_component]
fn App() -> Html {
    let stdnts = vec![
        Stdnt {
            id: 1,
            name: "Hamada".to_string(),
            email: "Hamada@Gmail.com".to_string(),
            gpa: 3.4,
        },
        Stdnt {
            id: 2,
            name: "Gamal".to_string(),
            email: "Gamal@Gmail.com".to_string(),
            gpa: 4.0,
        },
        Stdnt {
            id: 3,
            name: "Joe".to_string(),
            email: "joe@proton.mail".to_string(),
            gpa: 3.8,
        },
    ];
    let stdnts_comp = stdnts
        .iter()
        .map(|stdnt| {
            html! {
                <>
                <p key={stdnt.id}>{format!("Name: {}", stdnt.name)} </p>
                <p key={stdnt.id}>{format!("Email: {}", stdnt.email)} </p>
                <p key={stdnt.id}>{format!("GPA: {}",stdnt.gpa)} </p>
                <br/>
                </>
            }
        })
        .collect::<Html>();

    html! {
        <div>
            <h2> { "Students:" } </h2>
            <p> { stdnts_comp } </p>
        </div>
    }
}
struct Stdnt {
    id: usize,
    name: String,
    gpa: f32,
    email: String,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
