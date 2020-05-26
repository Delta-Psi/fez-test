use gl::types::*;

// square, normal to the z axis
pub static SQUARE_FACES: &[[GLfloat; 5]] = &[
    [-0.5, -0.5,  0.0,   0.0, 0.0],
    [-0.5,  0.5,  0.0,   0.0, 1.0],
    [ 0.5, -0.5,  0.0,   1.0, 0.0],

    [-0.5,  0.5,  0.0,   0.0, 1.0],
    [ 0.5, -0.5,  0.0,   1.0, 0.0],
    [ 0.5,  0.5,  0.0,   1.0, 1.0],
];

// triangles
pub static PLATFORM_FACES: &[[GLfloat; 6]] = &[
    [-0.5,  0.5,  0.5,   -1.0,  0.0,  0.0],
    [-0.5,  0.5, -0.5,   -1.0,  0.0,  0.0],
    [-0.5, -0.5, -0.5,   -1.0,  0.0,  0.0],
    [-0.5, -0.5, -0.5,   -1.0,  0.0,  0.0],
    [-0.5, -0.5,  0.5,   -1.0,  0.0,  0.0],
    [-0.5,  0.5,  0.5,   -1.0,  0.0,  0.0],

    [ 0.5,  0.5,  0.5,    1.0,  0.0,  0.0],
    [ 0.5,  0.5, -0.5,    1.0,  0.0,  0.0],
    [ 0.5, -0.5, -0.5,    1.0,  0.0,  0.0],
    [ 0.5, -0.5, -0.5,    1.0,  0.0,  0.0],
    [ 0.5, -0.5,  0.5,    1.0,  0.0,  0.0],
    [ 0.5,  0.5,  0.5,    1.0,  0.0,  0.0],

    [-0.5, -0.5, -0.5,    0.0, -1.0,  0.0],
    [ 0.5, -0.5, -0.5,    0.0, -1.0,  0.0],
    [ 0.5, -0.5,  0.5,    0.0, -1.0,  0.0],
    [ 0.5, -0.5,  0.5,    0.0, -1.0,  0.0],
    [-0.5, -0.5,  0.5,    0.0, -1.0,  0.0],
    [-0.5, -0.5, -0.5,    0.0, -1.0,  0.0],

    [-0.5,  0.5, -0.5,    0.0,  1.0,  0.0],
    [ 0.5,  0.5, -0.5,    0.0,  1.0,  0.0],
    [ 0.5,  0.5,  0.5,    0.0,  1.0,  0.0],
    [ 0.5,  0.5,  0.5,    0.0,  1.0,  0.0],
    [-0.5,  0.5,  0.5,    0.0,  1.0,  0.0],
    [-0.5,  0.5, -0.5,    0.0,  1.0,  0.0],
];
