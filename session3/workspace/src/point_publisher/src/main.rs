mod happiness;
mod pointcloud;

use happiness::is_happy;
use pointcloud::get_pointcloud;

use rclrs;
use sensor_msgs::msg::PointCloud2;
use std::sync::{Arc, Mutex};
use std_msgs::msg::String as StringMsg;


fn main() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;

    let node = rclrs::Node::new(&context, "point_publisher")?;
    let publisher: Arc<rclrs::Publisher<PointCloud2>> = node.create_publisher("points", rclrs::QOS_PROFILE_DEFAULT)?;
    // let publisher = Arc::new(Mutex::new(*publisher));

    let subscription: Arc<rclrs::Subscription<StringMsg>> = node.create_subscription(
        "compliment",
        rclrs::QOS_PROFILE_DEFAULT,
        |msg: StringMsg| {
            let is_happy = is_happy(&msg.data);
            let message: PointCloud2 = get_pointcloud(is_happy);
            // *publisher.lock().unwrap().publish(message)
        },
    )?;

    rclrs::spin(node)
}
