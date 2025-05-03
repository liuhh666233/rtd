use crate::storage::{self, add_item as storage_add_item, update_item as storage_update_item};

pub fn add_item() {
    storage_add_item();
}

pub fn update_item() {
    storage_update_item();
}

pub fn get_all() {
    storage_update_item();
}

pub fn delete_item() {
    storage_update_item();
}

pub fn restore_item() {
    storage_update_item();
}

pub fn destroy_deleted() {
    storage_update_item();
}

pub fn destroy_item() {
    storage_update_item();
}

pub fn clear() {
    storage_update_item();
}

pub fn list_uncompleted() {
    storage_update_item();
}

pub fn list_completed() {
    storage_update_item();
}

pub fn list_all() {
    storage_update_item();
}

pub fn list_deleted() {
    storage_update_item();
}