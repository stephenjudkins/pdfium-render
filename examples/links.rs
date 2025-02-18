use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // For every page in our sample file...

    let document = pdfium.load_pdf_from_file("test/links-test.pdf", None)?;

    for (page_index, page) in document.pages().iter().enumerate() {
        // ... output information about every link on the page to the console.

        println!("=============== Page {} ===============", page_index);

        let mut links_count = 0;

        for (link_index, link) in page.links().iter().enumerate() {
            println!(
                "Page {} link {} has action of type {:?}",
                page_index,
                link_index,
                link.action().map(|action| action.action_type())
            );

            // For links that have URI actions, output the destination URI.

            if let Some(action) = link.action() {
                if let Some(uri_action) = action.as_uri_action() {
                    println!("Link URI destination: {:#?}", uri_action.uri())
                }
            }

            links_count += 1;
        }

        assert_eq!(links_count, page.links().len());
    }

    Ok(())
}
