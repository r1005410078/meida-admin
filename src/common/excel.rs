use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReadHouseExcel {
    #[serde(rename = "标签")]
    tag: Option<String>,

    #[serde(rename = "类型")]
    property_type: Option<String>,

    #[serde(rename = "用途")]
    usage: Option<String>,

    #[serde(rename = "小区")]
    community: Option<String>,

    #[serde(rename = "座栋")]
    building: Option<String>,

    #[serde(rename = "单元")]
    unit: Option<String>,

    #[serde(rename = "房号")]
    room_number: Option<String>,

    #[serde(rename = "楼层")]
    floor: Option<String>,

    #[serde(rename = "房型")]
    layout: Option<String>,

    #[serde(rename = "建筑面积")]
    area: Option<f64>,

    #[serde(rename = "装修")]
    decoration: Option<String>,

    #[serde(rename = "配套")]
    facilities: Option<String>,

    #[serde(rename = "来源")]
    source: Option<String>,

    #[serde(rename = "售价")]
    sale_price: Option<String>,

    #[serde(rename = "单价")]
    unit_price: Option<f64>,

    #[serde(rename = "租价")]
    rental_price: Option<String>,

    #[serde(rename = "备注")]
    remarks: Option<String>,

    #[serde(rename = "录入日期")]
    entry_date: Option<String>,

    #[serde(rename = "修改日期")]
    modification_date: Option<String>,

    #[serde(rename = "全员最后维护")]
    last_maintained_by: Option<String>,

    #[serde(rename = "录入人")]
    entered_by: Option<String>,
}

#[derive(Debug)]
pub struct HouseLayout {
    // 卧室数量
    pub bedrooms: i32,
    // 客厅数量
    pub living_rooms: i32,
    // 卫生间数量
    pub bathrooms: i32,
    // 阳台
    pub balcony: i32,
    // 厨房
    pub kitchen: i32,
}

impl From<String> for HouseLayout {
    fn from(s: String) -> Self {
        let mut layout = HouseLayout {
            bedrooms: 0,
            living_rooms: 0,
            bathrooms: 0,
            balcony: 0,
            kitchen: 0,
        };

        let mut num_str: String = String::new();
        for (_, c) in s.chars().enumerate() {
            if let Ok(num) = num_str.parse::<i32>() {
                if c == '室' {
                    layout.bedrooms = num;
                } else if c == '厅' {
                    layout.living_rooms = num;
                } else if c == '卫' {
                    layout.bathrooms = num;
                } else if c == '阳' {
                    layout.balcony = num;
                } else if c == '厨' {
                    layout.kitchen = num;
                }
            }

            if c.is_numeric() {
                num_str.push(c);
            } else {
                num_str.clear();
            }
        }

        layout
    }
}

impl ReadHouseExcel {
    pub fn get_num(value: Option<String>) -> Option<f64> {
        value.map(|s| {
            let mut num_str = String::new();
            for c in s.chars() {
                if c.is_numeric() {
                    num_str.push(c);
                } else {
                    break;
                }
            }

            num_str.parse::<f64>().unwrap()
        })
    }
    pub fn get_layout(&self) -> Option<HouseLayout> {
        self.layout.clone().map(|l| l.into())
    }

    pub fn sale_price(&self) -> Option<f64> {
        Self::get_num(self.sale_price.clone())
    }

    pub fn rental_price(&self) -> Option<f64> {
        Self::get_num(self.rental_price.clone())
    }

    pub fn read_xlsx(path: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        println!("{}", path);
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        println!("成功 {}", path);
        let sheet_names = workbook.sheet_names();

        let mut data = vec![];

        for sheet_name in sheet_names {
            // Read whole worksheet data and provide some statistics
            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                let iter_records = RangeDeserializerBuilder::with_headers(&[
                    "标签",
                    "类型",
                    "用途",
                    "小区",
                    "座栋",
                    "单元",
                    "房号",
                    "楼层",
                    "房型",
                    "建筑面积",
                    "装修",
                    "配套",
                    "来源",
                    "售价",
                    "单价",
                    "租价",
                    "备注",
                    "录入日期",
                    "修改日期",
                    "全员最后维护",
                    "录入人",
                ])
                .from_range(&range)?;
                for record in iter_records {
                    data.push(record?);
                }
            }
        }

        Ok(data)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_read_xlsx() {
        let data = ReadHouseExcel::read_xlsx(
            "/Users/rongts/www/meida_house_admin/upload/properties/02d47ba1-fc0a-491d-b38b-c65cdd94a9ff+erp1.xltx",
        )
        .unwrap();
        println!("{:?}", data);
    }

    #[test]
    fn test_house_layout() {
        let data = ReadHouseExcel::read_xlsx("upload/properties/erp1.xltx").unwrap();
        let layout = data[0].get_layout();
        println!("{:?}", layout);
    }

    #[test]
    fn test_house_layout2() {
        let layout: HouseLayout = "21室21厅52卫".to_string().into();
        println!("111 {:?}", layout);
    }

    #[test]
    fn test_get_num() {
        let num = "50↑".to_string();
        println!("111 {:?}", ReadHouseExcel::get_num(Some(num)));
    }
}
