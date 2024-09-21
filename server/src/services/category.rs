use std::collections::HashMap;
use crate::persist::category::Category;



pub async fn filter_inactive_categories(it: Vec<(i32, Category)>) -> Vec<i32> {
    let mut cached_categories = HashMap::<Category, bool>::new();

    let mut active_it = Vec::with_capacity(it.len());

    for (id, category) in it {
        if let Some(category_status) = cached_categories.get(&category) {
            if category_status.to_owned() {
                active_it.push(id);
            }
        } else {
            let resp = reqwest::get(&category.url).await;

            if let Ok(resp) = resp {
                let status = resp.status();

                if status.is_success() {
                    cached_categories.insert(category.clone(), true);
                    active_it.push(id);
                } else {
                    error!("Failed to get category: {}", category);
                    cached_categories.insert(category.clone(), false);
                }
            } else {
                error!("Failed to get category: {}", category);
            }
        }
    }

    vec![]
}