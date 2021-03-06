
#[cfg(test)]
mod audio_test {
    use raylib::prelude::*;
    use crate::tests::*;
    #[test]
    fn test_init_audio() {
        let _ = RaylibAudio::init_audio_device();
    }
    #[test]
    fn test_load_wave() {
        let w = Wave::load_wave("resources/audio/wave.ogg").expect("wave loading failed");
        w.export_wave("test_out/wave.wav");
        w.export_wave_as_code("test_out/wave.h");
    }

    ray_test!(test_load_music);
    fn test_load_music(_thread: &RaylibThread) {
        // TODO uncomment when music is fixed
        // {
        //     let _ = Music::load_music_stream(thread, "resources/audio/chiptun1.mod")
        //         .expect("could not load music");
        // }
    }
}
