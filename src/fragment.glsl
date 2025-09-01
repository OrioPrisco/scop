#version 330 core
out vec4 FragColor;
in vec3 vertexColor;
in vec2 TexCoord;
in vec3 vertexNorm;
in vec3 FragPos;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform vec3 lightPos;
uniform float TextureOrColor;

void main()
{
    vec4 objColor = texture(texture1, TexCoord);
    vec3 lightDir = normalize(lightPos - FragPos);
    vec3 norm = normalize(vertexNorm);
    vec4 lightColor = max(dot(norm, lightDir), 0.0) * vec4(1.0, 1.0, 1.0, 1.0) + vec4(0.3, 0.3, 0.3, 1.0);
    FragColor = mix(lightColor * objColor, + vec4(vertexColor, 1.0), TextureOrColor);
}
