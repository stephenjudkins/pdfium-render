#[cfg(target_arch = "wasm32")]
use pdfium_render::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::ImageData;

// See https://github.com/ajrcarey/pdfium-render/tree/master/examples for information
// on how to build and package this example alongside a WASM build of Pdfium, suitable
// for running in a browser.

/// Downloads the given url, opens it as a PDF document, then Logs the width and height of
/// each page in the document, along with other document metrics, to the Javascript console.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn log_page_metrics_to_console(url: String) {
    // Our only option when targeting WASM is to bind to the "system library"
    // (a separate WASM build of Pdfium).

    let bindings = Pdfium::bind_to_system_library().unwrap();

    let pdfium = Pdfium::new(bindings);

    let document = pdfium.load_pdf_from_fetch(url, None).await.unwrap();

    // Output metadata and form information for the PDF file to the console.

    log::info!("PDF file version: {:#?}", document.version());

    log::info!("PDF metadata tags:");
    document
        .metadata()
        .iter()
        .enumerate()
        .for_each(|(index, tag)| log::info!("{}: {:#?} = {}", index, tag.tag_type(), tag.value()));

    match document.form() {
        Some(form) => log::info!(
            "PDF contains an embedded form of type {:#?}",
            form.form_type()
        ),
        None => log::info!("PDF does not contain an embedded form"),
    };

    // Report labels, boundaries, and metrics for each page to the console.

    document
        .pages()
        .iter()
        .enumerate()
        .for_each(|(page_index, page)| {
            if let Some(label) = page.label() {
                log::info!("Page {} has a label: {}", page_index, label);
            }

            log::info!(
                "Page {} width: {}, height: {}",
                page_index,
                page.width().value,
                page.height().value
            );

            for boundary in page.boundaries().iter() {
                log::info!(
                    "Page {} has defined {:#?} box ({}, {}) - ({}, {})",
                    page_index,
                    boundary.box_type,
                    boundary.bounds.left.value,
                    boundary.bounds.top.value,
                    boundary.bounds.right.value,
                    boundary.bounds.bottom.value,
                );
            }

            log::info!(
                "Page {} has paper size {:#?}",
                page_index,
                page.paper_size()
            );

            for (link_index, link) in page.links().iter().enumerate() {
                log::info!(
                    "Page {} link {} has action of type {:?}",
                    page_index,
                    link_index,
                    link.action().map(|action| action.action_type())
                );

                // For links that have URI actions, output the destination URI.

                if let Some(action) = link.action() {
                    if let Some(uri_action) = action.as_uri_action() {
                        log::info!("Link URI destination: {:#?}", uri_action.uri())
                    }
                }
            }
        });
}

/// Downloads the given url, opens it as a PDF document, then returns the ImageData for
/// the given page index using the given bitmap dimensions.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn get_image_data_for_page(
    url: String,
    index: PdfPageIndex,
    width: u16,
    height: u16,
) -> ImageData {
    Pdfium::new(Pdfium::bind_to_system_library().unwrap())
        .load_pdf_from_fetch(url, None)
        .await
        .unwrap()
        .pages()
        .get(index)
        .unwrap()
        .render_with_config(
            &PdfRenderConfig::new()
                .set_target_size(width, height)
                .render_form_data(true)
                .highlight_text_form_fields(PdfColor::SOLID_YELLOW.with_alpha(128))
                .highlight_checkbox_form_fields(PdfColor::SOLID_BLUE.with_alpha(128)),
        )
        .unwrap()
        .as_image_data()
        .unwrap()
}

// Source files in examples/ directory are expected to always have a main() entry-point.
// Since we're compiling to WASM, we'll never actually use this.
#[allow(dead_code)]
fn main() {}
