#version 330 core
out vec4 FragColor;  

in vec2 TexCoord;
in vec3 position;
in vec3 aCoord;

uniform sampler2D texture_map;

void main() {
    vec4 color = texture(texture_map, vec2((TexCoord.x + 2.0) / 4.0, TexCoord.y));
    if (color.a < 0.1)
        discard;
    FragColor = vec4(vec3(color.r * aCoord.x, color.g, color.b * aCoord.z), 1.0);
}