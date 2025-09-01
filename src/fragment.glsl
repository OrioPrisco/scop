#version 330 core
out vec4 FragColor;
in vec3 vertexColor;
in vec2 TexCoord;
in vec3 vertexNorm;
in vec3 FragPos;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform vec3 lightPos;

void main()
{
    vec4 tex2 = texture(texture2, TexCoord);
    vec4 tex2_final = (tex2 + vec4(vertexColor, 1.0)) * vec4(tex2.a, tex2.a, tex2.a, tex2.a);// seems like transparency isn't enabled'
    vec4 objColor = mix(texture(texture1, TexCoord), tex2_final, 0.2);
    vec3 lightDir = normalize(lightPos - FragPos);
    vec3 norm = normalize(vertexNorm);
    vec4 lightColor = max(dot(norm, lightDir), 0.0) * vec4(1.0, 1.0, 1.0, 1.0) + vec4(0.1, 0.1, 0.1, 1.0);
    FragColor = lightColor * objColor;
}
