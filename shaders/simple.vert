layout(location = 0) in vec3 vertexPosition;
layout(location = 1) in vec4 vertexColor;
layout(location = 2) in vec3 vertexNormal;

layout(location = 1) out vec4 fragColor;
layout(location = 2) out vec3 worldNormal;
layout(location = 3) out vec3 normalInterpolated;
layout(location = 4) out vec3 worldPosition;

uniform mat4 modelViewProjection;
uniform mat4 transformationMatrix;

void main()
{
    vec4 worldPos = vec4(vertexPosition, 1.0);
    gl_Position = modelViewProjection * worldPos;

    normalInterpolated = normalize(mat3(transpose(inverse(transformationMatrix))) * vertexNormal);
    worldPosition = vec3(transformationMatrix * worldPos);

    fragColor = vertexColor;
    worldNormal = vertexNormal;
}

