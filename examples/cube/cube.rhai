let builder = new_builder();

// build some point clouds
let square = create_square_cloud(1.0, 1.0);
let extrusion = square.translate(vec(0.0, 1.0, 0.0));

// create faces and shell
builder.face(square);
builder.shell(square, extrusion);
builder.face(extrusion);

// build final mesh
return builder.mesh;