use pipewire::{
    context::Context, main_loop::MainLoop, registry::GlobalObject, spa::utils::dict::DictRef,
};

const MEDIA_CLASS_KEY: &str = "media.class";
const DEVICE_DESCRIPTION_KEY: &str = "device.description";
const MEDIA_CLASS_AUDIO_DEVICE: &str = "Audio/Device";

fn enumerate_audio_devices(global: &GlobalObject<&DictRef>) {
    if let Some(props) = global.props {
        if let Some(media_class) = props.get(MEDIA_CLASS_KEY) {
            if media_class == MEDIA_CLASS_AUDIO_DEVICE {
                println!(
                    "New Device/Audio: {:?}",
                    props.get(DEVICE_DESCRIPTION_KEY).unwrap()
                )
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mainloop = MainLoop::new(None)?;
    let context = Context::new(&mainloop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    // Register a callback to the `global` event on the registry, which notifies of any new global
    // objects appearing on the remote. The callback will only get called as long as we keep the
    // returned listener alive.
    let _listener = registry
        .add_listener_local()
        .global(enumerate_audio_devices)
        .register();

    mainloop.run();

    Ok(())
}
