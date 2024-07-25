use log::info;
use rust_raknet::Reliability;
use B_Mine_SDK::plugin::plugin::{PluginConfig, PluginTrait};

#[no_mangle]
pub fn _app_extend_create() -> *mut dyn PluginTrait {
    // 创建对象
    let object = MyPlugin::default();
    // 通过 Box 在堆上存储该对象实例
    let boxed: Box<dyn PluginTrait> = Box::new(object);
    // 返回原始指针（这是个 unsafe 调用，不过在 extern "C" 这种 ABI 定义处整个代码段都处于 unsafe 下，所以不用额外写 unsafe）
    Box::into_raw(boxed)
}

#[derive(Default)]
struct MyPlugin;

impl PluginTrait for MyPlugin {
    fn load(&self) -> B_Mine_SDK::plugin::plugin::PluginConfig {
        PluginConfig {
            name: "MyPlugin".to_string(),
            version: [1, 0, 0, 0],
            description: "测试插件".to_owned(),
        }
    }

    fn receive_packet(&self, data: &Vec<u8>) -> Option<(Vec<u8>, Reliability)> {
        info!("Receive packet: {:?}", data);
        Some((vec![254, 0, 1, 1], Reliability::Unreliable))
    }
}
