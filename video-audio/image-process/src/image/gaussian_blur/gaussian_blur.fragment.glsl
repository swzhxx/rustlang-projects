#define u_KernelSize {kernel_size}
precision mediump float;
uniform bool u_Horizontal;
uniform sampler2D u_Sampler;
uniform vec2 u_TextureSize;
varying vec2 v_TexCoord;
uniform float u_Kernel[u_KernelSize];
const int center = u_KernelSize / 2 + 1;
void main() {
  vec3 color = texture2D(u_Sampler, v_TexCoord).rgb * u_Kernel[center];
  vec2 one_pixel = vec2(1.0, 1.0) / u_TextureSize;
  if(!u_Horizontal) {
    for(int offset = 1; offset <= u_KernelSize - center; offset++) {
      color += texture2D(u_Sampler, v_TexCoord + vec2(0, float(offset) * one_pixel.y)).rgb * u_Kernel[center - 1 - offset];
      color += texture2D(u_Sampler, v_TexCoord - vec2(0, float(offset) * one_pixel.y)).rgb * u_Kernel[center - 1 - offset];
    }
  } else {
    for(int offset = 1; offset <= u_KernelSize - center; offset++) {
      color += texture2D(u_Sampler, v_TexCoord + vec2(float(offset) * one_pixel.x, 0)).rgb * u_Kernel[center - 1 - offset];
      color += texture2D(u_Sampler, v_TexCoord - vec2(float(offset) * one_pixel.x, 0)).rgb * u_Kernel[center - 1 - offset];
    }
  }
  gl_FragColor = vec4(color, 1.);
}
