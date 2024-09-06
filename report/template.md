---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise 1 
author:
- Ivan Zubčić 
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "Table of Contents"
toc-depth: 2
numbersections: true
header-includes:
# The `atkinson` font, requires 'texlive-fontsextra' on arch or the 'atkinson' CTAN package
# Uncomment this line to enable:
#- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

`\clearpage`{=latex}

# Tasks	 

## Task 1: Drwaing your first triangle 

While creating the first triangle we need to use the function vao. This function creates Vertex Array Object (VAO) and binds it.
After creating VAO we create Vertex Buffer Object (VBO) which holds geometry data of the object. VBO like VAO is also binded. 
Basically VAO is object which takes data parameters (In this case float vector for vertex position and integer data for indices) for drawing the data on screen using OpenGL. 

This would be create_vao function we are using:

```rust
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>) -> u32 {

    // This should:
    // * Generate a VAO and bind it
    // * Generate a VBO and bind it
    // * Fill it with data
    // * Configure a VAP for the data and enable it
    // * Generate a IBO and bind it
    // * Fill it with data
    // * Return the ID of the VAO

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut indices_val: u32 = 0;

    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(vertices),  pointer_to_array(vertices), gl::STATIC_DRAW);

    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3*size_of::<f32>(),
        ptr::null()
    );

    gl::EnableVertexAttribArray(0);

    gl::GenBuffers(1, &mut indices_val);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_val);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array(indices),  pointer_to_array(indices), gl::STATIC_DRAW);

    gl::BindVertexArray(0);
    vao
}
```

Indices and creation of VAO with given vector values:

```rust
let my_vert: Vec<f32> = vec![
            0.1, 0.0, 0.0,
            0.1, 0.1, 0.0,
            0.0, 0.0, 0.0,

            0.3, 0.0, 0.0,
            0.3, 0.1, 0.0,
            0.2, 0.0, 0.0,

            0.5, 0.0, 0.0,
            0.5, 0.1, 0.0,
            0.4, 0.0, 0.0,

            0.1, 0.2, 0.0,
            0.1, 0.3, 0.0,
            0.0, 0.2, 0.0,

            0.3, 0.2, 0.0,
            0.3, 0.3, 0.0,
            0.2, 0.2, 0.0
        ];


        let my_indi: Vec<u32> = vec![
            0, 1, 2,
            3, 4, 5,
            6, 7, 8,
            9, 10, 11,
            12, 13, 14
        ];

        let my_vao = unsafe { create_vao(&my_vert, &my_indi)};
```


Before we draw the triangles we need to link shaders:

```rust
let simple_shader = unsafe {
                shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };
```

`\clearpage`{=latex}

And after clearing the scene we now can draw our triangles:

```rust
unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                // == // Issue the necessary gl:: commands to draw your scene here
                //
                simple_shader.activate();

                gl::BindVertexArray(my_vao);
                gl::DrawElements(gl::TRIANGLES, my_indi.len() as i32, gl::UNSIGNED_INT, ptr::null());
                gl::BindVertexArray(0);
            }
```

![Drawn Triangles](images/triangles.png)

### Subsubheading

This is a paragraph.
This is the same paragraph.

This is a new paragraph, with *italic*, **bold**, and `inline code` formatting.
It is possible to use special classes to format text: [this is a test]{.smallcaps}.

```rust
//this is a code block with rust syntax highlighting
println!("Hello, {}", 42);
```

[This](https://www.ntnu.no) is a link.
[This][] is also a link. <!-- defined below -->
This[^this_is_a_unique_footnote_label] is a footnote. <!-- defined below -->
This^[Footnotes can also be written inline] is also a footnote.


[This]: https://www.uio.no
[^this_is_a_unique_footnote_label]: In footnotes you can write anything tangentially related.

* This
* is
* a
* unordered
* list

1. This
1. is
1. a
1. ordered
1. list
    a. with
    a. sub
    a. list

       with multiple paragraphs

This is still on the first page

`\clearpage`{=latex}

<!--
Above is a raw LaTeX statement.
Those are included when exporting to LaTeX or PDF, and ignored when exporting to HTML.
-->

This is on the second page

i) Roman ordered list
i) Roman ordered list
i) Roman ordered list

This
: is a definition

> this is a
block quote


This is a paragraph with _inline_ \LaTeX\ style math: $\frac{1}{2}$.
Below is a math _block_:

$$
    \int_{a}^{b} f(x)dx
$$


| This | is  | a   | table |
| ---- | --- | --- | ----- |
| 1    | 2   | 3   | 4     |
| 5    | 6   | 7   | 8     |

: This is a table caption

This is an inline image with a fixed height:
![](images/logo.png){height=5em}

Below is a _figure_ (i.e. an image with a caption).
It floats and may as a result move to a different page depending on the layout.

![
    Image with caption
](images/logo.png)

Enable and use the `pandoc-crossref` filter to reference figures, tables and equations.
