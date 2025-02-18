//! Defines the [PdfActionLocalDestination] struct, exposing functionality related to a single
//! action of type `PdfActionType::GoToDestinationInSameDocument`.

use crate::action_private::internal::PdfActionPrivate;
use crate::bindgen::{FPDF_ACTION, FPDF_DOCUMENT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::prelude::PdfDestination;

pub struct PdfActionLocalDestination<'a> {
    handle: FPDF_ACTION,
    document: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfActionLocalDestination<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ACTION,
        document: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfActionLocalDestination {
            handle,
            document,
            bindings,
        }
    }

    /// Returns the target [PdfDestination] for this [PdfActionLocalDestination].
    pub fn destination(&self) -> Result<PdfDestination, PdfiumError> {
        let handle = self.bindings.FPDFAction_GetDest(self.document, self.handle);

        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfDestination::from_pdfium(handle, self.bindings))
        }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionLocalDestination<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
