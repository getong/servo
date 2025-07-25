/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
//
// WebGL IDL definitions scraped from the Khronos specification:
// https://www.khronos.org/registry/webgl/specs/latest/
//
// This IDL depends on the typed array specification defined at:
// https://www.khronos.org/registry/typedarray/specs/latest/typedarrays.idl

typedef long long GLint64;
typedef unsigned long long GLuint64;

typedef (/*[AllowShared]*/ Uint32Array or sequence<GLuint>) Uint32List;

interface mixin WebGL2RenderingContextBase
{
  const GLenum READ_BUFFER                                   = 0x0C02;
  const GLenum UNPACK_ROW_LENGTH                             = 0x0CF2;
  const GLenum UNPACK_SKIP_ROWS                              = 0x0CF3;
  const GLenum UNPACK_SKIP_PIXELS                            = 0x0CF4;
  const GLenum PACK_ROW_LENGTH                               = 0x0D02;
  const GLenum PACK_SKIP_ROWS                                = 0x0D03;
  const GLenum PACK_SKIP_PIXELS                              = 0x0D04;
  const GLenum COLOR                                         = 0x1800;
  const GLenum DEPTH                                         = 0x1801;
  const GLenum STENCIL                                       = 0x1802;
  const GLenum RED                                           = 0x1903;
  const GLenum RGB8                                          = 0x8051;
  const GLenum RGBA8                                         = 0x8058;
  const GLenum RGB10_A2                                      = 0x8059;
  const GLenum TEXTURE_BINDING_3D                            = 0x806A;
  const GLenum UNPACK_SKIP_IMAGES                            = 0x806D;
  const GLenum UNPACK_IMAGE_HEIGHT                           = 0x806E;
  const GLenum TEXTURE_3D                                    = 0x806F;
  const GLenum TEXTURE_WRAP_R                                = 0x8072;
  const GLenum MAX_3D_TEXTURE_SIZE                           = 0x8073;
  const GLenum UNSIGNED_INT_2_10_10_10_REV                   = 0x8368;
  const GLenum MAX_ELEMENTS_VERTICES                         = 0x80E8;
  const GLenum MAX_ELEMENTS_INDICES                          = 0x80E9;
  const GLenum TEXTURE_MIN_LOD                               = 0x813A;
  const GLenum TEXTURE_MAX_LOD                               = 0x813B;
  const GLenum TEXTURE_BASE_LEVEL                            = 0x813C;
  const GLenum TEXTURE_MAX_LEVEL                             = 0x813D;
  const GLenum MIN                                           = 0x8007;
  const GLenum MAX                                           = 0x8008;
  const GLenum DEPTH_COMPONENT24                             = 0x81A6;
  const GLenum MAX_TEXTURE_LOD_BIAS                          = 0x84FD;
  const GLenum TEXTURE_COMPARE_MODE                          = 0x884C;
  const GLenum TEXTURE_COMPARE_FUNC                          = 0x884D;
  const GLenum CURRENT_QUERY                                 = 0x8865;
  const GLenum QUERY_RESULT                                  = 0x8866;
  const GLenum QUERY_RESULT_AVAILABLE                        = 0x8867;
  const GLenum STREAM_READ                                   = 0x88E1;
  const GLenum STREAM_COPY                                   = 0x88E2;
  const GLenum STATIC_READ                                   = 0x88E5;
  const GLenum STATIC_COPY                                   = 0x88E6;
  const GLenum DYNAMIC_READ                                  = 0x88E9;
  const GLenum DYNAMIC_COPY                                  = 0x88EA;
  const GLenum MAX_DRAW_BUFFERS                              = 0x8824;
  const GLenum DRAW_BUFFER0                                  = 0x8825;
  const GLenum DRAW_BUFFER1                                  = 0x8826;
  const GLenum DRAW_BUFFER2                                  = 0x8827;
  const GLenum DRAW_BUFFER3                                  = 0x8828;
  const GLenum DRAW_BUFFER4                                  = 0x8829;
  const GLenum DRAW_BUFFER5                                  = 0x882A;
  const GLenum DRAW_BUFFER6                                  = 0x882B;
  const GLenum DRAW_BUFFER7                                  = 0x882C;
  const GLenum DRAW_BUFFER8                                  = 0x882D;
  const GLenum DRAW_BUFFER9                                  = 0x882E;
  const GLenum DRAW_BUFFER10                                 = 0x882F;
  const GLenum DRAW_BUFFER11                                 = 0x8830;
  const GLenum DRAW_BUFFER12                                 = 0x8831;
  const GLenum DRAW_BUFFER13                                 = 0x8832;
  const GLenum DRAW_BUFFER14                                 = 0x8833;
  const GLenum DRAW_BUFFER15                                 = 0x8834;
  const GLenum MAX_FRAGMENT_UNIFORM_COMPONENTS               = 0x8B49;
  const GLenum MAX_VERTEX_UNIFORM_COMPONENTS                 = 0x8B4A;
  const GLenum SAMPLER_3D                                    = 0x8B5F;
  const GLenum SAMPLER_2D_SHADOW                             = 0x8B62;
  const GLenum FRAGMENT_SHADER_DERIVATIVE_HINT               = 0x8B8B;
  const GLenum PIXEL_PACK_BUFFER                             = 0x88EB;
  const GLenum PIXEL_UNPACK_BUFFER                           = 0x88EC;
  const GLenum PIXEL_PACK_BUFFER_BINDING                     = 0x88ED;
  const GLenum PIXEL_UNPACK_BUFFER_BINDING                   = 0x88EF;
  const GLenum FLOAT_MAT2x3                                  = 0x8B65;
  const GLenum FLOAT_MAT2x4                                  = 0x8B66;
  const GLenum FLOAT_MAT3x2                                  = 0x8B67;
  const GLenum FLOAT_MAT3x4                                  = 0x8B68;
  const GLenum FLOAT_MAT4x2                                  = 0x8B69;
  const GLenum FLOAT_MAT4x3                                  = 0x8B6A;
  const GLenum SRGB                                          = 0x8C40;
  const GLenum SRGB8                                         = 0x8C41;
  const GLenum SRGB8_ALPHA8                                  = 0x8C43;
  const GLenum COMPARE_REF_TO_TEXTURE                        = 0x884E;
  const GLenum RGBA32F                                       = 0x8814;
  const GLenum RGB32F                                        = 0x8815;
  const GLenum RGBA16F                                       = 0x881A;
  const GLenum RGB16F                                        = 0x881B;
  const GLenum VERTEX_ATTRIB_ARRAY_INTEGER                   = 0x88FD;
  const GLenum MAX_ARRAY_TEXTURE_LAYERS                      = 0x88FF;
  const GLenum MIN_PROGRAM_TEXEL_OFFSET                      = 0x8904;
  const GLenum MAX_PROGRAM_TEXEL_OFFSET                      = 0x8905;
  const GLenum MAX_VARYING_COMPONENTS                        = 0x8B4B;
  const GLenum TEXTURE_2D_ARRAY                              = 0x8C1A;
  const GLenum TEXTURE_BINDING_2D_ARRAY                      = 0x8C1D;
  const GLenum R11F_G11F_B10F                                = 0x8C3A;
  const GLenum UNSIGNED_INT_10F_11F_11F_REV                  = 0x8C3B;
  const GLenum RGB9_E5                                       = 0x8C3D;
  const GLenum UNSIGNED_INT_5_9_9_9_REV                      = 0x8C3E;
  const GLenum TRANSFORM_FEEDBACK_BUFFER_MODE                = 0x8C7F;
  const GLenum MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS    = 0x8C80;
  const GLenum TRANSFORM_FEEDBACK_VARYINGS                   = 0x8C83;
  const GLenum TRANSFORM_FEEDBACK_BUFFER_START               = 0x8C84;
  const GLenum TRANSFORM_FEEDBACK_BUFFER_SIZE                = 0x8C85;
  const GLenum TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN         = 0x8C88;
  const GLenum RASTERIZER_DISCARD                            = 0x8C89;
  const GLenum MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS = 0x8C8A;
  const GLenum MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS       = 0x8C8B;
  const GLenum INTERLEAVED_ATTRIBS                           = 0x8C8C;
  const GLenum SEPARATE_ATTRIBS                              = 0x8C8D;
  const GLenum TRANSFORM_FEEDBACK_BUFFER                     = 0x8C8E;
  const GLenum TRANSFORM_FEEDBACK_BUFFER_BINDING             = 0x8C8F;
  const GLenum RGBA32UI                                      = 0x8D70;
  const GLenum RGB32UI                                       = 0x8D71;
  const GLenum RGBA16UI                                      = 0x8D76;
  const GLenum RGB16UI                                       = 0x8D77;
  const GLenum RGBA8UI                                       = 0x8D7C;
  const GLenum RGB8UI                                        = 0x8D7D;
  const GLenum RGBA32I                                       = 0x8D82;
  const GLenum RGB32I                                        = 0x8D83;
  const GLenum RGBA16I                                       = 0x8D88;
  const GLenum RGB16I                                        = 0x8D89;
  const GLenum RGBA8I                                        = 0x8D8E;
  const GLenum RGB8I                                         = 0x8D8F;
  const GLenum RED_INTEGER                                   = 0x8D94;
  const GLenum RGB_INTEGER                                   = 0x8D98;
  const GLenum RGBA_INTEGER                                  = 0x8D99;
  const GLenum SAMPLER_2D_ARRAY                              = 0x8DC1;
  const GLenum SAMPLER_2D_ARRAY_SHADOW                       = 0x8DC4;
  const GLenum SAMPLER_CUBE_SHADOW                           = 0x8DC5;
  const GLenum UNSIGNED_INT_VEC2                             = 0x8DC6;
  const GLenum UNSIGNED_INT_VEC3                             = 0x8DC7;
  const GLenum UNSIGNED_INT_VEC4                             = 0x8DC8;
  const GLenum INT_SAMPLER_2D                                = 0x8DCA;
  const GLenum INT_SAMPLER_3D                                = 0x8DCB;
  const GLenum INT_SAMPLER_CUBE                              = 0x8DCC;
  const GLenum INT_SAMPLER_2D_ARRAY                          = 0x8DCF;
  const GLenum UNSIGNED_INT_SAMPLER_2D                       = 0x8DD2;
  const GLenum UNSIGNED_INT_SAMPLER_3D                       = 0x8DD3;
  const GLenum UNSIGNED_INT_SAMPLER_CUBE                     = 0x8DD4;
  const GLenum UNSIGNED_INT_SAMPLER_2D_ARRAY                 = 0x8DD7;
  const GLenum DEPTH_COMPONENT32F                            = 0x8CAC;
  const GLenum DEPTH32F_STENCIL8                             = 0x8CAD;
  const GLenum FLOAT_32_UNSIGNED_INT_24_8_REV                = 0x8DAD;
  const GLenum FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING         = 0x8210;
  const GLenum FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE         = 0x8211;
  const GLenum FRAMEBUFFER_ATTACHMENT_RED_SIZE               = 0x8212;
  const GLenum FRAMEBUFFER_ATTACHMENT_GREEN_SIZE             = 0x8213;
  const GLenum FRAMEBUFFER_ATTACHMENT_BLUE_SIZE              = 0x8214;
  const GLenum FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE             = 0x8215;
  const GLenum FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE             = 0x8216;
  const GLenum FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE           = 0x8217;
  const GLenum FRAMEBUFFER_DEFAULT                           = 0x8218;
  // BUG: https://github.com/KhronosGroup/WebGL/issues/2216
  // const GLenum DEPTH_STENCIL_ATTACHMENT                      = 0x821A;
  // const GLenum DEPTH_STENCIL                                 = 0x84F9;
  const GLenum UNSIGNED_INT_24_8                             = 0x84FA;
  const GLenum DEPTH24_STENCIL8                              = 0x88F0;
  const GLenum UNSIGNED_NORMALIZED                           = 0x8C17;
  const GLenum DRAW_FRAMEBUFFER_BINDING                      = 0x8CA6; /* Same as FRAMEBUFFER_BINDING */
  const GLenum READ_FRAMEBUFFER                              = 0x8CA8;
  const GLenum DRAW_FRAMEBUFFER                              = 0x8CA9;
  const GLenum READ_FRAMEBUFFER_BINDING                      = 0x8CAA;
  const GLenum RENDERBUFFER_SAMPLES                          = 0x8CAB;
  const GLenum FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER          = 0x8CD4;
  const GLenum MAX_COLOR_ATTACHMENTS                         = 0x8CDF;
  const GLenum COLOR_ATTACHMENT1                             = 0x8CE1;
  const GLenum COLOR_ATTACHMENT2                             = 0x8CE2;
  const GLenum COLOR_ATTACHMENT3                             = 0x8CE3;
  const GLenum COLOR_ATTACHMENT4                             = 0x8CE4;
  const GLenum COLOR_ATTACHMENT5                             = 0x8CE5;
  const GLenum COLOR_ATTACHMENT6                             = 0x8CE6;
  const GLenum COLOR_ATTACHMENT7                             = 0x8CE7;
  const GLenum COLOR_ATTACHMENT8                             = 0x8CE8;
  const GLenum COLOR_ATTACHMENT9                             = 0x8CE9;
  const GLenum COLOR_ATTACHMENT10                            = 0x8CEA;
  const GLenum COLOR_ATTACHMENT11                            = 0x8CEB;
  const GLenum COLOR_ATTACHMENT12                            = 0x8CEC;
  const GLenum COLOR_ATTACHMENT13                            = 0x8CED;
  const GLenum COLOR_ATTACHMENT14                            = 0x8CEE;
  const GLenum COLOR_ATTACHMENT15                            = 0x8CEF;
  const GLenum FRAMEBUFFER_INCOMPLETE_MULTISAMPLE            = 0x8D56;
  const GLenum MAX_SAMPLES                                   = 0x8D57;
  const GLenum HALF_FLOAT                                    = 0x140B;
  const GLenum RG                                            = 0x8227;
  const GLenum RG_INTEGER                                    = 0x8228;
  const GLenum R8                                            = 0x8229;
  const GLenum RG8                                           = 0x822B;
  const GLenum R16F                                          = 0x822D;
  const GLenum R32F                                          = 0x822E;
  const GLenum RG16F                                         = 0x822F;
  const GLenum RG32F                                         = 0x8230;
  const GLenum R8I                                           = 0x8231;
  const GLenum R8UI                                          = 0x8232;
  const GLenum R16I                                          = 0x8233;
  const GLenum R16UI                                         = 0x8234;
  const GLenum R32I                                          = 0x8235;
  const GLenum R32UI                                         = 0x8236;
  const GLenum RG8I                                          = 0x8237;
  const GLenum RG8UI                                         = 0x8238;
  const GLenum RG16I                                         = 0x8239;
  const GLenum RG16UI                                        = 0x823A;
  const GLenum RG32I                                         = 0x823B;
  const GLenum RG32UI                                        = 0x823C;
  const GLenum VERTEX_ARRAY_BINDING                          = 0x85B5;
  const GLenum R8_SNORM                                      = 0x8F94;
  const GLenum RG8_SNORM                                     = 0x8F95;
  const GLenum RGB8_SNORM                                    = 0x8F96;
  const GLenum RGBA8_SNORM                                   = 0x8F97;
  const GLenum SIGNED_NORMALIZED                             = 0x8F9C;
  const GLenum COPY_READ_BUFFER                              = 0x8F36;
  const GLenum COPY_WRITE_BUFFER                             = 0x8F37;
  const GLenum COPY_READ_BUFFER_BINDING                      = 0x8F36; /* Same as COPY_READ_BUFFER */
  const GLenum COPY_WRITE_BUFFER_BINDING                     = 0x8F37; /* Same as COPY_WRITE_BUFFER */
  const GLenum UNIFORM_BUFFER                                = 0x8A11;
  const GLenum UNIFORM_BUFFER_BINDING                        = 0x8A28;
  const GLenum UNIFORM_BUFFER_START                          = 0x8A29;
  const GLenum UNIFORM_BUFFER_SIZE                           = 0x8A2A;
  const GLenum MAX_VERTEX_UNIFORM_BLOCKS                     = 0x8A2B;
  const GLenum MAX_FRAGMENT_UNIFORM_BLOCKS                   = 0x8A2D;
  const GLenum MAX_COMBINED_UNIFORM_BLOCKS                   = 0x8A2E;
  const GLenum MAX_UNIFORM_BUFFER_BINDINGS                   = 0x8A2F;
  const GLenum MAX_UNIFORM_BLOCK_SIZE                        = 0x8A30;
  const GLenum MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS        = 0x8A31;
  const GLenum MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS      = 0x8A33;
  const GLenum UNIFORM_BUFFER_OFFSET_ALIGNMENT               = 0x8A34;
  const GLenum ACTIVE_UNIFORM_BLOCKS                         = 0x8A36;
  const GLenum UNIFORM_TYPE                                  = 0x8A37;
  const GLenum UNIFORM_SIZE                                  = 0x8A38;
  const GLenum UNIFORM_BLOCK_INDEX                           = 0x8A3A;
  const GLenum UNIFORM_OFFSET                                = 0x8A3B;
  const GLenum UNIFORM_ARRAY_STRIDE                          = 0x8A3C;
  const GLenum UNIFORM_MATRIX_STRIDE                         = 0x8A3D;
  const GLenum UNIFORM_IS_ROW_MAJOR                          = 0x8A3E;
  const GLenum UNIFORM_BLOCK_BINDING                         = 0x8A3F;
  const GLenum UNIFORM_BLOCK_DATA_SIZE                       = 0x8A40;
  const GLenum UNIFORM_BLOCK_ACTIVE_UNIFORMS                 = 0x8A42;
  const GLenum UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES          = 0x8A43;
  const GLenum UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER     = 0x8A44;
  const GLenum UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER   = 0x8A46;
  const GLenum INVALID_INDEX                                 = 0xFFFFFFFF;
  const GLenum MAX_VERTEX_OUTPUT_COMPONENTS                  = 0x9122;
  const GLenum MAX_FRAGMENT_INPUT_COMPONENTS                 = 0x9125;
  const GLenum MAX_SERVER_WAIT_TIMEOUT                       = 0x9111;
  const GLenum OBJECT_TYPE                                   = 0x9112;
  const GLenum SYNC_CONDITION                                = 0x9113;
  const GLenum SYNC_STATUS                                   = 0x9114;
  const GLenum SYNC_FLAGS                                    = 0x9115;
  const GLenum SYNC_FENCE                                    = 0x9116;
  const GLenum SYNC_GPU_COMMANDS_COMPLETE                    = 0x9117;
  const GLenum UNSIGNALED                                    = 0x9118;
  const GLenum SIGNALED                                      = 0x9119;
  const GLenum ALREADY_SIGNALED                              = 0x911A;
  const GLenum TIMEOUT_EXPIRED                               = 0x911B;
  const GLenum CONDITION_SATISFIED                           = 0x911C;
  const GLenum WAIT_FAILED                                   = 0x911D;
  const GLenum SYNC_FLUSH_COMMANDS_BIT                       = 0x00000001;
  const GLenum VERTEX_ATTRIB_ARRAY_DIVISOR                   = 0x88FE;
  const GLenum ANY_SAMPLES_PASSED                            = 0x8C2F;
  const GLenum ANY_SAMPLES_PASSED_CONSERVATIVE               = 0x8D6A;
  const GLenum SAMPLER_BINDING                               = 0x8919;
  const GLenum RGB10_A2UI                                    = 0x906F;
  const GLenum INT_2_10_10_10_REV                            = 0x8D9F;
  const GLenum TRANSFORM_FEEDBACK                            = 0x8E22;
  const GLenum TRANSFORM_FEEDBACK_PAUSED                     = 0x8E23;
  const GLenum TRANSFORM_FEEDBACK_ACTIVE                     = 0x8E24;
  const GLenum TRANSFORM_FEEDBACK_BINDING                    = 0x8E25;
  const GLenum TEXTURE_IMMUTABLE_FORMAT                      = 0x912F;
  const GLenum MAX_ELEMENT_INDEX                             = 0x8D6B;
  const GLenum TEXTURE_IMMUTABLE_LEVELS                      = 0x82DF;

