use leptos::*;
// use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
  // view! {
  //   <Router>
  //     <nav>
  //       /* ... */
  //     </nav>
  //     <main>
  //       /* ... */
  //     </main>
  //   </Router>
  // }




  let (count, set_count) = leptos::create_signal(0);
  let double_count = move || count() * 2;
  let on_click = move |_| set_count.update(|x| *x += 1);

  view! {
      <button
          on:click=on_click
          // class:red=move || count() % 2 == 1
          class=move || if count() % 2 == 1 { "red" } else { "" }

      >
          "Click me"
      </button>
      <br/>
      <progress
          max=50
          value=count
      >
      </progress>
      <br/>
      <progress
          max="50"
          value=double_count
      >
      </progress>
      <p>"Count: " {count}</p>
      <p>"Double Count: " {double_count}</p>
  }
}