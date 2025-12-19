use crate::models::Herb;
use leptos::prelude::*;

#[component]
pub fn HerbCard(herb: Herb) -> impl IntoView {
    // Helper function สำหรับตัดคำ
    let truncate_text = |text: &str, max_len: usize| -> String {
        let chars: Vec<char> = text.chars().collect();
        if chars.len() <= max_len {
            text.to_string()
        } else {
            let truncated: String = chars.into_iter().take(max_len).collect();
            format!("{}...", truncated)
        }
    };

    let image_url = herb
        .image_url
        .clone()
        .unwrap_or_else(|| "/placeholder-herb.png".to_string());

    view! {
        <article class="herb-card">
            // Image Section
            <div class="card-image-wrapper">
                <img src=image_url alt=herb.name.clone() class="card-image" loading="lazy" />
                <div class="card-overlay"></div>
            </div>

            // Content Section
            <div class="card-content">
                <header>
                    <h3 class="card-title">{herb.name}</h3>
                </header>

                <div class="card-body">
                    <p class="card-description">
                        <strong class="label">"สรรพคุณ: "</strong>
                        {truncate_text(&herb.description, 80)}
                    </p>
                    <p class="card-usage">
                        <strong class="label">"วิธีใช้: "</strong>
                        {herb.usage.clone().unwrap_or_else(|| "-".to_string())}
                    </p>
                </div>

                // Footer Metadata
                <footer class="card-footer">
                    <div class="meta-grid">
                        <div class="meta-item">
                            <span class="meta-label">"อัตราจ่าย"</span>
                            <span class="meta-value">
                                {herb.nhso_price.clone().unwrap_or_else(|| "N/A".to_string())}
                            </span>
                        </div>
                        <div class="meta-item">
                            <span class="meta-label">"รอบเบิก"</span>
                            <span class="meta-value">
                                {herb.per_course.clone().unwrap_or_else(|| "N/A".to_string())}
                            </span>
                        </div>
                        <div class="meta-item">
                            <span class="meta-label">"ICD10"</span>
                            <span class="meta-value">
                                {herb.icd_10.clone().unwrap_or_else(|| "N/A".to_string())}
                            </span>
                        </div>
                    </div>

                    <div class="card-tags">
                        <span class="tag-category">{herb.category}</span>
                    </div>
                </footer>
            </div>
        </article>
    }
}
