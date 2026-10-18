use crate::{
    editor::change_list::{
        EditorChangeList,
        EditorChangeType,
    },
    level::level_loader::LevelData,
};

pub fn file_changes_apply(change_list: &EditorChangeList) {
    println!("Working directory: {:?}", std::env::current_dir().unwrap());
    
    let file = std::fs::read_to_string(&change_list.file_name).expect("File not found!");

    let mut level: LevelData = ron::from_str(&file).expect("Failed to load level from file!");

    for change in &change_list.changes {
        match change.change_type {
            EditorChangeType::Add => {
                level.objects.push(change.object.clone().expect("Cannot add None object."));
            },
            EditorChangeType::Remove => {
                todo!();
            },
        }
    }

    let updated = ron::ser::to_string_pretty(
        &level,
        ron::ser::PrettyConfig::default()
    ).expect("Couldn't convert level object to RON string!");

    std::fs::write(&change_list.file_name, updated).expect("Couldn't write to file!");
}