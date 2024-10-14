#version 410 core

in vec4 vertexColor;
in vec3 fragNormal;

out vec4 fragColor;

void main() {
    vec3 lightDir = normalize(vec3(0.0, 1.0, 1.0));
    float diff = max(dot(fragNormal, lightDir), 0.0);
    vec3 diffuse = diff * vec3(1.0, 1.0, 1.0);

    fragColor = vec4(vertexColor.rgb * diffuse, vertexColor.a);
}

