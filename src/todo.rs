use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    id: i16,
    title: String,
    completed: bool,
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut todos = Vec::new();
    let mut rows = sqlx::query_as::<_, Todo>("SELECT * FROM todos").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        todos.push(row);
    }

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server]
pub async fn delete_todo(id: i16) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .map(|_| ())?)
}

#[component]
pub fn TodoApp() -> impl IntoView {
    provide_meta_context();
    view! {

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/when3meet.css"/>
        <Router>
            <header>
                <h1>"My Tasks"</h1>
                 <h2 class="p-6 text-4x">"Welcome to Leptos with Tailwind"</h2>
            </header>
            <main>
                <Routes>
                    <Route path="" view=Todos/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Todos() -> impl IntoView {
    let add_todo = create_server_multi_action::<AddTodo>();
    let delete_todo = create_server_action::<DeleteTodo>();
    let submissions = add_todo.submissions();

    let todos = create_resource(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    view! {
        <div>
            <MultiActionForm
                on:submit=move |ev| {
                    let data = AddTodo::from_event(&ev).expect("to parse form data");
                    if data.title == "nope!" {
                        ev.prevent_default();
                    }
                }
                action=add_todo
            >
                <label>
                    "Add a Todo"
                    <input type="text" name="title"/>
                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
                {move || {
                    let existing_todos = {
                        move || {
                            todos.get()
                                .map(move |todos| match todos {
                                    Err(e) => {
                                        view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                    }
                                    Ok(todos) => {
                                        if todos.is_empty() {
                                            view! { <p>"No tasks were found."</p> }.into_view()
                                        } else {
                                            todos
                                                .into_iter()
                                                .map(move |todo| {
                                                    view! {

                                                        <li>
                                                            {todo.title}
                                                            <ActionForm action=delete_todo>
                                                                <input type="hidden" name="id" value={todo.id}/>
                                                                <input type="submit" value="X"/>
                                                            </ActionForm>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };

                    let pending_todos = move || {
                        submissions
                        .get()
                        .into_iter()
                        .filter(|submission| submission.pending().get())
                        .map(|submission| {
                            view! {

                                <li class="pending">{move || submission.input.get().map(|data| data.title) }</li>
                            }
                        })
                        .collect_view()
                    };

                    view! {

                        <ul>
                            {existing_todos}
                            {pending_todos}
                        </ul>
                    }
                }
            }
            </Transition>
        </div>
    }
}
