#version 330 core
out vec4 FragColor;  

in vec2 TexCoord;
in vec3 position;

uniform sampler2D texture_map;
uniform float time;

void main() {
    FragColor = texture(texture_map, vec2((TexCoord.x + 2.0) / 4.0, TexCoord.y));
    FragColor = vec4(FragColor.rgb * vec3(cos(time), 1.0, sin(time)), 1.0);
    if (FragColor.a < 0.1)
        discard;
}
