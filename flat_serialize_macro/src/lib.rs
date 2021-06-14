
use proc_macro::{TokenStream};

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};

use syn::{Attribute, Expr, Field, Ident, Token, parse_macro_input, punctuated::Punctuated, spanned::Spanned, visit_mut::VisitMut};

mod parser;

#[proc_macro]
pub fn flat_serialize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as FlatSerialize);
    let expanded = match input {
        FlatSerialize::Struct(input) => flat_serialize_struct(input),
        FlatSerialize::Enum(input) => flat_serialize_enum(input),
    };
    if cfg!(feature = "print-generated") {
        println!("{}", expanded.to_string());
    }
    expanded.into()
}

enum FlatSerialize {
    Enum(FlatSerializeEnum),
    Struct(FlatSerializeStruct),
}

/// a `flat_serialize`d enum e.g.
/// ```skip
/// flat_serialize! {
///     enum BasicEnum {
///         k: u8,
///         First: 2 {
///             data_len: usize,
///             data: [u8; self.data_len],
///         },
///         Fixed: 3 {
///             array: [u16; 3],
///         },
///     }
/// }
/// ```
/// the body of the enum variants must be the a valid FlatSerializeStruct body
struct FlatSerializeEnum {
    per_field_attrs: Vec<PerFieldsAttr>,
    attrs: Vec<Attribute>,
    ident: Ident,
    tag: FlatSerializeField,
    variants: Punctuated<FlatSerializeVariant, Token![,]>,
}

struct FlatSerializeVariant {
    tag_val: Expr,
    body: FlatSerializeStruct,
}

/// a `flat_serialize`d struct e.g.
/// ```skip
/// flat_serialize! {
///     #[derive(Debug)]
///     struct Basic {
///         header: u64,
///         data_len: u32,
///         array: [u16; 3],
///         data: [u8; self.data_len],
///         data2: [u8; self.data_len / 2],
///     }
/// }
/// ```
/// the syntax is the same as a regular struct, except that it allows
/// `self` expressions in the length of arrays; these will be represented as
/// variable-length fields. We also interpret
/// `#[flat_serialize::field_attr(fixed = "#[foo]", variable = "#[bar]"))]` as
/// applying the attribute `#[foo]` to every fixed-length field of the struct,
/// and `#[bar]` to every variable-length field. e.g.
/// ```skip
/// flat_serialize! {
///     #[flat_serialize::field_attr(fixed = "#[foo]", variable = "#[bar]"))]`
///     struct Struct {
///         a: i32,
///         b: i32,
///         c: [u16; self.a]
///         d: [u8; self.a]
///     }
/// ```
/// is equivalent to
/// ```skip
/// flat_serialize! {
///     struct Struct {
///         #[foo]
///         a: i32,
///         #[foo]
///         b: i32,
///         #[bar]
///         c: [u16; self.a]
///         #[bar]
///         d: [u8; self.a]
///     }
/// ```
/// This can be useful when generating flat_serialize structs from a macro
struct FlatSerializeStruct {
    per_field_attrs: Vec<PerFieldsAttr>,
    attrs: Vec<Attribute>,
    ident: Ident,
    fields: Punctuated<FlatSerializeField, Token![,]>,
}
struct FlatSerializeField {
    field: Field,
    /// call try_ref(...)/fill_vec(...) for de/serialization
    flatten: bool,
    // TODO is this mutually exclusive with `flatten` above? Should we make an
    // enum to select between them?
    length_info: Option<VariableLenFieldInfo>,
}

/// a `#[flat_serialize::field_attr(fixed = "#[foo]", variable = "#[bar]"))]`
/// attribute. The inner attribute(s) will be applied to each relevant field.
struct PerFieldsAttr {
    fixed: Attribute,
    variable: Option<Attribute>,
}

/// how to find the length of a variable-length or optional field.
struct VariableLenFieldInfo {
    ty: syn::Type,
    len_expr: syn::Expr,
    // is an optional field instead of a general varlen field, len_expr should
    // eval to a boolean
    is_optional: bool,
}

