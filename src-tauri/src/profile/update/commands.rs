use uuid::Uuid;

use crate::{
    prefs::Prefs,
    profile::ModManager,
    thunderstore::ModId,
    util::cmd::{Result, StateMutex},
};

#[tauri::command]
pub async fn change_mod_version(mod_ref: ModId, app: tauri::AppHandle) -> Result<()> {
    super::change_version(mod_ref, &app).await?;

    Ok(())
}

#[tauri::command]
pub async fn update_mods(
    uuids: Vec<Uuid>,
    respect_ignored: bool,
    app: tauri::AppHandle,
) -> Result<()> {
    super::update_mods(uuids, respect_ignored, &app).await?;

    Ok(())
}

#[tauri::command]
pub fn ignore_update(
    version_uuid: Uuid,
    manager: StateMutex<ModManager>,
    prefs: StateMutex<Prefs>,
) -> Result<()> {
    let mut manager = manager.lock().unwrap();
    let prefs = prefs.lock().unwrap();

    manager
        .active_profile_mut()
        .ignored_updates
        .insert(version_uuid);

    manager.save(&prefs)?;

    Ok(())
}
