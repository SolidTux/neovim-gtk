mod item;
mod analysis;

use std::ptr;
use std::ffi::CStr;

use pango;
use pango_sys;
use cairo;
use pangocairo_sys;

use glib::translate::*;

pub fn pango_itemize(
    context: &pango::Context,
    text: &CStr,
    start_index: usize,
    length: usize,
    attrs: &pango::AttrList,
) -> Vec<item::Item> {
    unsafe {
        FromGlibPtrContainer::from_glib_container(pango_sys::pango_itemize(
            context.to_glib_none().0,
            text.as_ptr(),
            start_index as i32,
            length as i32,
            attrs.to_glib_none().0,
            ptr::null_mut(),
        ))
    }
}

pub fn pango_shape(
    text: &CStr,
    length: usize,
    analysis: &analysis::Analysis,
    glyphs: &mut pango::GlyphString,
) {
    unsafe {
        pango_sys::pango_shape(
            text.as_ptr(),
            length as i32,
            analysis.to_glib_ptr(),
            glyphs.to_glib_none_mut().0,
        );
    }
}

pub fn pango_cairo_show_glyph_string(ctx: &cairo::Context, font: &pango::Font, glyphs: &pango::GlyphString) {
    pangocairo_sys::pango_cairo_show_glyph_string();
}

