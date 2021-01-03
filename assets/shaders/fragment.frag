#version 330 core
out vec4 FragColor;  

in vec2 TexCoord;
in vec3 position;

uniform sampler2D texture_map;

void main() {
    FragColor = texture(texture_map, vec2((TexCoord.x + 2.0) / 6.0, TexCoord.y));
    if (FragColor.a < 0.1)
        discard;
}
