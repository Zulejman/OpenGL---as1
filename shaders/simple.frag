#version 330 core

in vec4 vertexColor;  
in vec3 vertexNormal; 

out vec4 FragColor;

void main()
{
    vec3 normal = normalize(vertexNormal);
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    float diffuse = max(dot(normal, -lightDirection), 0.0);
    vec3 color = vertexColor.rgb * diffuse;
    FragColor = vec4(color, vertexColor.a);
}

