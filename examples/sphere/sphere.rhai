let builder = new_builder();

// build semi circle
let cloud = create_circle_cloud(1.0, 16).resize(0, 8);
repeat(16) {
    let cloud2 = cloud.rotate(vec(0.0, 0.0, 1/16 * 2pi));
    builder.shell(cloud, cloud2);
    cloud = cloud2;
}

builder.merge_identical();
return builder.mesh;