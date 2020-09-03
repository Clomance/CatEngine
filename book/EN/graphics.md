# Introduction

The graphics engine is divided into three parts:
1. Simple graphics - plain objects composed of `Vertex2D`.
2. Textures (images)
3. Text

There are three way to render objects (text is included, too)
 - Commom - for common drawing an object, `DrawType::Common`
 - Shifting - for drawing a shifted object, `DrawType::Shifting([f32;2])`
 - Rotating - for drawing a rotated object, `DrawType::Rotating((f32,[f32;2]))`



# Simple graphics

Simple graphics is responsible for rendering plain (for now) objects.

A vertex - `Vertex2D` - a point in the window coordinate system.



# Texture graphics

Texture graphics is responсible for rendering textures.

A vertex - `TextureVertex2D` - a point in the window coordinate system with texture coordinates.



# Text graphics

Text graphics is responсible for rendering text.

Same as texture graphics.

How a sign is rendering:
1. A glyph is built with the `rusttype` crate (or is given the ready one)
2. The glyph is written to the array as an image
3. The image is loaded to the texture
4. The texture is rendered