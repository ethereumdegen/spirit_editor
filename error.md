thread 'Compute Task Pool (1)' panicked at /home/andy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.13.2/src/system/system_param.rs:477:17:
Resource requested by main::asset_loading::load_magic_fx does not exist: main::asset_loading::MagicFxVariantAssets
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Encountered a panic in system `main::asset_loading::load_magic_fx`!
Encountered a panic in exclusive system `bevy_ecs::schedule::state::apply_state_transition<main::asset_loading::AssetLoadState>`!
Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
ALSA lib pcm.c:8526:(snd_pcm_recover) underrun occurred
