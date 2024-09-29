#version 410 core


layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

out vec4 fragColor;

uniform mat4 transform_matrix;

void main()
{
    gl_Position = transform_matrix * vec4(position, 1.0f);
    fragColor = color;
}
