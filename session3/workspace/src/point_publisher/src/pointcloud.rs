use std::intrinsics::size_of;

use builtin_interfaces::msg::Time;
use sensor_msgs::msg::PointCloud2;


#[repr(C)]
struct PointXYZ {
  x: f32,
  y: f32,
  z: f32
}

static happy_points: [PointXYZ; 9] = [
  PointXYZ { x:-3.0,y: 3.0, z:0.0 },
  PointXYZ { x:3.0,y: 3.0, z:0.0 },
  PointXYZ { x:-3.0,y: -2.0,z: 0.0 },
  PointXYZ { x:3.0,y: -2.0,z: 0.0 },
  PointXYZ { x:-2.0,y: -3.0,z: 0.0 },
  PointXYZ { x:-1.0,y: -3.0,z: 0.0 },
  PointXYZ { x:0.0,y: -3.0,z: 0.0 },
  PointXYZ { x:1.0,y: -3.0,z: 0.0 },
  PointXYZ { x:2.0,y: -3.0,z: 0.0 }
];

static unhappy_points: [PointXYZ; 9] = [
  PointXYZ { x:-3.0,y: 3.0, z:0.0 },
  PointXYZ { x:3.0,y: 3.0, z:0.0 },
  PointXYZ { x:-3.0,y: -3.0,z: 0.0 },
  PointXYZ { x:3.0,y: -3.0,z: 0.0 },
  PointXYZ { x:-2.0,y: -2.0,z: 0.0 },
  PointXYZ { x:-1.0,y: -2.0,z: 0.0 },
  PointXYZ { x:0.0,y: -2.0,z: 0.0 },
  PointXYZ { x:1.0,y: -2.0,z: 0.0 },
  PointXYZ { x:2.0,y: -2.0,z: 0.0 }
];

pub fn get_pointcloud(happy: bool) -> PointCloud2 {
    let mut pointcloud = PointCloud2 {
        header: std_msgs::msg::Header {
            stamp: {Time { sec: 0, nanosec: 0 }},
            frame_id: "my_frame".to_owned(),
        },
        height: 1,
        width: 9,
        fields: todo!(),
        is_bigendian: false,
        point_step: 12,
        row_step: 12 * 9,
        data: todo!(),
        is_dense: false,
    };
}
