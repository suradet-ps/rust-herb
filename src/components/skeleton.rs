use leptos::prelude::*;

#[component]
pub fn HerbCardSkeleton() -> impl IntoView {
    view! {
        <div class="herb-card skeleton-card animate-pulse">
            <div class="card-image-wrapper skeleton-image-placeholder"></div>

            <div class="card-content">
                <div class="skeleton-title-line"></div>

                <div class="skeleton-body">
                    <div class="skeleton-line full"></div>
                    <div class="skeleton-line medium"></div>
                    <div class="skeleton-line short"></div>
                </div>

                <div class="card-footer">
                    <div class="meta-grid">
                        <div class="skeleton-meta-item"></div>
                        <div class="skeleton-meta-item"></div>
                        <div class="skeleton-meta-item"></div>
                    </div>
                    <div class="skeleton-tag"></div>
                </div>
            </div>
        </div>
    }
}