  const GLint64 TIMEOUT_IGNORED                              = -1;

  /* WebGL-specific enums */
  const GLenum MAX_CLIENT_WAIT_TIMEOUT_WEBGL                 = 0x9247;

  /* Buffer objects */
  undefined copyBufferSubData(GLenum readTarget, GLenum writeTarget, GLintptr readOffset,
                         GLintptr writeOffset, GLsizeiptr size);
  // MapBufferRange, in particular its read-only and write-only modes,
  // can not be exposed safely to JavaScript. GetBufferSubData
  // replaces it for the purpose of fetching data back from the GPU.
  undefined getBufferSubData(GLenum target, GLintptr srcByteOffset, /*[AllowShared]*/ ArrayBufferView dstBuffer,
                        optional GLuint dstOffset = 0, optional GLuint length = 0);

  /* Framebuffer objects */
  undefined blitFramebuffer(GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0,
                       GLint dstX1, GLint dstY1, GLbitfield mask, GLenum filter);
  undefined framebufferTextureLayer(GLenum target, GLenum attachment, WebGLTexture? texture, GLint level,
                               GLint layer);
  undefined invalidateFramebuffer(GLenum target, sequence<GLenum> attachments);
  undefined invalidateSubFramebuffer(GLenum target, sequence<GLenum> attachments,
                                GLint x, GLint y, GLsizei width, GLsizei height);
  undefined readBuffer(GLenum src);

