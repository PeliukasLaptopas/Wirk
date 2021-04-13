mod ui_utils {
    use sdl2::pixels::Color;
    use std::ffi::c_void;
    use crate::rendering::texture::Texture;

    //todo use this
    /*pub unsafe fn generate_bitmap_from_string(text: String) -> *const c_void {
        unsafe {
            let ttf_context = sdl2::ttf::init().map_err(|e| format_err!("{:?}", e))?;
            // Load a font
            let mut font = ttf_context.load_font(resources.root_path.join("font.otf"), 128).map_err(|e| format_err!("{:?}", e))?;

            // render a surface, and convert it to a texture bound to the canvas
            let surface = font
                .render(text.as_str())
                .blended(Color::RGBA(35, 121, 100, 255))
                .map_err(|e| format_err!("{:?}", e))?;

            (*surface.raw()).pixels as *const std::os::raw::c_void
        }
    }*/
}