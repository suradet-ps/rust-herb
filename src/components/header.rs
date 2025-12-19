use crate::components::logo_icon::LogoIcon;
use leptos::prelude::*; // สมมติว่าแยกไฟล์ Logo ไว้ หรือจะใส่ Inline ก็ได้

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="app-header">
            <div class="container header-content">
                <div class="header-branding">
                    // เรียกใช้ Logo Component หรือใส่ SVG ตรงนี้
                    <LogoIcon class="header-logo" />

                    <div class="header-text">
                        <h1 class="header-title">
                            "บัญชียาสมุนไพร รพ.สระโบสถ์"
                        </h1>
                        <p class="header-subtitle">
                            "รายการสมุนไพรที่สปสช.สนับสนุนจ่ายชดเชย"
                        </p>
                    </div>
                </div>
            </div>
        </header>
    }
}
