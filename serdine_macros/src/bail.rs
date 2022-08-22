macro_rules! bail {
    ( $msg:expr $(,)? ) => {
        return syn::Result::<_>::Err(syn::Error::new(proc_macro2::Span::call_site(), &$msg))
    };

    ( $msg:expr => $spanned:expr $(,)? ) => {
        return syn::Result::<_>::Err(syn::Error::new_spanned(&$spanned, &$msg))
    };
}

pub(crate) use bail;