fn flat_serialize_struct(input: FlatSerializeStruct) -> TokenStream2 {
    let ident = input.ident.clone();

    let ref_def = {
        let alignment_check = input.alignment_check(quote!(0), quote!(8));
        let trait_check = input.fn_trait_check();
        let required_alignment = input.fn_required_alignment();
        let max_provided_alignment = input.fn_max_provided_alignment();
        let min_len = input.fn_min_len();
        let try_ref = input.fn_try_ref();
        let fill_vec = input.fn_fill_vec();
        let len = input.fn_len();
        let fields = input
            .fields
            .iter()
            .map(|f| f.declaration(true, input.per_field_attrs.iter()));
        let attrs = &*input.attrs;

        quote! {
            #[derive(Copy, Clone)]
            #(#attrs)*
            pub struct #ident<'a> {
                #(#fields)*
            }

            #alignment_check

            #trait_check

            impl<'a> #ident<'a> {
                #required_alignment

                #max_provided_alignment

                #min_len

                #try_ref

                #fill_vec

                #len
            }

            unsafe impl<'a> FlattenableRef<'a> for #ident<'a> {}
        }
    };

    // TODO
    // let mut_def = {
    //     let fields: Vec<_> = input.fields.iter().enumerate().map(|(i, f)| {
    //         let name = f.ident.as_ref().unwrap();
    //         let ty = exposed_ty(&field_paths[i], &f.ty);
    //         quote! { pub #name: &'a mut #ty, }
    //     }).collect();
    //     quote! {
    //         #[derive(Copy, Clone)]
    //         pub struct Mut<'a> {
    //             #(#fields)*
    //         }
    //     }
    // };

    let expanded = quote! {
        #ref_def
        // #mut_def
    };

    expanded
}

fn flat_serialize_enum(input: FlatSerializeEnum) -> TokenStream2 {
    let alignment_check = input.alignment_check();
    let uniqueness_check = input.uniqueness_check();
    let trait_check = input.fn_trait_check();
    let required_alignment = input.fn_required_alignment();
    let max_provided_alignment = input.fn_max_provided_alignment();
    let min_len = input.fn_min_len();
    let try_ref = input.fn_try_ref();
    let fill_vec = input.fn_fill_vec();
    let len = input.fn_len();
    let body = input.variants();
    let ident = &input.ident;
    let attrs = &*input.attrs;

    quote! {
        #[derive(Copy, Clone)]
        #(#attrs)*
        #body

        #alignment_check

        #uniqueness_check

        #trait_check

        impl<'a> #ident<'a> {
            #required_alignment

            #max_provided_alignment

            #min_len

            #try_ref

            #fill_vec

            #len
        }

        unsafe impl<'a> FlattenableRef<'a> for #ident<'a> {}
    }
}

impl VariableLenFieldInfo {
    fn len_from_bytes(&self) -> TokenStream2 {
        let mut lfb = SelfReplacer(|name|
            syn::parse_quote! { #name.cloned().unwrap() }
        );
        let mut len = self.len_expr.clone();
        lfb.visit_expr_mut(&mut len);
        quote! { #len }
    }

    fn counter_expr(&self) -> TokenStream2 {
        let mut ce = SelfReplacer(|name|
            syn::parse_quote! { (*#name) }
        );
        let mut len = self.len_expr.clone();
        ce.visit_expr_mut(&mut len);
        quote! { #len }
    }

    fn err_size_expr(&self) -> TokenStream2 {
        let mut ese = SelfReplacer(|name|
            syn::parse_quote! {
                match #name { Some(#name) => *#name, None => return 0usize, }
            }
        );
        let mut len = self.len_expr.clone();
        ese.visit_expr_mut(&mut len);
        quote! { #len }
    }
}

struct SelfReplacer<F: FnMut(&Ident) -> syn::Expr>(F);

impl<F: FnMut(&Ident) -> syn::Expr> VisitMut for SelfReplacer<F> {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if let syn::Expr::Field(field) = expr {
            if let syn::Expr::Path(path) = &mut *field.base {
                if path.path.segments[0].ident == "self" {
                    let name = match &field.member {
                        syn::Member::Named(name) => name,
                        syn::Member::Unnamed(_) => panic!("unnamed fields not supported"),
                    };
                    *expr = self.0(name)
                }
            }
        } else {
            syn::visit_mut::visit_expr_mut(self, expr)
        }
    }
}

struct TryRefBody {
    vars: TokenStream2,
    body: TokenStream2,
    set_fields: TokenStream2,
    err_size: TokenStream2,
}

impl FlatSerializeEnum {
    fn variants(&self) -> TokenStream2 {
        let id = &self.ident;
        let variants = self.variants.iter().map(|variant| {
            let fields = variant.body.fields.iter().map(|f|
                f.declaration(false, self.per_field_attrs.iter())
            );
            let ident = &variant.body.ident;
            quote! {
                #ident {
                    #(#fields)*
                },
            }
        });
        quote! {
            pub enum #id<'a> {
                #(#variants)*
            }
        }
    }

    fn uniqueness_check(&self) -> TokenStream2 {
        let variants = self.variants.iter().map(|variant| {
            let ident = &variant.body.ident;
            let tag_val = &variant.tag_val;
            quote! {
                #ident = #tag_val,
            }
        });
        quote! {
            // uniqueness check
            const _: () = {
                #[allow(dead_code)]
                enum UniquenessCheck {
                    #(#variants)*
                }
            };
        }
    }

    fn alignment_check(&self) -> TokenStream2 {
        let mut size = quote! { 0 };
        let mut min_align = quote! { 8 };
        let tag_check = self.tag.alignment_check(&mut size, &mut min_align);
        let variant_checks = self.variants.iter()
            .map(|v| v.body.alignment_check(size.clone(), min_align.clone()));
        quote! {
            // alignment assertions
            const _: () = {
                use std::mem::{align_of, size_of};
                #tag_check
                #(#variant_checks)*
            };
        }
    }

    fn fn_trait_check(&self) -> TokenStream2 {
        let tag_check = self.tag.trait_check();
        let checks = self.variants.iter().map(|v| v.body.fn_trait_check());
        quote! {
            const _: () = {
                #tag_check
                #(
                    const _: () = {
                        #checks
                    };
                )*
            };
        }
    }

    fn fn_required_alignment(&self) -> TokenStream2 {
        let tag_alignment = self.tag.required_alignment();
        let alignments = self.variants.iter().map(|v| {
            let alignments = v.body.fields.iter().map(|f| f.required_alignment());
            quote!{
                let mut required_alignment = #tag_alignment;
                #(
                    let alignment = #alignments;
                    if alignment > required_alignment {
                        required_alignment = alignment;
                    }
                )*
                required_alignment
            }
        });

        quote! {
            pub const fn required_alignment() -> usize {
                use std::mem::align_of;
                let mut required_alignment: usize = #tag_alignment;
                #(
                    let alignment: usize = {
                        #alignments
                    };
                    if alignment > required_alignment {
                        required_alignment = alignment;
                    }
                )*
                required_alignment
            }
        }
    }

