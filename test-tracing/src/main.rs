mod custom_layer;

use custom_layer::CustomLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    // 设置 `tracing-subscriber` 对 tracing 数据的处理方式
    tracing_subscriber::registry().with(CustomLayer).init();
    // 打印一条简单的日志。用 `tracing` 的行话来说，`info!` 将创建一个事件
    info!(a_bool = true, answer = 42, message = "first example");
}
