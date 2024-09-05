use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    #[serde(rename(deserialize = "标签"))]
    tag: Option<i32>,
    #[serde(rename(deserialize = "类型"))]
    r#type: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook("upload/properties/erp1.xltx").unwrap();
    let sheet_names = workbook.sheet_names();
    for sheet_name in sheet_names {
        // Read whole worksheet data and provide some statistics
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let iter_records =
                RangeDeserializerBuilder::with_headers(&["标签", "类型"]).from_range(&range)?;

            for result in iter_records {
                let record: Record = result?;
                println!("{:?}", record);
            }
        }
    }

    Ok(())
}
