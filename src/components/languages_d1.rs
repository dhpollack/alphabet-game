use leptos::prelude::*;

#[server]
pub async fn show_languages() -> Result<String, ServerFnError> {
    use axum::Extension;
    use serde::Serialize;
    use std::sync::Arc;
    use worker::Env;

    #[derive(Serialize, sqlx_d1::FromRow)]
    struct Language {
        id: u32,
        name: String,
        name_other: Option<String>,
        code: String,
    }

    println!("starting show_languages...");
    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);
    let res = sqlx_d1::query_as!(Language, "SELECT id, name, name_other, code FROM Languages",)
        .fetch_all(&conn)
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;
    let langs: Vec<String> = res.into_iter().map(|rec| rec.name).collect();
    let val = format!("Found Languages: {}", langs.join(", "));
    Ok(val)
}

#[component]
pub fn ShowLanguagesFromD1() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let on_click = move |_| {
        leptos::task::spawn_local(async move {
            match show_languages().await {
                Ok(val) => value.set(val.clone()),
                Err(e) => println!("error: {e}"),
            }
        });
    };

    view! {
        <div>
            <button on:click=on_click>What languages are available?</button>
            <p>{value}</p>
        </div>
    }
}
