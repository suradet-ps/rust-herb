use leptos::prelude::*;

#[component]
pub fn SearchBar(
    #[prop(into)] categories: Signal<Vec<String>>,
    #[prop(into)] on_search: Callback<String>,
    #[prop(into)] on_filter: Callback<String>,
) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    let handle_search = move |_| {
        on_search.run(search_query.get());
    };

    let handle_enter = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            on_search.run(search_query.get());
        }
    };

    view! {
        <div class="search-section">
            <div class="container search-container">

                // Input Group
                <div class="search-input-group">
                    <div class="search-icon-wrapper">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="icon"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            />
                        </svg>
                    </div>

                    <input
                        type="text"
                        class="search-input"
                        placeholder="ค้นหาสมุนไพร..."
                        prop:value=search_query
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        on:keydown=handle_enter
                    />

                    <button class="search-button" on:click=handle_search>
                        "ค้นหา"
                    </button>
                </div>

                // Filter Select
                <div class="filter-group">
                    <div class="select-wrapper">
                        <select
                            class="filter-select"
                            on:change=move |ev| on_filter.run(event_target_value(&ev))
                        >
                            <option value="">"ทุกหมวดหมู่"</option>
                            <For
                                each=move || categories.get()
                                key=|cat| cat.clone()
                                // แก้ไขตรงนี้ครับ: cat.clone() ทั้ง value และ child
                                children=move |cat| {
                                    view! { <option value=cat.clone()>{cat.clone()}</option> }
                                }
                            />
                        </select>

                        <div class="select-arrow">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="icon"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
