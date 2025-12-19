use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    // ใช้ปีปัจจุบันแบบ Hardcode หรือใช้ library time ก็ได้
    let current_year = 2025;

    view! {
        <footer class="app-footer">
            <div class="container footer-content">
                <p class="footer-ref">
                    "อ้างอิงตามประกาศ สปสช. วันที่ 10 เมษายน 2568"
                </p>
                <p class="footer-copyright">
                    "&copy; " {current_year}
                    " ลิขสิทธิ์ | พัฒนาโดย ภก. สุรเดช ประถมศักดิ์"
                </p>
            </div>
        </footer>
    }
}
