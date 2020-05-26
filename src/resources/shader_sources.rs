/*pub static TEXTURED_VERTEX_SHADER: &str = r#"
#version 150 core

in vec3 inPosition;
in vec2 inTexCoords;

out vec2 TexCoords;

uniform mat4 model;
uniform mat4 model_inv;
uniform mat4 view;
uniform mat4 proj;

void main()
{
        gl_Position = proj * view * model * vec4(inPosition, 1.0);
        TexCoords = inTexCoords;
}
"#;

pub static TEXTURED_FRAGMENT_SHADER: &str = r#"
#version 150 core

in vec2 TexCoords;

out vec4 outColor;

uniform sampler2D tex;

void main()
{
        outColor = texture(tex, TexCoords * (1.0, -1.0));
}
"#;*/

pub static SOLID_VERTEX_SHADER: &str = r#"
#version 150 core

in vec3 inPosition;
in vec3 inNormal;

out vec3 Normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main() {
    gl_Position = proj * view * model * vec4(inPosition, 1.0);
    Normal = inNormal * inverse(mat3(view * model));
}
"#;

pub static SOLID_FRAGMENT_SHADER: &str = r#"
#version 150

in vec3 Normal;

out vec4 outColor;

uniform vec3 color;
uniform bool apply_diffuse;

void main() {
    float diffuse = 1.0;
    if (apply_diffuse) {
        diffuse = max(normalize(Normal).z, 0.0);
    }

    outColor = vec4(vec3(0.5 + 0.5*diffuse)*color, 1.0);
}
"#;
