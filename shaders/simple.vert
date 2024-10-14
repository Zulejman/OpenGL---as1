#version 410 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 normal;

uniform mat4 MVP;
uniform mat4 model_matrix;

out vec4 vertexColor;
out vec3 fragNormal;

void main() {
    gl_Position = MVP * vec4(position, 1.0);
    fragNormal = normalize(mat3(model_matrix) * normal);
    vertexColor = color;
}

