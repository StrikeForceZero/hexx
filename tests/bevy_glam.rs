#[cfg(test)]
mod tests {
    use glam::IVec2 as GlamIVec2;
    use bevy::prelude::IVec2 as BevyIVec2;

    #[test]
    fn ensure_glam_version_matches_bevy() {
        // This will fail to build rather than the test itself
        assert_eq!(GlamIVec2::ONE, BevyIVec2::ONE, "glam version does not match bevy's")
    }
}
