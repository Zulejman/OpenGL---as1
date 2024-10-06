#version 430 core

layout(location = 1) in vec4 vertexColor;
layout(location = 2) in vec3 vertexNormal;
layout(location = 3) in vec3 surfaceNormal;
layout(location = 4) in vec3 surfacePosition;

layout(location = 0) out vec4 fragColor;
layout(location = 1) out vec3 fragNormal;

void main()
{
    vec3 lightSourcePos = vec3(0.0, 40.0, 40.0);
    vec3 lightIntensity = vec3(1.0, 1.0, 1.0);

    vec3 normalizedSurfaceNormal = normalize(surfaceNormal);

    vec3 directionToLight = normalize(lightSourcePos - surfacePosition);
    float brightness = max(0.0, dot(normalizedSurfaceNormal, directionToLight));

    vec3 diffuseComponent = brightness * lightIntensity;
    fragColor = vec4(diffuseComponent, 1.0) * vertexColor;
}