  /* Renderbuffer objects */
  any getInternalformatParameter(GLenum target, GLenum internalformat, GLenum pname);
  undefined renderbufferStorageMultisample(GLenum target, GLsizei samples, GLenum internalformat,
                                      GLsizei width, GLsizei height);

  /* Texture objects */
  undefined texStorage2D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width,
                    GLsizei height);
  undefined texStorage3D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width,
                    GLsizei height, GLsizei depth);

  //[Throws]
  //void texImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
  //                GLsizei depth, GLint border, GLenum format, GLenum type, GLintptr pboOffset);
  //[Throws]
  //void texImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
  //                GLsizei depth, GLint border, GLenum format, GLenum type,
  //                TexImageSource source); // May throw DOMException
  [Throws]
  undefined texImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
                  GLsizei depth, GLint border, GLenum format, GLenum type, /*[AllowShared]*/ ArrayBufferView? srcData);
  //[Throws]
  //void texImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
  //                GLsizei depth, GLint border, GLenum format, GLenum type, [AllowShared] ArrayBufferView srcData,
  //                GLuint srcOffset);

  //[Throws]
  //void texSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset,
  //                   GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type,
  //                   GLintptr pboOffset);
  //[Throws]
  //void texSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset,
  //                   GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type,
  //                   TexImageSource source); // May throw DOMException
  //[Throws]
  //void texSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset,
  //                   GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type,
  //                   [AllowShared] ArrayBufferView? srcData, optional GLuint srcOffset = 0);

  //void copyTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset,
  //                       GLint x, GLint y, GLsizei width, GLsizei height);

  //void compressedTexImage3D(GLenum target, GLint level, GLenum internalformat, GLsizei width,
  //                          GLsizei height, GLsizei depth, GLint border, GLsizei imageSize, GLintptr offset);
  //void compressedTexImage3D(GLenum target, GLint level, GLenum internalformat, GLsizei width,
  //                          GLsizei height, GLsizei depth, GLint border, [AllowShared] ArrayBufferView srcData,
  //                          optional GLuint srcOffset = 0, optional GLuint srcLengthOverride = 0);

  //void compressedTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset,
  //                             GLint zoffset, GLsizei width, GLsizei height, GLsizei depth,
  //                             GLenum format, GLsizei imageSize, GLintptr offset);
  //void compressedTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset,
  //                             GLint zoffset, GLsizei width, GLsizei height, GLsizei depth,
  //                             GLenum format, [AllowShared] ArrayBufferView srcData,
  //                             optional GLuint srcOffset = 0,
  //                             optional GLuint srcLengthOverride = 0);

  /* Programs and shaders */
  [WebGLHandlesContextLoss] GLint getFragDataLocation(WebGLProgram program, DOMString name);

  /* Uniforms */
  undefined uniform1ui(WebGLUniformLocation? location, GLuint v0);
  undefined uniform2ui(WebGLUniformLocation? location, GLuint v0, GLuint v1);
  undefined uniform3ui(WebGLUniformLocation? location, GLuint v0, GLuint v1, GLuint v2);
  undefined uniform4ui(WebGLUniformLocation? location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);

  undefined uniform1uiv(WebGLUniformLocation? location, Uint32List data, optional GLuint srcOffset = 0,
                   optional GLuint srcLength = 0);
  undefined uniform2uiv(WebGLUniformLocation? location, Uint32List data, optional GLuint srcOffset = 0,
                   optional GLuint srcLength = 0);
  undefined uniform3uiv(WebGLUniformLocation? location, Uint32List data, optional GLuint srcOffset = 0,
                   optional GLuint srcLength = 0);
  undefined uniform4uiv(WebGLUniformLocation? location, Uint32List data, optional GLuint srcOffset = 0,
                   optional GLuint srcLength = 0);

  undefined uniformMatrix3x2fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                          optional GLuint srcOffset = 0, optional GLuint srcLength = 0);
  undefined uniformMatrix4x2fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                          optional GLuint srcOffset = 0, optional GLuint srcLength = 0);

  undefined uniformMatrix2x3fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                          optional GLuint srcOffset = 0, optional GLuint srcLength = 0);
  undefined uniformMatrix4x3fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                          optional GLuint srcOffset = 0, optional GLuint srcLength = 0);

  undefined uniformMatrix2x4fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                          optional GLuint srcOffset = 0, optional GLuint srcLength = 0);
  undefined uniformMatrix3x4fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                          optional GLuint srcOffset = 0, optional GLuint srcLength = 0);

  /* Vertex attribs */
  undefined vertexAttribI4i(GLuint index, GLint x, GLint y, GLint z, GLint w);
  undefined vertexAttribI4iv(GLuint index, Int32List values);
  undefined vertexAttribI4ui(GLuint index, GLuint x, GLuint y, GLuint z, GLuint w);
  undefined vertexAttribI4uiv(GLuint index, Uint32List values);
  undefined vertexAttribIPointer(GLuint index, GLint size, GLenum type, GLsizei stride, GLintptr offset);

  /* Writing to the drawing buffer */
  undefined vertexAttribDivisor(GLuint index, GLuint divisor);
  undefined drawArraysInstanced(GLenum mode, GLint first, GLsizei count, GLsizei instanceCount);
  undefined drawElementsInstanced(GLenum mode, GLsizei count, GLenum type, GLintptr offset, GLsizei instanceCount);
  undefined drawRangeElements(GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, GLintptr offset);

  /* Multiple Render Targets */
  undefined drawBuffers(sequence<GLenum> buffers);

  undefined clearBufferfv(GLenum buffer, GLint drawbuffer, Float32List values,
                     optional GLuint srcOffset = 0);
  undefined clearBufferiv(GLenum buffer, GLint drawbuffer, Int32List values,
                     optional GLuint srcOffset = 0);
  undefined clearBufferuiv(GLenum buffer, GLint drawbuffer, Uint32List values,
                      optional GLuint srcOffset = 0);

  undefined clearBufferfi(GLenum buffer, GLint drawbuffer, GLfloat depth, GLint stencil);

  /* Query Objects */
  WebGLQuery? createQuery();
  undefined deleteQuery(WebGLQuery? query);
  /*[WebGLHandlesContextLoss]*/ GLboolean isQuery(WebGLQuery? query);
  undefined beginQuery(GLenum target, WebGLQuery query);
  undefined endQuery(GLenum target);
  WebGLQuery? getQuery(GLenum target, GLenum pname);
  any getQueryParameter(WebGLQuery query, GLenum pname);

  /* Sampler Objects */
  WebGLSampler? createSampler();
  undefined deleteSampler(WebGLSampler? sampler);
  [WebGLHandlesContextLoss] GLboolean isSampler(WebGLSampler? sampler);
  undefined bindSampler(GLuint unit, WebGLSampler? sampler);
  undefined samplerParameteri(WebGLSampler sampler, GLenum pname, GLint param);
  undefined samplerParameterf(WebGLSampler sampler, GLenum pname, GLfloat param);
  any getSamplerParameter(WebGLSampler sampler, GLenum pname);

  /* Sync objects */
  WebGLSync? fenceSync(GLenum condition, GLbitfield flags);
  [WebGLHandlesContextLoss] GLboolean isSync(WebGLSync? sync);
  undefined deleteSync(WebGLSync? sync);
  GLenum clientWaitSync(WebGLSync sync, GLbitfield flags, GLuint64 timeout);
  undefined waitSync(WebGLSync sync, GLbitfield flags, GLint64 timeout);
  any getSyncParameter(WebGLSync sync, GLenum pname);

  /* Transform Feedback */
  WebGLTransformFeedback? createTransformFeedback();
  undefined deleteTransformFeedback(WebGLTransformFeedback? tf);
  [WebGLHandlesContextLoss] GLboolean isTransformFeedback(WebGLTransformFeedback? tf);
  undefined bindTransformFeedback (GLenum target, WebGLTransformFeedback? tf);
  undefined beginTransformFeedback(GLenum primitiveMode);
  undefined endTransformFeedback();
  undefined transformFeedbackVaryings(WebGLProgram program, sequence<DOMString> varyings, GLenum bufferMode);
  WebGLActiveInfo? getTransformFeedbackVarying(WebGLProgram program, GLuint index);
  undefined pauseTransformFeedback();
  undefined resumeTransformFeedback();

  /* Uniform Buffer Objects and Transform Feedback Buffers */
  undefined bindBufferBase(GLenum target, GLuint index, WebGLBuffer? buffer);
  undefined bindBufferRange(GLenum target, GLuint index, WebGLBuffer? buffer, GLintptr offset, GLsizeiptr size);
  any getIndexedParameter(GLenum target, GLuint index);
  sequence<GLuint>? getUniformIndices(WebGLProgram program, sequence<DOMString> uniformNames);
  any getActiveUniforms(WebGLProgram program, sequence<GLuint> uniformIndices, GLenum pname);
  GLuint getUniformBlockIndex(WebGLProgram program, DOMString uniformBlockName);
  any getActiveUniformBlockParameter(WebGLProgram program, GLuint uniformBlockIndex, GLenum pname);
  DOMString? getActiveUniformBlockName(WebGLProgram program, GLuint uniformBlockIndex);
  undefined uniformBlockBinding(WebGLProgram program, GLuint uniformBlockIndex, GLuint uniformBlockBinding);

  /* Vertex Array Objects */
  WebGLVertexArrayObject? createVertexArray();
  undefined deleteVertexArray(WebGLVertexArrayObject? vertexArray);
  [WebGLHandlesContextLoss] GLboolean isVertexArray(WebGLVertexArrayObject? vertexArray);
  undefined bindVertexArray(WebGLVertexArrayObject? array);
};

