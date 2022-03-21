precision mediump float;
uniform sampler2D u_Sampler;
varying vec2 v_TexCoord;
void main() {
  vec4 color = texture2D(u_Sampler, v_TexCoord);
  float gray = color.r * 0.3 + color.g * 0.59 + color.b * 0.11;
  gl_FragColor = vec4(gray, gray, gray, color.a);
}