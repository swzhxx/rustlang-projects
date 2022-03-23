precision mediump float;
uniform sampler2D u_Sampler;
uniform float u_Iter;
varying vec2 v_TexCoord;
const vec2 offset = vec2(0.5, 0.5);
void main() {
  vec2 coord = v_TexCoord - offset;
  float l = length(coord);
  float x = coord.x;
  float y = coord.y;
  coord.x = x * cos(u_Iter * l) - y * sin(u_Iter * l);
  coord.y = y * sin(u_Iter * l) + y * cos(u_Iter * l);
  coord = coord + offset;
  gl_FragColor = texture2D(u_Sampler, coord);
}