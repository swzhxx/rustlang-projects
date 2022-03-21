#version 100
precision mediump float;

attribute vec2 a_TexCoord;
attribute vec4 a_Position;
varying vec2 v_TexCoord;
void main() {
  gl_Position = a_Position;
  v_TexCoord = a_TexCoord;
}