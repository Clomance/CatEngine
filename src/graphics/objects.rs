use crate::Colour;

use glium::{
    implement_vertex,
    index::{
        PrimitiveType,
        IndicesSource,
        IndexType,
        Index
    },
    vertex::{
        VerticesSource,
        VertexFormat,
    },
    buffer::{
        Buffer,
        Content,
    },
};

/// Типаж для создания объектов, которые зависят от буферов вершин и индексов.
/// A trait for creating objects depend on vertex and index buffers.
/// 
/// The crate's graphics engine uses `TexturedVertex2D` and `Vertex2D` for vertices
/// and `u8` for indices, but you can add your own
/// and draw objects with `window.draw()`.
pub trait DependentObject<'o,V:Copy,I:Copy>
    where
        [V]:Content,
        [I]:Content,
{
    type Vertices:AsRef<[V]>+'o;
    type Indices:AsRef<[I]>+'o;

    /// Цвет объекта.
    /// 
    /// Object's colour.
    fn colour(&self)->Colour;

    /// Вершины объекта.
    /// 
    /// Object's vertices.
    /// 
    /// The crate's graphics uses the window coordinate system.
    fn vertices(&'o self)->Self::Vertices;

    /// Индексы для построения объекта.
    /// 
    /// Indices to build the object.
    fn indices(&'o self)->Option<Self::Indices>;

    fn primitive_type(&self)->PrimitiveType;

    /// Вписывает индексы в буфер индексов и возвращает `Some(IndicesSource)` для рисования
    /// или `None`, если недостаточно места.
    /// 
    /// Writes indices to the index buffer and return `Some(IndicesSource)` to draw
    /// or `None` if there is not enough space.
    fn write_indices<'a>(
        &'o self,
        index_buffer:&'a Buffer<[I]>
    )->Option<IndicesSource<'a>>{
        Some(
            if let Some(indicesb)=self.indices(){
                let indices=indicesb.as_ref();

                let slice=match index_buffer.slice(0..indices.len()){
                    Some(slice)=>slice,
                    None=>return None,
                };
                slice.write(&indices);

                IndicesSource::IndexBuffer{
                    buffer:slice.as_slice_any(),
                    data_type:IndexType::U8,
                    primitives:self.primitive_type(),
                }
            }
            else{
                IndicesSource::NoIndices{
                    primitives:self.primitive_type(),
                }
            }
        )
    }

    /// Вписывает вершины в буфер индексов и возвращает `Some(IndicesSource)` для рисования
    /// или `None`, если недостаточно места.
    /// 
    /// Writes indices to the index buffer and return `Some(IndicesSource)` to draw
    /// or `None` if there is not enough space.
    fn write_vertices<'a>(
        &'o self,
        vertex_buffer:&'a Buffer<[V]>,
        vertex_format:&'a VertexFormat,
    )->Option<VerticesSource<'a>>{
        let verticesb=self.vertices();
        let vertices:&[V]=verticesb.as_ref();

        let slice=match vertex_buffer.slice(0..vertices.len()){
            Some(slice)=>slice,
            None=>return None,
        };

        slice.write(&vertices);

        Some(
            VerticesSource::VertexBuffer(
                slice.as_slice_any(),
                vertex_format,
                false
            )
        )
    }
}