interface mixin WebGL2RenderingContextOverloads
{
  // WebGL1:
  undefined bufferData(GLenum target, GLsizeiptr size, GLenum usage);
  undefined bufferData(GLenum target, /*[AllowShared]*/ BufferSource? srcData, GLenum usage);
  undefined bufferSubData(GLenum target, GLintptr dstByteOffset, /*[AllowShared]*/ BufferSource srcData);
  // WebGL2:
  undefined bufferData(GLenum target, /*[AllowShared]*/ ArrayBufferView srcData, GLenum usage, GLuint srcOffset,
                  optional GLuint length = 0);
  undefined bufferSubData(GLenum target, GLintptr dstByteOffset, /*[AllowShared]*/ ArrayBufferView srcData,
                     GLuint srcOffset, optional GLuint length = 0);

  // WebGL1 legacy entrypoints:
  [Throws]
  undefined texImage2D(GLenum target, GLint level, GLint internalformat,
                  GLsizei width, GLsizei height, GLint border, GLenum format,
                  GLenum type, /*[AllowShared]*/ ArrayBufferView? pixels);
  [Throws]
  undefined texImage2D(GLenum target, GLint level, GLint internalformat,
                  GLenum format, GLenum type, TexImageSource source); // May throw DOMException

  [Throws]
  undefined texSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset,
                     GLsizei width, GLsizei height,
                     GLenum format, GLenum type, /*[AllowShared]*/ ArrayBufferView? pixels);
  [Throws]
  undefined texSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset,
                     GLenum format, GLenum type, TexImageSource source); // May throw DOMException

  // WebGL2 entrypoints:
  [Throws]
  undefined texImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
                  GLint border, GLenum format, GLenum type, GLintptr pboOffset);
  [Throws]
  undefined texImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
                  GLint border, GLenum format, GLenum type,
                  TexImageSource source); // May throw DOMException
  [Throws]
  undefined texImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height,
                  GLint border, GLenum format, GLenum type, /*[AllowShared]*/ ArrayBufferView srcData,
                  GLuint srcOffset);

  //[Throws]
  //void texSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width,
  //                   GLsizei height, GLenum format, GLenum type, GLintptr pboOffset);
  //[Throws]
  //void texSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width,
  //                   GLsizei height, GLenum format, GLenum type,
  //                   TexImageSource source); // May throw DOMException
  //[Throws]
  //void texSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width,
  //                   GLsizei height, GLenum format, GLenum type, /*[AllowShared]*/ ArrayBufferView srcData,
  //                   GLuint srcOffset);

  //void compressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width,
  //                          GLsizei height, GLint border, GLsizei imageSize, GLintptr offset);
  undefined compressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width,
                            GLsizei height, GLint border, /*[AllowShared]*/ ArrayBufferView srcData,
                            optional GLuint srcOffset = 0, optional GLuint srcLengthOverride = 0);

  //void compressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset,
  //                             GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, GLintptr offset);
  undefined compressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset,
                               GLsizei width, GLsizei height, GLenum format,
                               /*[AllowShared]*/ ArrayBufferView srcData,
                               optional GLuint srcOffset = 0,
                               optional GLuint srcLengthOverride = 0);

  undefined uniform1fv(WebGLUniformLocation? location, Float32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);
  undefined uniform2fv(WebGLUniformLocation? location, Float32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);
  undefined uniform3fv(WebGLUniformLocation? location, Float32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);
  undefined uniform4fv(WebGLUniformLocation? location, Float32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);

  undefined uniform1iv(WebGLUniformLocation? location, Int32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);
  undefined uniform2iv(WebGLUniformLocation? location, Int32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);
  undefined uniform3iv(WebGLUniformLocation? location, Int32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);
  undefined uniform4iv(WebGLUniformLocation? location, Int32List data, optional GLuint srcOffset = 0,
                  optional GLuint srcLength = 0);

  undefined uniformMatrix2fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                        optional GLuint srcOffset = 0, optional GLuint srcLength = 0);
  undefined uniformMatrix3fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                        optional GLuint srcOffset = 0, optional GLuint srcLength = 0);
  undefined uniformMatrix4fv(WebGLUniformLocation? location, GLboolean transpose, Float32List data,
                        optional GLuint srcOffset = 0, optional GLuint srcLength = 0);

  /* Reading back pixels */
  // WebGL1:
  undefined readPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type,
                  /*[AllowShared]*/ ArrayBufferView? dstData);
  // WebGL2:
  undefined readPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type,
                  GLintptr offset);
  undefined readPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type,
                  /*[AllowShared]*/ ArrayBufferView dstData, GLuint dstOffset);
};

[Exposed=Window, Func="WebGL2RenderingContext::is_webgl2_enabled"]
interface WebGL2RenderingContext
{
};
WebGL2RenderingContext includes WebGLRenderingContextBase;
WebGL2RenderingContext includes WebGL2RenderingContextBase;
WebGL2RenderingContext includes WebGL2RenderingContextOverloads;
