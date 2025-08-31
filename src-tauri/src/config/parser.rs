use ahash::AHashMap;

use crate::{
    config::model::{FlatStorage, StoredMacro, StoredProfile},
    domain::{config::Config, macros::Macro, profiles::Profile, sequence_step::SequenceStep},
};

#[inline]
pub fn config_to_flat(config: &Config) -> FlatStorage {
    let mut profiles = AHashMap::new();
    let mut macros = AHashMap::new();
    let mut steps = AHashMap::new();

    for profile in &config.profiles {
        // 1.1  Pasos anidados → tabla plana
        for mac in &profile.macros {
            for step in &mac.sequence {
                steps.insert(step_id(step), step.clone());
            }
            macros.insert(
                mac.id,
                StoredMacro {
                    id: mac.id,
                    name: mac.name.clone(),
                    trigger: mac.trigger.clone(),
                    sequence_step_ids: mac.sequence.iter().map(step_id).collect(),
                },
            );
        }

        // 1.2  Perfil sin macros reales
        profiles.insert(
            profile.id,
            StoredProfile {
                id: profile.id,
                name: profile.name.clone(),
                function_key: profile.function_key.clone(),
                macro_ids: profile.macros.iter().map(|m| m.id).collect(),
            },
        );
    }

    FlatStorage {
        profiles,
        macros,
        steps,
        selected_profile_id: config.selected_profile_id,
    }
}

#[inline]
pub fn flat_to_config(storage: &FlatStorage) -> Config {
    let mut profiles = Vec::new();

    for (_, stored_profile) in &storage.profiles {
        let macros: Vec<Macro> = stored_profile
            .macro_ids
            .iter()
            .filter_map(|mid| storage.macros.get(mid))
            .map(|stored_macro| Macro {
                id: stored_macro.id,
                name: stored_macro.name.clone(),
                trigger: stored_macro.trigger.clone(),
                sequence: stored_macro
                    .sequence_step_ids
                    .iter()
                    .filter_map(|sid| storage.steps.get(sid).cloned())
                    .collect(),
            })
            .collect();

        profiles.push(Profile {
            id: stored_profile.id,
            name: stored_profile.name.clone(),
            function_key: stored_profile.function_key.clone(),
            macros,
        });
    }

    Config {
        profiles,
        selected_profile_id: storage.selected_profile_id,
    }
}

#[inline(always)]
fn step_id(step: &SequenceStep) -> u64 {
    match step {
        SequenceStep::KeyDown { id, .. }
        | SequenceStep::KeyUp { id, .. }
        | SequenceStep::Delay { id, .. } => *id,
    }
}
