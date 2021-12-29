pub static vertex_shader_src: &'static str = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;
        out vec3 v_normal;
        out vec3 v_position;
        out vec2 v_tex_coords;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        void main() {
            v_tex_coords = tex_coords;
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;



pub static fragment_shader_custom_light_src: &'static str = r#"
        #version 140
        uniform float intensity;
        uniform sampler2D diffuse_tex;
        in vec2 v_tex_coords;
        in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;

        void main() {
            vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
            color = vec4(intensity * diffuse_color, 1.0);
        }
    "#;