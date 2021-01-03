#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord; 

out vec2 TexCoord;
out vec3 position;
out vec3 aCoord;

uniform mat4 view;
uniform mat4 projection;
uniform float time;

void main() {
    gl_Position = projection * view * vec4(vec3(aPos.x, aPos.y - cos(time) / 3.0, aPos.z), 1.0);
    TexCoord = aTexCoord;
    position = aPos;
    aCoord = vec3(sin(time), aPos.y, cos(time));
    }