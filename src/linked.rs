use crate::bindgen::{
    size_t, FPDFANNOT_COLORTYPE, FPDF_ACTION, FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE,
    FPDF_ANNOT_APPEARANCEMODE, FPDF_ATTACHMENT, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL,
    FPDF_CLIPPATH, FPDF_DEST, FPDF_DOCUMENT, FPDF_DUPLEXTYPE, FPDF_DWORD, FPDF_FILEACCESS,
    FPDF_FILEIDTYPE, FPDF_FILEWRITE, FPDF_FONT, FPDF_FORMFILLINFO, FPDF_FORMHANDLE, FPDF_GLYPHPATH,
    FPDF_IMAGEOBJ_METADATA, FPDF_LINK, FPDF_OBJECT_TYPE, FPDF_PAGE, FPDF_PAGELINK, FPDF_PAGEOBJECT,
    FPDF_PAGEOBJECTMARK, FPDF_PAGERANGE, FPDF_PATHSEGMENT, FPDF_SCHHANDLE, FPDF_SIGNATURE,
    FPDF_STRUCTELEMENT, FPDF_STRUCTTREE, FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR,
    FPDF_WIDESTRING, FS_FLOAT, FS_MATRIX, FS_POINTF, FS_QUADPOINTSF, FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

pub(crate) struct StaticPdfiumBindings;

impl StaticPdfiumBindings {
    #[inline]
    pub fn new() -> Self {
        StaticPdfiumBindings {}
    }
}

impl Default for StaticPdfiumBindings {
    #[inline]
    fn default() -> Self {
        StaticPdfiumBindings::new()
    }
}

impl PdfiumLibraryBindings for StaticPdfiumBindings {
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self) {
        unsafe {
            crate::bindgen::FPDF_InitLibrary();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self) {
        unsafe {
            crate::bindgen::FPDF_DestroyLibrary();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong {
        unsafe { crate::bindgen::FPDF_GetLastError() }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        unsafe { crate::bindgen::FPDF_CreateNewDocument() }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT {
        let c_file_path = CString::new(file_path).unwrap();
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe { crate::bindgen::FPDF_LoadDocument(c_file_path.as_ptr(), c_password.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument64(&self, data_buf: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            crate::bindgen::FPDF_LoadMemDocument64(
                data_buf.as_ptr() as *const c_void,
                data_buf.len() as size_t,
                c_password.as_ptr(),
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: *mut FPDF_FILEACCESS,
        password: Option<&str>,
    ) -> FPDF_DOCUMENT {
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe { crate::bindgen::FPDF_LoadCustomDocument(pFileAccess, c_password.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDF_SaveAsCopy(document, pFileWrite, flags) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SaveWithVersion(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: c_int,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDF_SaveWithVersion(document, pFileWrite, flags, fileVersion) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        unsafe {
            crate::bindgen::FPDF_CloseDocument(document);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDF_GetFileVersion(doc, fileVersion) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileIdentifier(
        &self,
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_GetFileIdentifier(document, id_type, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_GetFormType(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        let c_tag = CString::new(tag).unwrap();

        unsafe { crate::bindgen::FPDF_GetMetaText(document, c_tag.as_ptr(), buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> c_ulong {
        unsafe { crate::bindgen::FPDF_GetDocPermissions(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSecurityHandlerRevision(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_GetSecurityHandlerRevision(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_GetPageCount(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE {
        unsafe { crate::bindgen::FPDF_LoadPage(document, page_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        unsafe {
            crate::bindgen::FPDF_ClosePage(page);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportPagesByIndex(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDF_ImportPagesByIndex(dest_doc, src_doc, page_indices, length, index)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportPages(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: &str,
        index: c_int,
    ) -> FPDF_BOOL {
        let c_pagerange = CString::new(pagerange).unwrap();

        unsafe { crate::bindgen::FPDF_ImportPages(dest_doc, src_doc, c_pagerange.as_ptr(), index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportNPagesToOne(
        &self,
        src_doc: FPDF_DOCUMENT,
        output_width: c_float,
        output_height: c_float,
        num_pages_on_x_axis: size_t,
        num_pages_on_y_axis: size_t,
    ) -> FPDF_DOCUMENT {
        unsafe {
            crate::bindgen::FPDF_ImportNPagesToOne(
                src_doc,
                output_width,
                output_height,
                num_pages_on_x_axis,
                num_pages_on_y_axis,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> c_float {
        unsafe { crate::bindgen::FPDF_GetPageWidthF(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> c_float {
        unsafe { crate::bindgen::FPDF_GetPageHeightF(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_GetPageLabel(document, page_index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexFromTextIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nTextIndex: c_int,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFText_GetCharIndexFromTextIndex(text_page, nTextIndex) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextIndexFromCharIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nCharIndex: c_int,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFText_GetTextIndexFromCharIndex(text_page, nCharIndex) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSignatureCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_GetSignatureCount(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSignatureObject(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE {
        unsafe { crate::bindgen::FPDF_GetSignatureObject(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetContents(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFSignatureObj_GetContents(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetByteRange(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_int,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFSignatureObj_GetByteRange(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetSubFilter(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFSignatureObj_GetSubFilter(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetReason(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFSignatureObj_GetReason(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetTime(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFSignatureObj_GetTime(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetDocMDPPermission(&self, signature: FPDF_SIGNATURE) -> c_uint {
        unsafe { crate::bindgen::FPDFSignatureObj_GetDocMDPPermission(signature) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetForPage(&self, page: FPDF_PAGE) -> FPDF_STRUCTTREE {
        unsafe { crate::bindgen::FPDF_StructTree_GetForPage(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE) {
        unsafe { crate::bindgen::FPDF_StructTree_Close(struct_tree) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_CountChildren(&self, struct_tree: FPDF_STRUCTTREE) -> c_int {
        unsafe { crate::bindgen::FPDF_StructTree_CountChildren(struct_tree) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: FPDF_STRUCTTREE,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT {
        unsafe { crate::bindgen::FPDF_StructTree_GetChildAtIndex(struct_tree, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetAltText(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_StructElement_GetAltText(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_StructElement_GetID(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetLang(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_StructElement_GetLang(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetStringAttribute(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        let c_attr_name = CString::new(attr_name).unwrap();

        unsafe {
            crate::bindgen::FPDF_StructElement_GetStringAttribute(
                struct_element,
                c_attr_name.as_ptr(),
                buffer,
                buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetMarkedContentID(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_StructElement_GetMarkedContentID(struct_element) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_StructElement_GetType(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetTitle(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDF_StructElement_GetTitle(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_CountChildren(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_StructElement_CountChildren(struct_element) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetChildAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT {
        unsafe { crate::bindgen::FPDF_StructElement_GetChildAtIndex(struct_element, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_New(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: c_double,
        height: c_double,
    ) -> FPDF_PAGE {
        unsafe { crate::bindgen::FPDFPage_New(document, page_index, width, height) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: c_int) {
        unsafe { crate::bindgen::FPDFPage_Delete(document, page_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int {
        unsafe { crate::bindgen::FPDFPage_GetRotation(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int) {
        unsafe { crate::bindgen::FPDFPage_SetRotation(page, rotate) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDF_GetPageBoundingBox(page, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_GetMediaBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetCropBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_GetCropBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_GetBleedBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_GetTrimBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetArtBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_GetArtBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        unsafe { crate::bindgen::FPDFPage_SetMediaBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetCropBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        unsafe { crate::bindgen::FPDFPage_SetCropBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        unsafe { crate::bindgen::FPDFPage_SetBleedBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        unsafe { crate::bindgen::FPDFPage_SetTrimBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetArtBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        unsafe { crate::bindgen::FPDFPage_SetArtBox(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_TransFormWithClip(
        &self,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_TransFormWithClip(page, matrix, clipRect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPathSegments(&self, clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int {
        unsafe { crate::bindgen::FPDFClipPath_CountPathSegments(clip_path, path_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT {
        unsafe { crate::bindgen::FPDFClipPath_GetPathSegment(clip_path, path_index, segment_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_HasTransparency(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_GenerateContent(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_CreateEx(
        &self,
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP {
        unsafe { crate::bindgen::FPDFBitmap_CreateEx(width, height, format, first_scan, stride) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        unsafe { crate::bindgen::FPDFBitmap_Destroy(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { crate::bindgen::FPDFBitmap_GetFormat(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    ) {
        unsafe {
            crate::bindgen::FPDFBitmap_FillRect(bitmap, left, top, width, height, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void {
        unsafe { crate::bindgen::FPDFBitmap_GetBuffer(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { crate::bindgen::FPDFBitmap_GetWidth(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { crate::bindgen::FPDFBitmap_GetHeight(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { crate::bindgen::FPDFBitmap_GetStride(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmap(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ) {
        unsafe {
            crate::bindgen::FPDF_RenderPageBitmap(
                bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
            );
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmapWithMatrix(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ) {
        unsafe {
            crate::bindgen::FPDF_RenderPageBitmapWithMatrix(bitmap, page, matrix, clipping, flags);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_IsSupportedSubtype(subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION {
        unsafe { crate::bindgen::FPDFPage_CreateAnnot(page, subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> c_int {
        unsafe { crate::bindgen::FPDFPage_GetAnnotCount(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION {
        unsafe { crate::bindgen::FPDFPage_GetAnnot(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { crate::bindgen::FPDFPage_GetAnnotIndex(page, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) {
        unsafe { crate::bindgen::FPDFPage_CloseAnnot(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_RemoveAnnot(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE {
        unsafe { crate::bindgen::FPDFAnnot_GetSubtype(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_IsObjectSupportedSubtype(subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_UpdateObject(annot, obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_AddInkStroke(annot, points, point_count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_RemoveInkList(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_AppendObject(annot, obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetObjectCount(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFAnnot_GetObject(annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_RemoveObject(annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetColor(
        &self,
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_SetColor(annot, color_type, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetColor(
        &self,
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_GetColor(annot, color_type, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_HasAttachmentPoints(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_SetAttachmentPoints(annot, quad_index, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_AppendAttachmentPoints(annot, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> size_t {
        unsafe { crate::bindgen::FPDFAnnot_CountAttachmentPoints(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_GetAttachmentPoints(annot, quad_index, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_SetRect(annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_GetRect(annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetVertices(annot, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetInkListCount(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListPath(
        &self,
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetInkListPath(annot, path_index, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_GetLine(annot, start, end) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFAnnot_SetBorder(
                annot,
                horizontal_radius,
                vertical_radius,
                border_width,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut f32,
        vertical_radius: *mut f32,
        border_width: *mut f32,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFAnnot_GetBorder(
                annot,
                horizontal_radius,
                vertical_radius,
                border_width,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasKey(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_HasKey(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_GetValueType(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_SetStringValue(annot, c_key.as_ptr(), value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_GetStringValue(annot, c_key.as_ptr(), buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetNumberValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: *mut f32,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_GetNumberValue(annot, c_key.as_ptr(), value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_SetAP(annot, appearanceMode, value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetAP(annot, appearanceMode, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_ANNOTATION {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_GetLinkedAnnot(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetFlags(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_SetFlags(annot, flags) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetFormFieldFlags(handle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION {
        unsafe { crate::bindgen::FPDFAnnot_GetFormFieldAtPoint(hHandle, page, point) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldName(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetFormFieldName(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldType(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetFormFieldType(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetFormFieldValue(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetOptionCount(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionLabel(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetOptionLabel(hHandle, annot, index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_IsOptionSelected(handle, annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_GetFontSize(hHandle, annot, value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_IsChecked(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_SetFocusableSubtypes(hHandle, subtypes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetFocusableSubtypesCount(hHandle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAnnot_GetFocusableSubtypes(hHandle, subtypes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK {
        unsafe { crate::bindgen::FPDFAnnot_GetLink(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetFormControlCount(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFAnnot_GetFormControlIndex(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldExportValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAnnot_GetFormFieldExportValue(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: &str) -> FPDF_BOOL {
        let c_uri = CString::new(uri).unwrap();

        unsafe { crate::bindgen::FPDFAnnot_SetURI(annot, c_uri.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        unsafe { crate::bindgen::FPDFDOC_InitFormFillEnvironment(document, form_info) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE) {
        unsafe {
            crate::bindgen::FPDFDOC_ExitFormFillEnvironment(handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDFDoc_GetPageMode(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_Flatten(&self, page: FPDF_PAGE, nFlag: c_int) -> c_int {
        unsafe { crate::bindgen::FPDFPage_Flatten(page, nFlag) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    ) {
        unsafe {
            crate::bindgen::FPDF_SetFormFieldHighlightColor(handle, field_type, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar) {
        unsafe {
            crate::bindgen::FPDF_SetFormFieldHighlightAlpha(handle, alpha);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_FFLDraw(
        &self,
        handle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ) {
        unsafe {
            crate::bindgen::FPDF_FFLDraw(
                handle, bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
            );
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        unsafe { crate::bindgen::FPDFBookmark_GetFirstChild(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        unsafe { crate::bindgen::FPDFBookmark_GetNextSibling(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFBookmark_GetTitle(bookmark, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK {
        unsafe { crate::bindgen::FPDFBookmark_Find(document, title) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST {
        unsafe { crate::bindgen::FPDFBookmark_GetDest(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION {
        unsafe { crate::bindgen::FPDFBookmark_GetAction(bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong {
        unsafe { crate::bindgen::FPDFAction_GetType(action) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST {
        unsafe { crate::bindgen::FPDFAction_GetDest(document, action) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAction_GetFilePath(action, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetURIPath(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAction_GetURIPath(document, action, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int {
        unsafe { crate::bindgen::FPDFDest_GetDestPageIndex(document, dest) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetView(
        &self,
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFDest_GetView(dest, pNumParams, pParams) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetLocationInPage(
        &self,
        dest: FPDF_DEST,
        hasXVal: *mut FPDF_BOOL,
        hasYVal: *mut FPDF_BOOL,
        hasZoomVal: *mut FPDF_BOOL,
        x: *mut FS_FLOAT,
        y: *mut FS_FLOAT,
        zoom: *mut FS_FLOAT,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFDest_GetLocationInPage(
                dest, hasXVal, hasYVal, hasZoomVal, x, y, zoom,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> FPDF_LINK {
        unsafe { crate::bindgen::FPDFLink_GetLinkAtPoint(page, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkZOrderAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> c_int {
        unsafe { crate::bindgen::FPDFLink_GetLinkZOrderAtPoint(page, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetDest(&self, document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST {
        unsafe { crate::bindgen::FPDFLink_GetDest(document, link) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAction(&self, link: FPDF_LINK) -> FPDF_ACTION {
        unsafe { crate::bindgen::FPDFLink_GetAction(link) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_Enumerate(
        &self,
        page: FPDF_PAGE,
        start_pos: *mut c_int,
        link_annot: *mut FPDF_LINK,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFLink_Enumerate(page, start_pos, link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnot(&self, page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION {
        unsafe { crate::bindgen::FPDFLink_GetAnnot(page, link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnotRect(&self, link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFLink_GetAnnotRect(link_annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountQuadPoints(&self, link_annot: FPDF_LINK) -> c_int {
        unsafe { crate::bindgen::FPDFLink_CountQuadPoints(link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetQuadPoints(
        &self,
        link_annot: FPDF_LINK,
        quad_index: c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFLink_GetQuadPoints(link_annot, quad_index, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE {
        unsafe { crate::bindgen::FPDFText_LoadPage(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        unsafe {
            crate::bindgen::FPDFText_ClosePage(text_page);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int {
        unsafe { crate::bindgen::FPDFText_CountChars(text_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetUnicode(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint {
        unsafe { crate::bindgen::FPDFText_GetUnicode(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontSize(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_double {
        unsafe { crate::bindgen::FPDFText_GetFontSize(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontInfo(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        flags: *mut c_int,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFText_GetFontInfo(text_page, index, buffer, buflen, flags) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontWeight(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_int {
        unsafe { crate::bindgen::FPDFText_GetFontWeight(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextRenderMode(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
    ) -> FPDF_TEXT_RENDERMODE {
        unsafe { crate::bindgen::FPDFText_GetTextRenderMode(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFillColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetFillColor(text_page, index, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetStrokeColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetStrokeColor(text_page, index, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharAngle(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_float {
        unsafe { crate::bindgen::FPDFText_GetCharAngle(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        left: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
        top: *mut c_double,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetCharBox(text_page, index, left, right, bottom, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetLooseCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetLooseCharBox(text_page, index, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetMatrix(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetMatrix(text_page, index, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharOrigin(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        x: *mut c_double,
        y: *mut c_double,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetCharOrigin(text_page, index, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexAtPos(
        &self,
        text_page: FPDF_TEXTPAGE,
        x: c_double,
        y: c_double,
        xTolerance: c_double,
        yTolerance: c_double,
    ) -> c_int {
        unsafe {
            crate::bindgen::FPDFText_GetCharIndexAtPos(text_page, x, y, xTolerance, yTolerance)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetText(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
        result: *mut c_ushort,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFText_GetText(text_page, start_index, count, result) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountRects(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFText_CountRects(text_page, start_index, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetRect(
        &self,
        text_page: FPDF_TEXTPAGE,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_GetRect(text_page, rect_index, left, top, right, bottom) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetBoundedText(
        &self,
        text_page: FPDF_TEXTPAGE,
        left: c_double,
        top: c_double,
        right: c_double,
        bottom: c_double,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int {
        unsafe {
            crate::bindgen::FPDFText_GetBoundedText(
                text_page, left, top, right, bottom, buffer, buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindStart(
        &self,
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE {
        unsafe { crate::bindgen::FPDFText_FindStart(text_page, findwhat, flags, start_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindNext(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_FindNext(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindPrev(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_FindPrev(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetSchResultIndex(&self, handle: FPDF_SCHHANDLE) -> c_int {
        unsafe { crate::bindgen::FPDFText_GetSchResultIndex(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetSchCount(&self, handle: FPDF_SCHHANDLE) -> c_int {
        unsafe { crate::bindgen::FPDFText_GetSchCount(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE) {
        unsafe { crate::bindgen::FPDFText_FindClose(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_LoadWebLinks(&self, text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK {
        unsafe { crate::bindgen::FPDFLink_LoadWebLinks(text_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountWebLinks(&self, link_page: FPDF_PAGELINK) -> c_int {
        unsafe { crate::bindgen::FPDFLink_CountWebLinks(link_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetURL(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int {
        unsafe { crate::bindgen::FPDFLink_GetURL(link_page, link_index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountRects(&self, link_page: FPDF_PAGELINK, link_index: c_int) -> c_int {
        unsafe { crate::bindgen::FPDFLink_CountRects(link_page, link_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetRect(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFLink_GetRect(
                link_page, link_index, rect_index, left, top, right, bottom,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetTextRange(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        start_char_index: *mut c_int,
        char_count: *mut c_int,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFLink_GetTextRange(
                link_page,
                link_index,
                start_char_index,
                char_count,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK) {
        unsafe { crate::bindgen::FPDFLink_CloseWebLinks(link_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetDecodedThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFPage_GetDecodedThumbnailData(page, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRawThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFPage_GetRawThumbnailData(page, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetThumbnailAsBitmap(&self, page: FPDF_PAGE) -> FPDF_BITMAP {
        unsafe { crate::bindgen::FPDFPage_GetThumbnailAsBitmap(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFFormObj_CountObjects(form_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFFormObj_GetObject(form_object, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFPageObj_CreateTextObj(document, font, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE {
        unsafe { crate::bindgen::FPDFTextObj_GetTextRenderMode(text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFTextObj_SetTextRenderMode(text, render_mode) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetText(
        &self,
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFTextObj_GetText(text_object, text_page, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT {
        unsafe { crate::bindgen::FPDFTextObj_GetFont(text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFTextObj_GetFontSize(text, size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_Close(&self, font: FPDF_FONT) {
        unsafe { crate::bindgen::FPDFFont_Close(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPath_MoveTo(path, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPath_LineTo(path, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_BezierTo(
        &self,
        path: FPDF_PAGEOBJECT,
        x1: c_float,
        y1: c_float,
        x2: c_float,
        y2: c_float,
        x3: c_float,
        y3: c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPath_BezierTo(path, x1, y1, x2, y2, x3, y3) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPath_Close(path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_SetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPath_SetDrawMode(path, fillmode, stroke) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPath_GetDrawMode(path, fillmode, stroke) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: &str,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        let c_font = CString::new(font).unwrap();

        unsafe { crate::bindgen::FPDFPageObj_NewTextObj(document, c_font.as_ptr(), font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_SetText(text_object, text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFText_SetCharcodes(text_object, charcodes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadFont(
        &self,
        document: FPDF_DOCUMENT,
        data: *const c_uchar,
        size: c_uint,
        font_type: c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT {
        unsafe { crate::bindgen::FPDFText_LoadFont(document, data, size, font_type, cid) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadStandardFont(&self, document: FPDF_DOCUMENT, font: &str) -> FPDF_FONT {
        let c_font = CString::new(font).unwrap();

        unsafe { crate::bindgen::FPDFText_LoadStandardFont(document, c_font.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) {
        unsafe { crate::bindgen::FPDFPage_InsertObject(page, page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPage_RemoveObject(page, page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> c_int {
        unsafe { crate::bindgen::FPDFPage_CountObjects(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFPage_GetObject(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Destroy(&self, page_obj: FPDF_PAGEOBJECT) {
        unsafe { crate::bindgen::FPDFPageObj_Destroy(page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_HasTransparency(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFPageObj_GetType(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Transform(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) {
        unsafe { crate::bindgen::FPDFPageObj_Transform(page_object, a, b, c, d, e, f) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetMatrix(page_object, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetMatrix(&self, path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetMatrix(path, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFPageObj_NewImageObj(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFPageObj_CountMarks(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECTMARK {
        unsafe { crate::bindgen::FPDFPageObj_GetMark(page_object, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_AddMark(&self, page_object: FPDF_PAGEOBJECT, name: &str) -> FPDF_PAGEOBJECTMARK {
        let c_name = CString::new(name).unwrap();

        unsafe { crate::bindgen::FPDFPageObj_AddMark(page_object, c_name.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_RemoveMark(page_object, mark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetName(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObjMark_GetName(mark, buffer, buflen, out_buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> c_int {
        unsafe { crate::bindgen::FPDFPageObjMark_CountParams(mark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamKey(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        index: c_ulong,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFPageObjMark_GetParamKey(mark, index, buffer, buflen, out_buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFPageObjMark_GetParamValueType(mark, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        out_value: *mut c_int,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFPageObjMark_GetParamIntValue(mark, c_key.as_ptr(), out_value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe {
            crate::bindgen::FPDFPageObjMark_GetParamStringValue(
                mark,
                c_key.as_ptr(),
                buffer,
                buflen,
                out_buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe {
            crate::bindgen::FPDFPageObjMark_GetParamBlobValue(
                mark,
                c_key.as_ptr(),
                buffer,
                buflen,
                out_buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetIntParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: c_int,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe {
            crate::bindgen::FPDFPageObjMark_SetIntParam(
                document,
                page_object,
                mark,
                c_key.as_ptr(),
                value,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetStringParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: &str,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        let c_value = CString::new(value).unwrap();

        unsafe {
            crate::bindgen::FPDFPageObjMark_SetStringParam(
                document,
                page_object,
                mark,
                c_key.as_ptr(),
                c_value.as_ptr(),
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe {
            crate::bindgen::FPDFPageObjMark_SetBlobParam(
                document,
                page_object,
                mark,
                c_key.as_ptr(),
                value,
                value_len,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFPageObjMark_RemoveParam(page_object, mark, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFile(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFImageObj_LoadJpegFile(pages, count, image_object, file_access)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFileInline(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFImageObj_LoadJpegFileInline(pages, count, image_object, file_access)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_SetMatrix(
        &self,
        image_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFImageObj_SetMatrix(image_object, a, b, c, d, e, f) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_SetBitmap(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFImageObj_SetBitmap(pages, count, image_object, bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP {
        unsafe { crate::bindgen::FPDFImageObj_GetBitmap(image_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        unsafe { crate::bindgen::FPDFImageObj_GetRenderedBitmap(document, page, image_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFImageObj_GetImageDataDecoded(image_object, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFImageObj_GetImageDataRaw(image_object, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFImageObj_GetImageFilterCount(image_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilter(
        &self,
        image_object: FPDF_PAGEOBJECT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFImageObj_GetImageFilter(image_object, index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFImageObj_GetImageMetadata(image_object, page, metadata) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewPath(&self, x: c_float, y: c_float) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFPageObj_CreateNewPath(x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewRect(
        &self,
        x: c_float,
        y: c_float,
        w: c_float,
        h: c_float,
    ) -> FPDF_PAGEOBJECT {
        unsafe { crate::bindgen::FPDFPageObj_CreateNewRect(x, y, w, h) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetBounds(
        &self,
        page_object: FPDF_PAGEOBJECT,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetBounds(page_object, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: &str) {
        let c_blend_mode = CString::new(blend_mode).unwrap();

        unsafe { crate::bindgen::FPDFPageObj_SetBlendMode(page_object, c_blend_mode.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetStrokeColor(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetStrokeColor(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetStrokeWidth(page_object, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetStrokeWidth(page_object, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFPageObj_GetLineJoin(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetLineJoin(page_object, line_join) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFPageObj_GetLineCap(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetLineCap(page_object, line_cap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetFillColor(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetFillColor(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetDashPhase(page_object, phase) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_SetDashPhase(page_object, phase) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFPageObj_GetDashCount(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPageObj_GetDashArray(page_object, dash_array, dash_count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const c_float,
        dash_count: size_t,
        phase: c_float,
    ) -> FPDF_BOOL {
        unsafe {
            crate::bindgen::FPDFPageObj_SetDashArray(page_object, dash_array, dash_count, phase)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_CountSegments(&self, path: FPDF_PAGEOBJECT) -> c_int {
        unsafe { crate::bindgen::FPDFPath_CountSegments(path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetPathSegment(&self, path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT {
        unsafe { crate::bindgen::FPDFPath_GetPathSegment(path, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetPoint(
        &self,
        segment: FPDF_PATHSEGMENT,
        x: *mut c_float,
        y: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPathSegment_GetPoint(segment, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetType(&self, segment: FPDF_PATHSEGMENT) -> c_int {
        unsafe { crate::bindgen::FPDFPathSegment_GetType(segment) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetClose(&self, segment: FPDF_PATHSEGMENT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFPathSegment_GetClose(segment) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFFont_GetFontName(font, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> c_int {
        unsafe { crate::bindgen::FPDFFont_GetFlags(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> c_int {
        unsafe { crate::bindgen::FPDFFont_GetWeight(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetItalicAngle(&self, font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFFont_GetItalicAngle(font, angle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetAscent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        ascent: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFFont_GetAscent(font, font_size, ascent) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetDescent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        descent: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFFont_GetDescent(font, font_size, descent) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphWidth(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFFont_GetGlyphWidth(font, glyph, font_size, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphPath(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
    ) -> FPDF_GLYPHPATH {
        unsafe { crate::bindgen::FPDFFont_GetGlyphPath(font, glyph, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: FPDF_GLYPHPATH) -> c_int {
        unsafe { crate::bindgen::FPDFGlyphPath_CountGlyphSegments(glyphpath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: FPDF_GLYPHPATH,
        index: c_int,
    ) -> FPDF_PATHSEGMENT {
        unsafe { crate::bindgen::FPDFGlyphPath_GetGlyphPathSegment(glyphpath, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDF_VIEWERREF_GetPrintScaling(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetNumCopies(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDF_VIEWERREF_GetNumCopies(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRange(&self, document: FPDF_DOCUMENT) -> FPDF_PAGERANGE {
        unsafe { crate::bindgen::FPDF_VIEWERREF_GetPrintPageRange(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> size_t {
        unsafe { crate::bindgen::FPDF_VIEWERREF_GetPrintPageRangeCount(pagerange) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: FPDF_PAGERANGE,
        index: size_t,
    ) -> c_int {
        unsafe { crate::bindgen::FPDF_VIEWERREF_GetPrintPageRangeElement(pagerange, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE {
        unsafe { crate::bindgen::FPDF_VIEWERREF_GetDuplex(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetName(
        &self,
        document: FPDF_DOCUMENT,
        key: &str,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDF_VIEWERREF_GetName(document, c_key.as_ptr(), buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachmentCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { crate::bindgen::FPDFDoc_GetAttachmentCount(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_AddAttachment(
        &self,
        document: FPDF_DOCUMENT,
        name: FPDF_WIDESTRING,
    ) -> FPDF_ATTACHMENT {
        unsafe { crate::bindgen::FPDFDoc_AddAttachment(document, name) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT {
        unsafe { crate::bindgen::FPDFDoc_GetAttachment(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_DeleteAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFDoc_DeleteAttachment(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetName(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { crate::bindgen::FPDFAttachment_GetName(attachment, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_HasKey(&self, attachment: FPDF_ATTACHMENT, key: &str) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAttachment_HasKey(attachment, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetValueType(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAttachment_GetValueType(attachment, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { crate::bindgen::FPDFAttachment_SetStringValue(attachment, c_key.as_ptr(), value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        let c_key = CString::new(key).unwrap();

        unsafe {
            crate::bindgen::FPDFAttachment_GetStringValue(
                attachment,
                c_key.as_ptr(),
                buffer,
                buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const c_void,
        len: c_ulong,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAttachment_SetFile(attachment, document, contents, len) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        unsafe { crate::bindgen::FPDFAttachment_GetFile(attachment, buffer, buflen, out_buflen) }
    }
}
