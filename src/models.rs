use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Herb {
    // Google Sheets มักส่ง ID มาเป็นตัวเลขหรือ String ก็ได้ ใช้ Value รับจะปลอดภัยสุด
    #[serde(alias = "ID")]
    pub id: serde_json::Value,

    #[serde(alias = "Name")]
    pub name: String,

    #[serde(alias = "ScientificName")]
    pub scientific_name: Option<String>,

    #[serde(alias = "Description")]
    pub description: String,

    #[serde(alias = "Usage")]
    pub usage: Option<String>,

    #[serde(alias = "ImageUrl")]
    pub image_url: Option<String>,

    #[serde(alias = "Category")]
    pub category: String,

    #[serde(alias = "NHSO_Price")]
    pub nhso_price: Option<String>,

    #[serde(alias = "Per_Course")]
    pub per_course: Option<String>,

    #[serde(alias = "ICD10")]
    pub icd_10: Option<String>,

    #[serde(alias = "Benefits")]
    pub benefits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
    pub message: Option<String>,
}