    fn fn_max_provided_alignment(&self) -> TokenStream2 {
        let min_align = match self.tag.max_provided_alignment() {
            Some(align) => align,
            None => quote!(Some(8)),
        };

        let min_size = self.tag.min_len();

        let alignments = self.variants.iter().map(|v| {
            let alignments = v.body.fields.iter().flat_map(|f| f.max_provided_alignment());
            let sizes = v.body.fields.iter().map(|f| f.min_len());
            quote!{
                let mut min_align: Option<usize> = #min_align;
                #(
                    let alignment = {
                        #alignments
                    };
                    match (alignment, min_align) {
                        (None, _) => (),
                        (Some(align), None) => min_align = Some(align),
                        (Some(align), Some(min)) if align < min =>
                            min_align = Some(align),
                        _ => (),
                    }
                )*
                let variant_size: usize = #min_size #(+ #sizes)*;
                let effective_alignment = match min_align {
                    Some(align) => align,
                    None => 8,
                };

                if variant_size % 8 == 0 && effective_alignment >= 8 {
                    8
                } else if variant_size % 4 == 0 && effective_alignment >= 4 {
                    4
                } else if variant_size % 2 == 0 && effective_alignment >= 2 {
                    2
                } else {
                    1
                }
            }
        });
        quote! {
            pub const fn max_provided_alignment() -> Option<usize> {
                use std::mem::{align_of, size_of};
                let mut min_align: usize = match #min_align {
                    None => 8,
                    Some(align) => align,
                };
                #(
                    let variant_alignment: usize = {
                        #alignments
                    };
                    if variant_alignment < min_align {
                        min_align = variant_alignment
                    }
                )*
                let min_size = Self::min_len();
                if min_size % 8 == 0 && min_align >= 8 {
                    return Some(8)
                }
                if min_size % 4 == 0 && min_align >= 4 {
                    return Some(4)
                }
                if min_size % 2 == 0 && min_align >= 2 {
                    return Some(2)
                }
                return Some(1)
            }
        }
    }

    fn fn_min_len(&self) -> TokenStream2 {
        let tag_size = self.tag.min_len();
        let sizes = self.variants.iter().map(|v| {
            let sizes = v.body.fields.iter().map(|f| f.min_len());
            quote! {
                let mut size: usize = #tag_size;
                #(size += #sizes;)*
                size
            }
        });
        quote! {
            pub const fn min_len() -> usize {
                use std::mem::size_of;
                let mut size: Option<usize> = None;
                #(
                    let variant_size = {
                        #sizes
                    };
                    size = match size {
                        None => Some(variant_size),
                        Some(size) if size > variant_size => Some(variant_size),
                        Some(size) => Some(size),
                    };
                )*
                if let Some(size) = size {
                    return size
                }
                #tag_size
            }
        }
    }

    fn fn_try_ref(&self) -> TokenStream2 {
        let break_label = syn::Lifetime::new("'tryref_tag", proc_macro2::Span::call_site());
        let try_wrap_tag = self.tag.try_wrap(&break_label);
        let id = &self.ident;
        let tag_ty = &self.tag.ty;

        let bodies = self.variants.iter().enumerate().map(|(i, v)| {
            let tag_val = &v.tag_val;

            let variant = &v.body.ident;

            let break_label =
                syn::Lifetime::new(&format!("'tryref_{}", i), proc_macro2::Span::call_site());

            let TryRefBody {
                vars,
                body,
                set_fields,
                err_size,
            } = v
                .body
                .fn_try_ref_body(&break_label);

            quote! {
                Some(&#tag_val) => {
                    #vars
                    #break_label: loop {
                        #body
                        let _ref = #id::#variant { #set_fields };
                        return Ok((_ref, __packet_macro_bytes))
                    }
                    return Err(WrapErr::NotEnoughBytes(std::mem::size_of::<#tag_ty>() #err_size))
                }
            }
        });

        let tag_ident = self.tag.ident.as_ref().unwrap();

        quote! {
            #[allow(unused_assignments, unused_variables)]
            #[inline(always)]
            pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
                let __packet_macro_read_len = 0usize;
                let mut #tag_ident = None;
                'tryref_tag: loop {
                    #try_wrap_tag;
                    match #tag_ident {
                        #(#bodies),*
                        _ => return Err(WrapErr::InvalidTag(0)),
                    }
                }
                //TODO
                Err(WrapErr::NotEnoughBytes(::std::mem::size_of::<#tag_ty>()))
            }
        }
    }

    fn fn_fill_vec(&self) -> TokenStream2 {
        let tag_ty = &self.tag.ty;
        let tag_ident = self.tag.ident.as_ref().unwrap();
        let fill_vec_tag = self.tag.fill_vec();
        let id = &self.ident;
        let bodies = self.variants.iter().map(|v| {
            let tag_val = &v.tag_val;
            let variant = &v.body.ident;
            let (fields, fill_vec_with) = v.body.fill_vec_body();
            quote! {
                &#id::#variant { #fields } => {
                    let #tag_ident: &#tag_ty = &#tag_val;
                    #fill_vec_tag
                    #fill_vec_with
                }
            }
        });
        quote! {
            #[allow(unused_assignments, unused_variables)]
            pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
                __packet_macro_bytes.reserve_exact(self.len());
                match self {
                    #(#bodies),*
                }
            }
        }
    }

    fn fn_len(&self) -> TokenStream2 {
        let tag_ty = &self.tag.ty;
        let tag_size = quote! { ::std::mem::size_of::<#tag_ty>() };
        let id = &self.ident;
        let bodies = self.variants.iter().map(|v| {
            let variant = &v.body.ident;

            let size = v
                .body
                .fields
                .iter()
                .map(|f| f.size_fn());
            let fields = v.body.fields.iter().map(|f| f.ident.as_ref().unwrap());
            quote! {
                &#id::#variant { #(#fields),* } => {
                    #tag_size #(+ #size)*
                },
            }
        });
        quote! {
            #[allow(unused_assignments, unused_variables)]
            pub fn len(&self) -> usize {
                match self {
                    #(#bodies)*
                }
            }
        }
    }
}

