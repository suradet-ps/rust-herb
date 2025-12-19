use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

// Import components from crate root
use crate::api::get_all_herbs;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::herb_card::HerbCard;
use crate::components::search_bar::SearchBar;
use crate::components::skeleton::HerbCardSkeleton;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="th">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="theme-color" content="#2c7a7b" />
                <meta
                    name="description"
                    content="ค้นหาข้อมูลยาสมุนไพรในบัญชีโรงพยาบาลสระโบสถ์..."
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // แก้ไขชื่อไฟล์ CSS ให้ตรงกับชื่อโปรเจกต์ (rust-herb)
        <Stylesheet id="leptos" href="/pkg/rust-herb.css" />
        <Title text="Thai Herbal NHSO Support App" />

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomeView />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomeView() -> impl IntoView {
    // State Signals
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal(String::new());

    // Data Fetching Resource
    let herbs_resource = Resource::new(|| (), |_| get_all_herbs());

    // Filtering Logic (Memoized)
    let filtered_herbs = Memo::new(move |_| {
        let query = search_query.get().trim().to_lowercase();
        let category = selected_category.get();

        herbs_resource.get().and_then(|res| {
            res.ok().map(|herbs| {
                herbs
                    .into_iter()
                    .filter(|herb| {
                        let name_match = herb.name.to_lowercase().contains(&query);
                        let desc_match = herb.description.to_lowercase().contains(&query);

                        // Check scientific_name safely
                        let sci_match = herb
                            .scientific_name
                            .as_ref()
                            .map(|s| s.to_lowercase().contains(&query))
                            .unwrap_or(false);

                        let matches_search =
                            query.is_empty() || name_match || desc_match || sci_match;
                        let matches_category = category.is_empty() || herb.category == category;

                        matches_search && matches_category
                    })
                    .collect::<Vec<_>>()
            })
        })
    });

    // Extract Categories Logic (Memoized)
    let categories = Memo::new(move |_| {
        herbs_resource
            .get()
            .and_then(|res| {
                res.ok().map(|herbs| {
                    let mut cats: Vec<String> = herbs.iter().map(|h| h.category.clone()).collect();
                    cats.sort();
                    cats.dedup(); // Remove duplicates
                    cats
                })
            })
            .unwrap_or_default()
    });

    view! {
        <div class="app-layout">
            <Header />

            // --- FIX: Wrapping SearchBar in Suspense to prevent Hydration Panic ---
            <Suspense fallback=|| {
                view! {
                    // Fallback UI: Dummy SearchBar while loading
                    <div class="search-section">
                        <div class="container search-container">
                            <div
                                class="search-input-group"
                                style="opacity: 0.6; pointer-events: none;"
                            >
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
                                    placeholder="กำลังโหลดข้อมูล..."
                                    disabled
                                />
                                <button class="search-button" disabled>
                                    "ค้นหา"
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }>
                {move || {
                    herbs_resource
                        .get()
                        .map(|_| {
                            // Access resource to trigger suspense
                            view! {
                                <SearchBar
                                    categories=categories
                                    on_search=move |q| set_search_query.set(q)
                                    on_filter=move |c| set_selected_category.set(c)
                                />
                            }
                        })
                }}
            </Suspense>

            <main class="main-content container">
                <Suspense fallback=move || {
                    view! {
                        <div class="loading-state">
                            <div class="loading-indicator">
                                <div class="spinner"></div>
                                <span class="loading-text">
                                    "กำลังดาวน์โหลดข้อมูลสมุนไพร..."
                                </span>
                            </div>
                            <div class="herb-grid">
                                <For
                                    each=|| 0..8
                                    key=|i| *i
                                    children=move |_| view! { <HerbCardSkeleton /> }
                                />
                            </div>
                        </div>
                    }
                }>
                    {move || {
                        match herbs_resource.get() {
                            Some(Ok(_)) => {
                                let list = filtered_herbs.get().unwrap_or_default();

                                view! {
                                    <div class="results-container">
                                        {if !list.is_empty() {
                                            view! {
                                                <div class="results-count">
                                                    "พบ " {list.len()} " รายการ"
                                                </div>
                                                <div class="herb-grid">
                                                    <For
                                                        each=move || list.clone()
                                                        key=|herb| herb.id.to_string()
                                                        children=move |herb| view! { <HerbCard herb=herb /> }
                                                    />
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <div class="empty-state">
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        class="empty-icon"
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
                                                    <p class="empty-title">
                                                        "ไม่พบสมุนไพรที่ตรงกับการค้นหา"
                                                    </p>
                                                    <p class="empty-subtitle">
                                                        "ลองเปลี่ยนคำค้นหาหรือเลือกหมวดหมู่อื่น"
                                                    </p>
                                                </div>
                                            }
                                                .into_any()
                                        }}
                                    </div>
                                }
                                    .into_any()
                            }
                            Some(Err(e)) => {
                                view! {
                                    <div class="error-state">
                                        <p class="error-message">
                                            "เกิดข้อผิดพลาด: "
                                            {e.to_string()}
                                        </p>
                                        <button
                                            class="retry-button"
                                            on:click=move |_| herbs_resource.refetch()
                                        >
                                            "ลองใหม่อีกครั้ง"
                                        </button>
                                    </div>
                                }
                                    .into_any()
                            }
                            None => ().into_any(),
                        }
                    }}
                </Suspense>
            </main>

            <Footer />
        </div>
    }
}
