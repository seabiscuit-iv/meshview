// pub mod Shader {
    use eframe::glow::{self, HasContext};
    use egui::Vec2;
    use nalgebra::{Vector2, Vector3, Vector4};
    use rand;

    use crate::camera::Camera;

    
    pub struct ShaderProgram {
        pub program : glow::Program,
        vert_shader: glow::Shader,
        frag_shader: glow::Shader
    }


    impl ShaderProgram {
        pub fn new(gl: &glow::Context, vs_path: &str, fs_path: &str) -> Self {
            use glow::HasContext as _;

            unsafe {
                let program = gl.create_program().expect("Cannot create program");

                let (vertex_shader_source, fragment_shader_source) = (
                    std::fs::read_to_string(vs_path).unwrap(),
                    std::fs::read_to_string(fs_path).unwrap(),
                );

                let shader_sources = [
                    (glow::VERTEX_SHADER, vertex_shader_source),
                    (glow::FRAGMENT_SHADER, fragment_shader_source),
                ];

                let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();


                // assert status of the shader
                gl.link_program(program);
                assert!(
                    gl.get_program_link_status(program),
                    "{}",
                    gl.get_program_info_log(program)
                );

                for shader in shaders.iter() {
                    gl.detach_shader(program, *shader);
                    gl.delete_shader(*shader);
                }

                Self {
                    program,
                    vert_shader: shaders[0],
                    frag_shader: shaders[1]
                }
            }
        }


        pub fn destroy(&self, gl: &glow::Context) {
            use glow::HasContext as _;
            unsafe {
                gl.delete_program(self.program);
            }
        }

        pub fn paint(&self, gl: &glow::Context, mesh: &Mesh, camera: &Camera) {
            use glow::HasContext as _;

            unsafe {
                
                gl.clear(glow::DEPTH_BUFFER_BIT);
                gl.depth_func(glow::LESS);
                gl.enable(glow::DEPTH_TEST);

                gl.use_program(Some(self.program));

                gl.uniform_matrix_4_f32_slice(
                    gl.get_uniform_location(self.program, "u_ViewProj").as_ref(),
                    false, 
                    camera.get_proj_view_mat().as_slice()
                );

                gl.bind_vertex_array(Some(mesh.vertex_array));
                gl.draw_elements(glow::TRIANGLES, mesh.index_buffer_size as i32, glow::UNSIGNED_INT, 0);
            }
        }
    }

    


    
    #[derive(Debug)]
    pub struct Mesh {
        pub positions: Vec<Vector3<f32>>,
        pub indicies : Vec<u32>,
        uvs: Vec<Vector2<f32>>,
        colors: Vec<Vector4<f32>>,
        vertex_array: glow::VertexArray,
        position_buffer: glow::Buffer,
        color_buffer: glow::Buffer,
        index_buffer: glow::Buffer,
        uv_buffer: glow::Buffer,
        index_buffer_size: u32
    }


    impl Mesh {
        pub fn new(gl: &glow::Context, positions: Vec<Vector3<f32>>, indicies: Vec<u32>) -> Self {
            use glow::HasContext as _;

            unsafe {
                let vert_count = positions.len();

                let mut uvs: Vec<Vector2<f32>> = Vec::new();
                
                for _ in 0..vert_count {
                    let x : Vector2<f32> = [0.0, 0.0].into();
                    uvs.push(x);
                }
                
                let mut colors: Vec<Vector4<f32>> = Vec::new();

                for (i, pos) in positions.iter().enumerate() {
                    let i = i as f32;

                    if i as i32 % 3 == 0 {
                        let col = Vector3::new(
                            rand::random::<f32>().fract(),
                            rand::random::<f32>().fract(),
                            rand::random::<f32>().fract()
                        );
                        let col = col.push(1.0);
                        colors.push(col);
                    } else {
                        colors.push(colors[i as usize - 1]);
                    }
                }

                let position_buffer: glow::NativeBuffer = gl.create_buffer().expect("Cannot create position buffer");
                let color_buffer = gl.create_buffer().expect("Cannot create color buffer");
                let uv_buffer = gl.create_buffer().expect("Cannot create uv buffer");
                let index_buffer = gl.create_buffer().expect("Cannot create index buffer");

                
                let vertex_array = gl.create_vertex_array().expect("Cannot create vertex array");
                gl.bind_vertex_array(Some(vertex_array));
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
                gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indicies), glow::STATIC_DRAW);

                gl.bind_buffer(glow::ARRAY_BUFFER, Some(position_buffer));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&positions.iter().flat_map(|x| {
                    vec![x.x, x.y, x.z, 1.0].into_iter()
                }).collect::<Vec<f32>>()), glow::STATIC_DRAW);
                gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, 0, 0);  // Position (2 floats per vertex)
                gl.enable_vertex_attrib_array(0);  // Enable position attribute

                gl.bind_buffer(glow::ARRAY_BUFFER, Some(color_buffer));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&colors.iter().flat_map(|x| {
                    vec![x.x, x.y, x.z, x.w].into_iter()
                }).collect::<Vec<f32>>()), glow::STATIC_DRAW);
                gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0, 0);  // Color (4 floats per vertex)
                gl.enable_vertex_attrib_array(1);  // Enable color attribute

                gl.bind_buffer(glow::ARRAY_BUFFER, Some(uv_buffer));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&uvs.iter().flat_map(|x|{
                    vec![x.x, x.y].into_iter()
                }).collect::<Vec<f32>>()), glow::STATIC_DRAW);
                gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(2);  // Enable uv attribute

                Self {
                    positions: positions.clone(), 
                    indicies: indicies.clone(),
                    uvs: uvs.clone(),
                    colors: colors.clone(),
                    vertex_array,
                    position_buffer,
                    color_buffer,
                    index_buffer,
                    uv_buffer,
                    index_buffer_size: indicies.len() as u32
                }
            }
        }

        pub fn destroy(&self, gl: &glow::Context) {
            use glow::HasContext as _;
            unsafe {
                gl.delete_buffer(self.position_buffer);
                gl.delete_vertex_array(self.vertex_array);
                gl.delete_buffer(self.color_buffer);
                gl.delete_buffer(self.index_buffer);
                gl.delete_buffer(self.uv_buffer);
            }
        }
    }


// }