impl FlatSerializeStruct {
    fn alignment_check(
        &self,
        start: TokenStream2,
        min_align: TokenStream2,
    ) -> TokenStream2 {
        // we create the size/align values as ever-increasing expressions
        // instead of variables due to the way span info works: we want to use
        // the span of the inputted type so that errors are reported only at the
        // type that causes the misalignment, however, if the type is inputted
        // via a macro, that span will be unable to reference local variables
        // we create ourselves.
        let mut size = start;
        let mut min_align = min_align;

        let checks = self.fields.iter().map(|f| f.alignment_check(&mut size, &mut min_align));

        quote! {
            // alignment assertions
            const _: () = {
                use std::mem::{align_of, size_of};
                #(#checks)*
            };
        }
    }

    fn fn_trait_check(&self) -> TokenStream2 {
        let checks = self.fields.iter().map(|f| f.trait_check());
        quote! {
            const _: () = {
                #(#checks)*
            };
        }
    }

    fn fn_required_alignment(&self) -> TokenStream2 {
        let alignments = self.fields.iter().map(|f| f.required_alignment());
        quote! {
            pub const fn required_alignment() -> usize {
                use std::mem::align_of;
                let mut required_alignment = 1;
                #(
                    let alignment = #alignments;
                    if alignment > required_alignment {
                        required_alignment = alignment;
                    }
                )*
                required_alignment
            }
        }
    }

    fn fn_max_provided_alignment(&self) -> TokenStream2 {
        let alignments = self.fields.iter().flat_map(|f| f.max_provided_alignment());
        quote! {
            pub const fn max_provided_alignment() -> Option<usize> {
                use std::mem::align_of;
                let mut min_align: Option<usize> = None;
                #(
                    match (#alignments, min_align) {
                        (None, _) => (),
                        (Some(align), None) => min_align = Some(align),
                        (Some(align), Some(min)) if align < min =>
                            min_align = Some(align),
                        _ => (),
                    }
                )*
                let min_align = match min_align {
                    None => return None,
                    Some(min_align) => min_align,
                };
                let min_size = Self::min_len();
                if min_size % 8 == 0 && min_align >= 8 {
                    return Some(8)
                }
                if min_size % 4 == 0 && min_align >= 4 {
                    return Some(4)
                }
                if min_size % 2 == 0 && min_align >= 2 {
                    return Some(2)
                }
                return Some(1)
            }
        }
    }

    fn fn_min_len(&self) -> TokenStream2 {
        let sizes = self.fields.iter().map(|f| f.min_len());
        quote! {
            pub const fn min_len() -> usize {
                use std::mem::size_of;
                let mut size = 0;
                #(size += #sizes;)*
                size
            }
        }
    }

    fn fn_try_ref(
        &self,
    ) -> TokenStream2 {
        let break_label = syn::Lifetime::new("'tryref", proc_macro2::Span::call_site());
        let id = &self.ident;
        let TryRefBody {
            vars,
            body,
            set_fields,
            err_size,
        } = self.fn_try_ref_body(&break_label);
        quote! {
            #[allow(unused_assignments, unused_variables)]
            #[inline(always)]
            pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
                let __packet_macro_read_len = 0usize;
                #vars
                #break_label: loop {
                    #body
                    let _ref = #id { #set_fields };
                    return Ok((_ref, __packet_macro_bytes))
                }
                Err(WrapErr::NotEnoughBytes(0 #err_size))
            }
        }
    }

    fn fn_try_ref_body(
        &self,
        break_label: &syn::Lifetime,
    ) -> TryRefBody {
        let field_names = self
            .fields
            .iter()
            .map(|f| &f.ident);
        let ty1 =  self
            .fields
            .iter()
            .map(|f| f.local_ty());
        let field1 = field_names.clone();
        let field2 = field_names.clone();
        let field_setters = self.fields.iter().map(|field| {
            let name = &field.ident;
            if field.is_optional() {
                quote!{ #name }
            } else {
                quote!{ #name.unwrap() }
            }
        });

        let vars = quote!( #(let mut #field1: #ty1 = None;)* );
        let try_wrap_fields = self.fields.iter().map(|f| f.try_wrap(break_label));
        let body = quote! ( #(#try_wrap_fields)* );

        let set_fields = quote!( #(#field2: #field_setters),* );

        let err_size = self
        .fields
            .iter()
            .map(|f| f.err_size());
        let err_size = quote!( #( + #err_size)* );
        TryRefBody {
            vars,
            body,
            set_fields,
            err_size,
        }
    }

    fn fn_fill_vec(
        &self,
    ) -> TokenStream2 {
        let id = &self.ident;
        let (fields, fill_vec_with) = self.fill_vec_body();
        quote! {
            #[allow(unused_assignments, unused_variables)]
            pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
                __packet_macro_bytes.reserve_exact(self.len());
                let &#id { #fields } = self;
                #fill_vec_with
            }
        }
    }
    fn fill_vec_body(
        &self,
    ) -> (TokenStream2, TokenStream2) {
        //FIXME assert multiple values of counters are equal...
        let fill_vec_with = self.fields.iter().map(|f| f.fill_vec());
        let fill_vec_with = quote!( #(#fill_vec_with);* );

        let field = self.fields.iter().map(|f| f.ident.as_ref().unwrap());
        let fields = quote!( #(#field),* );
        (fields, fill_vec_with)
    }

    fn fn_len(
        &self,
    ) -> TokenStream2 {
        let size = self
            .fields
            .iter()
            .map(|f| f.size_fn());
        let field = self.fields.iter().map(|f| f.ident.as_ref().unwrap());
        let id = &self.ident;

        quote! {
            #[allow(unused_assignments, unused_variables)]
            pub fn len(&self) -> usize {
                let &#id { #(#field),* } = self;
                0usize #(+ #size)*
            }
        }
    }
}

impl FlatSerializeField {

    fn alignment_check(&self, size: &mut TokenStream2, min_align: &mut TokenStream2) -> TokenStream2 {
        use std::mem::replace;
        if self.flatten {
            let ty = parser::as_turbofish(&self.ty);

            let new_size = quote!{#size + #ty::min_len()};
            let new_min_align = quote!{
                match #ty::max_provided_alignment() {
                    Some(align) => {
                        if align < #min_align {
                            align
                        } else {
                            #min_align
                        }
                    },
                    None => #min_align
                }
            };

            let size = replace(size, new_size);
            let min_align = replace(min_align, new_min_align);
            return quote_spanned!{self.ty.span()=>
                let _alignment_check: () = [()][(#size) % #ty::required_alignment()];
                let _alignment_check2: () = [()][(#ty::required_alignment() > #min_align) as u8 as usize];
            }
        }
        match &self.length_info {
            None => {
                let ty = &self.ty;
                let new_size = quote!{#size + size_of::<#ty>()};
                let size = replace(size, new_size);
                quote_spanned!{self.ty.span()=>
                    let _alignment_check= [()][(#size) % align_of::<#ty>()];
                    let _alignment_check2 = [()][(align_of::<#ty>() > #min_align) as u8 as usize];
                    let _padding_check = [()][(size_of::<#ty>() < align_of::<#ty>()) as u8 as usize];
                }
            }
            Some(info) => {
                let ty = &info.ty;
                let new_min_align = quote!{
                    if align_of::<#ty>() < #min_align {
                        align_of::<#ty>()
                    } else {
                        #min_align
                    }
                };
                let min_align = replace(min_align, new_min_align);
                quote_spanned!{self.ty.span()=>
                    let _alignment_check: () = [()][(#size) % align_of::<#ty>()];
                    let _alignment_check2: () = [()][(align_of::<#ty>() > #min_align) as u8 as usize];
                    let _padding_check: () = [()][(size_of::<#ty>() < align_of::<#ty>()) as u8 as usize];
                }
            }
        }
    }

    fn trait_check(&self) -> TokenStream2 {
        if self.flatten {
            let ty = parser::as_turbofish(&self.ty);
            let name = self.ident.as_ref().unwrap();
            // based on static_assertions
            return quote_spanned!{self.ty.span()=>
                fn #name<'a, T: FlattenableRef<'a>>() {}
                let _ = #name::<#ty<'static>>;
            }
        }
        let ty = match &self.length_info {
            None => &self.ty,
            Some(info) => &info.ty,
        };
        let name = self.ident.as_ref().unwrap();
        return quote_spanned!{ty.span()=>
            fn #name<T: FlatSerializable>() {}
            let _ = #name::<#ty>;
        }
    }

    fn required_alignment(&self) -> TokenStream2 {
        if self.flatten {
            let ty = parser::as_turbofish(&self.ty);
            return quote_spanned!{self.ty.span()=>
                #ty::required_alignment()
            }
        }
        match &self.length_info {
            None => {
                let ty = &self.ty;
                quote_spanned!{self.ty.span()=>
                    align_of::<#ty>()
                }
            }
            Some(info) => {
                let ty = &info.ty;
                quote_spanned!{self.ty.span()=>
                    align_of::<#ty>()
                }
            }
        }
    }

    fn max_provided_alignment(&self) -> Option<TokenStream2> {
        if self.flatten {
            let ty = parser::as_turbofish(&self.ty);
            return Some(quote_spanned!{self.ty.span()=>
                #ty::max_provided_alignment()
            })
        }
        self.length_info.as_ref().map(|info| {
            let ty = &info.ty;
            quote_spanned!{self.ty.span()=>
                Some(align_of::<#ty>())
            }
        })
    }

    fn min_len(&self) -> TokenStream2 {
        let ty = &self.ty;
        if self.flatten {
            let ty = parser::as_turbofish(ty);
            return quote_spanned!{self.ty.span()=>
                #ty::min_len()
            }
        }
        match &self.length_info {
            None => quote_spanned!{self.ty.span()=>
                size_of::<#ty>()
            },
            Some(..) =>
                quote_spanned!{ty.span()=>
                    0
                },
        }
    }

    fn try_wrap(&self, break_label: &syn::Lifetime,) -> TokenStream2 {
        let ident = self.ident.as_ref().unwrap();
        let ty = &self.ty;

        if self.flatten {
            let ty = parser::as_turbofish(ty);
            return quote!{
                let __packet_macro_read_len: usize = {
                    let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                    let (__packet_macro_field, __packet_macro_rem_bytes) = match #ty::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) =>
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset)),
                        Err(..) => break #break_label

                    };
                    let __packet_macro_size = __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                    __packet_macro_bytes = __packet_macro_rem_bytes;
                    #ident = Some(__packet_macro_field);
                    __packet_macro_read_len + __packet_macro_size
                };
            };
        }
        match &self.length_info {
            Some(info @ VariableLenFieldInfo { is_optional: false, .. }) => {
                let count = info.len_from_bytes();
                let ty = &info.ty;
                quote! {
                    let __packet_macro_read_len: usize = {
                        let __packet_macro_count = (#count) as usize;
                        let __packet_macro_size = ::std::mem::size_of::<#ty>() * __packet_macro_count;
                        let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                        if __packet_macro_bytes.len() < __packet_macro_size {
                            break #break_label
                        }
                        let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                            __packet_macro_bytes.split_at(__packet_macro_size);
                        let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                        let __packet_macro_field = ::std::slice::from_raw_parts(
                            __packet_macro_field_ptr as *const #ty, __packet_macro_count);
                        debug_assert_eq!(
                            __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                            __packet_macro_field.as_ptr().offset(__packet_macro_count as isize) as usize
                        );
                        __packet_macro_bytes = __packet_macro_rem_bytes;
                        #ident = Some(__packet_macro_field);
                        __packet_macro_read_len
                    };
                }
            }
            Some(info @ VariableLenFieldInfo { is_optional: true, .. }) => {
                let is_present = info.len_from_bytes();
                let ty = &info.ty;
                quote! {
                    let __packet_macro_read_len: usize = if #is_present {
                            let __packet_macro_size = ::std::mem::size_of::<#ty>();
                            let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break #break_label
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &#ty =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            #ident = Some(__packet_macro_field);
                            __packet_macro_read_len
                        } else {
                            __packet_macro_read_len
                        };
                }
            }
            None => {
                quote! {
                    let __packet_macro_read_len: usize = {
                        let __packet_macro_size = ::std::mem::size_of::<#ty>();
                        let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                        if __packet_macro_bytes.len() < __packet_macro_size {
                            break #break_label
                        }
                        let (__packet_macro_field, __packet_macro_rem_bytes) =
                            __packet_macro_bytes.split_at(__packet_macro_size);
                        let __packet_macro_field: &#ty =
                            ::std::mem::transmute(__packet_macro_field.as_ptr());
                        __packet_macro_bytes = __packet_macro_rem_bytes;
                        #ident = Some(__packet_macro_field);
                        __packet_macro_read_len
                    };
                }
            }
        }
    }

    fn fill_vec(&self) -> TokenStream2 {
        if self.flatten {
            self.fill_vec_with_flatten()
        } else {
            self.fill_vec_with_field()
        }
    }

    fn fill_vec_with_flatten(&self) -> TokenStream2 {
        let ident = self.ident.as_ref().unwrap();
        quote! {
            #ident.fill_vec(__packet_macro_bytes);
        }
    }

    fn fill_vec_with_field(&self) -> TokenStream2 {
        let ident = self.ident.as_ref().unwrap();
        match &self.length_info {
            Some(info @ VariableLenFieldInfo { is_optional: false, .. }) => {
                let count = info.counter_expr();
                let ty = &info.ty;
                quote! {
                    unsafe {
                        let __packet_field_count = (#count) as usize;
                        let #ident = &#ident[..__packet_field_count];
                        let __packet_field_size =
                            ::std::mem::size_of::<#ty>() * __packet_field_count;
                        let __packet_field_field_bytes = #ident.as_ptr() as *const u8;
                        let __packet_field_field_slice =
                            ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
                        __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
                    }
                }
            }
            Some(info @ VariableLenFieldInfo { is_optional: true, .. }) => {
                let is_present = info.counter_expr();
                let ty = &info.ty;
                quote! {
                    unsafe {
                        if #is_present {
                            let #ident: &#ty = #ident.unwrap();
                            let __packet_field_size = ::std::mem::size_of::<#ty>();
                            let __packet_field_field_bytes = #ident as *const #ty as *const u8;
                            let __packet_field_field_slice =
                                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
                            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
                        }
                    }
                }
            }
            None => {
                let ty = &self.ty;
                quote! {
                    unsafe {
                        let __packet_field_size = ::std::mem::size_of::<#ty>();
                        let __packet_field_bytes = #ident as *const #ty as *const u8;
                        let __packet_field_slice =
                            ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                        __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                    }
                }
            }
        }
    }

    fn err_size(&self) -> TokenStream2 {
        if self.flatten {
            let ty = parser::as_turbofish(&self.ty);
            return quote! {
                #ty::min_len()
            };
        }

        match &self.length_info {
            Some(info @ VariableLenFieldInfo { is_optional: false, .. }) => {
                let count = info.err_size_expr();
                let ty = &info.ty;
                quote! {
                    (|| ::std::mem::size_of::<#ty>() * (#count) as usize)()
                }
            }
            Some(info @ VariableLenFieldInfo { is_optional: true, .. }) => {
                let is_present = info.err_size_expr();
                let ty = &info.ty;
                quote! {
                    (|| if #is_present { ::std::mem::size_of::<#ty>() } else { 0 })()
                }
            }
            None => {
                let ty = &self.ty;
                quote! { ::std::mem::size_of::<#ty>() }
            },
        }
    }

    fn exposed_ty(&self) -> TokenStream2 {
        match &self.length_info {
            None => {
                let nominal_ty = &self.ty;
                quote! { &'a #nominal_ty }
            },
            Some(VariableLenFieldInfo { is_optional: false, ty, .. }) => quote! { &'a [#ty] },
            Some(VariableLenFieldInfo { is_optional: true, ty, .. }) => {
                quote! { Option<&'a #ty> }
            },
        }
    }

    fn local_ty(&self) -> TokenStream2 {
        match &self.length_info {
            None => {
                let ty = &self.ty;
                if self.flatten {
                    quote! { Option<#ty> }
                } else {
                    quote! { Option<&#ty> }
                }
            },
            Some(VariableLenFieldInfo { is_optional: false, ty, .. }) => quote! { Option<&[#ty]> },
            Some(VariableLenFieldInfo { is_optional: true, ty, .. }) => {
                quote! { Option<&#ty> }
            },
        }
    }

    fn size_fn(&self) -> TokenStream2 {
        let ident = self.ident.as_ref().unwrap();
        if self.flatten {
            return quote_spanned!{self.span()=>
                #ident.len()
            };
        }
        match &self.length_info {
            Some(info @ VariableLenFieldInfo { is_optional: false, .. }) => {
                let ty = &info.ty;
                let count = info.counter_expr();
                quote! {
                    (::std::mem::size_of::<#ty>() * (#count) as usize)
                }
            }
            Some(info @ VariableLenFieldInfo { is_optional: true, .. }) => {
                let ty = &info.ty;
                let is_present = info.counter_expr();
                quote! {
                    (if #is_present {::std::mem::size_of::<#ty>() } else { 0 })
                }
            }
            None => {
                let nominal_ty = &self.ty;
                quote!( ::std::mem::size_of::<#nominal_ty>() )
            }
        }
    }

    fn declaration<'a, 'b: 'a>(
        &'b self,
        is_pub: bool,
        pf_attrs: impl Iterator<Item = &'a PerFieldsAttr> + 'a
    ) -> TokenStream2 {
        let name = self.ident.as_ref().unwrap();
        let attrs = self.attrs.iter();
        let pub_marker = is_pub.then(|| quote!{ pub });
        if self.flatten {
            let ty = &self.ty;
            quote! { #(#attrs)* #pub_marker #name: #ty, }
        } else {
            let ty = self.exposed_ty();
            let per_field_attrs = self.per_field_attrs(pf_attrs);
            quote! { #(#per_field_attrs)* #(#attrs)* #pub_marker #name: #ty, }
        }
    }

    fn per_field_attrs<'a, 'b: 'a>(
        &'b self,
        attrs: impl Iterator<Item = &'a PerFieldsAttr> + 'a,
    ) -> impl Iterator<Item = TokenStream2> + 'a {
        attrs.map(move |attr| match &self.length_info {
            None => {
                let attr = &attr.fixed;
                quote! { #attr }
            }
            Some(_) => match &attr.variable {
                Some(attr) => quote! { #attr },
                None => quote! {},
            },
        })
    }

    fn is_optional(&self) -> bool {
        matches!(self.length_info, Some(VariableLenFieldInfo { is_optional: true, ..}))
    }
}


#[proc_macro_derive(FlatSerializable)]
pub fn flat_serializable_derive(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let s = match &input.data {
        syn::Data::Enum(e) => {
            let has_repr = input.attrs.iter()
                .any(|attr| {
                    let meta = match attr.parse_meta() {
                        Ok(meta) => meta,
                        _ => return false,
                    };
                    meta.path().get_ident().map_or(false, |id| id == "repr")
                        && attr.parse_args().map_or(false, |ident: Ident|
                            ident == "u8" || ident == "u16" || ident == "u32" || ident == "u64"
                        )
                });
            if !has_repr {
                return quote_spanned! {e.enum_token.span()=>
                    compile_error!{"FlatSerializable only allowed on #[repr(u..)] enums without variants"}
                }.into()
            }
            let all_unit = e.variants.iter().all(|variant| matches!(variant.fields, syn::Fields::Unit));
            if !all_unit {
                return quote_spanned! {e.enum_token.span()=>
                    compile_error!{"FlatSerializable only allowed on until enums"}
                }.into()
            }
            return quote!{unsafe impl FlatSerializable for #name {}}.into()
        },
        syn::Data::Union(u) => return quote_spanned! {u.union_token.span()=>
            compile_error!("FlatSerializable not allowed on unions")
        }.into(),
        syn::Data::Struct(s) => s,

    };

    let alignment_checks = s.fields.iter().map(|f| {
        let ty = &f.ty;
        quote_spanned!{ty.span()=>
            let _alignment = [()][size % align_of::<#ty>()];
            let _internal_padding = [()][(size_of::<#ty>() < align_of::<#ty>()) as u8 as usize];
            size += size_of::<#ty>();
        }
    });
    let trait_checks = s.fields.iter().map(|f| {
        let ty = &f.ty;
        return quote_spanned!{ty.span()=>
            const _: () = {
                fn trait_check<T: FlatSerializable>() {}
                let _ = trait_check::<#ty>;
            };
        }
    });

    let out = quote! {
        const _:() = {
            use std::mem::{size_of, align_of};
            let mut size = 0;
            #(#alignment_checks)*
            let _trailing_padding = [()][(size_of::<#name>() != size) as u8 as usize];
        };
        const _:() = {
            #(#trait_checks)*
        };
        unsafe impl FlatSerializable for #name {}
    };
    out.into()
}
