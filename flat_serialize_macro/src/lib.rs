
use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};

use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    visit_mut::VisitMut,
    Attribute, Expr, Field, Ident, Token,
};

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
    /// use the FlattenableRef trait for (de)serialization
    use_trait: bool,
    // TODO is this mutually exclusive with `use_trait` above? Should we make an
    // enum to select between them?
    length_info: Option<ExternalLenFieldInfo>,
}

/// a `#[flat_serialize::field_attr(fixed = "#[foo]", variable = "#[bar]"))]`
/// attribute. The inner attribute(s) will be applied to each relevant field.
struct PerFieldsAttr {
    fixed: Attribute,
    variable: Option<Attribute>,
}

/// how to find the length of a variable-length field.
struct ExternalLenFieldInfo {
    ty: syn::Type,
    len_expr: syn::Expr,
}

fn flat_serialize_struct(input: FlatSerializeStruct) -> TokenStream2 {
    let ident = input.ident.clone();

    let ref_def = {
        let alignment_check = input.alignment_check(quote!(0));
        let try_ref = input.fn_try_ref();
        let fill_vec = input.fn_fill_vec();
        let len = input.fn_len();
        let fields = input
            .fields
            .iter()
            .flat_map(|f| {
                let name = f.ident.as_ref().unwrap();
                let attrs = f.attrs.iter();
                if f.use_trait {
                    let ty = &f.ty;
                    Some(quote! { #(#attrs)* pub #name: #ty, })
                } else {
                    let ty = exposed_ty(&f);
                    let per_field_attrs =
                        per_field_attrs(&f.length_info, input.per_field_attrs.iter());
                    Some(quote! { #(#per_field_attrs)* #(#attrs)* pub #name: &'a #ty, })
                }
            });
        let attrs = &*input.attrs;

        quote! {
            #[derive(Copy, Clone)]
            #(#attrs)*
            pub struct #ident<'a> {
                #(#fields)*
            }

            #alignment_check

            impl<'a> #ident<'a> {
                #try_ref

                #fill_vec

                #len
            }

            impl<'a> FlattenableRef<'a> for #ident<'a> {
                unsafe fn try_ref(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr>
                where Self: Sized + 'a {
                    #ident::try_ref(bytes)
                }
                fn fill_vec(&self, vec: &mut Vec<u8>) {
                    #ident::fill_vec(self, vec)
                }
                fn len(&self) -> usize {
                    #ident::len(self)
                }
            }
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

        impl<'a> #ident<'a> {
            #try_ref

            #fill_vec

            #len
        }

        impl<'a> FlattenableRef<'a> for #ident<'a> {
            unsafe fn try_ref(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr>
            where Self: Sized + 'a {
                #ident::try_ref(bytes)
            }
            fn fill_vec(&self, vec: &mut Vec<u8>) {
                #ident::fill_vec(self, vec)
            }
            fn len(&self) -> usize {
                #ident::len(self)
            }
        }
    }
}

impl ExternalLenFieldInfo {
    fn len_from_bytes(&self) -> TokenStream2 {
        let mut lfb = SelfReplacer(|name| syn::parse_quote! { #name.cloned().unwrap() as usize });
        let mut len = self.len_expr.clone();
        lfb.visit_expr_mut(&mut len);
        quote! { #len }
    }

    fn counter_expr(&self) -> TokenStream2 {
        let mut ce = SelfReplacer(|name| syn::parse_quote! { (*#name) as usize });
        let mut len = self.len_expr.clone();
        ce.visit_expr_mut(&mut len);
        quote! { #len }
    }

    fn err_size_expr(&self) -> TokenStream2 {
        let mut ese = SelfReplacer(|name| {
            syn::parse_quote! {
                (#name.as_ref().map(|c| **c).unwrap_or(0)) as usize
            }
        });
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
            let fields = variant.body.fields.iter().flat_map(|f| {
                let name = f.ident.as_ref().unwrap();
                let attrs = f.attrs.iter();
                if f.use_trait {
                    let ty = &f.ty;
                    Some(quote! { #(#attrs)* #name: #ty, })
                } else {
                    let ty = exposed_ty(&f);
                    let per_field_attrs =
                        per_field_attrs(&f.length_info, self.per_field_attrs.iter());
                    Some(quote! { #(#per_field_attrs)* #(#attrs)* #name: &'a #ty, })
                }
            });
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
    fn fn_try_ref(&self) -> TokenStream2 {
        let break_label = syn::Lifetime::new("'tryref_tag", proc_macro2::Span::call_site());
        let try_wrap_tag = try_wrap_field(&self.tag, &break_label);
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
        let fill_vec_tag = fill_vec_with_field(&self.tag);
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
                .map(size_fn);
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
    ) -> TokenStream2 {
        // TODO
        if self.fields.iter().any(|f| f.use_trait) {
            //TODO
            return quote! {};
        }

        // we create the size/align values as ever-increasing expressions
        // instead of variables due to the way span info works: we want to use
        // the span of the inputted type so that errors are reported only at the
        // type that causes the misalignment, however, if the type is inputted
        // via a macro, that span will be unable to reference local variables
        // we create ourselves.
        let mut size = quote! { #start };
        let mut min_align = quote! { 8 };

        let checks = self.fields.iter().map(|f| {
            use syn::spanned::Spanned;
            use std::mem::replace;
            match &f.length_info {
                None => {
                    let ty = &f.ty;
                    let new_size = quote!{#size + size_of::<#ty>()};
                    let size = replace(&mut size, new_size);
                    quote_spanned!{f.ty.span()=>
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
                    let min_align = replace(&mut min_align, new_min_align);
                    quote_spanned!{f.ty.span()=>
                        let _alignment_check: () = [()][(#size) % align_of::<#ty>()];
                        let _alignment_check2: () = [()][(align_of::<#ty>() > #min_align) as u8 as usize];
                        let _padding_check: () = [()][(size_of::<#ty>() < align_of::<#ty>()) as u8 as usize];
                    }
                }
            }
        });

        quote! {
            // alignment assertions
            const _: () = {
                use std::mem::{align_of, size_of};
                #(#checks)*
            };
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
            .map(|f| {
                let ty = exposed_ty(&f);
                if f.use_trait {
                    quote! { Option<#ty> }
                } else {
                    quote! { Option<&#ty> }
                }
            });
        let field1 = field_names.clone();
        let field2 = field_names.clone();
        let field3 = field_names;

        let vars = quote!( #(let mut #field1: #ty1 = None;)* );
        let try_wrap_fields = self.fields.iter().map(|f| {
            try_wrap_field(
                f,
                break_label,
            )
        });
        let body = quote! ( #(#try_wrap_fields)* );

        let set_fields = quote!( #(#field2: #field3.unwrap()),* );

        let err_size = self
        .fields
            .iter()
            .map(err_size);
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
        let fill_vec_with = self.fields.iter().map(|f| {
            if f.use_trait {
                return fill_vec_with_trait(f);
            }
            fill_vec_with_field(f)
        });
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
            .map(size_fn);
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

fn try_wrap_field(
    field: &FlatSerializeField,
    break_label: &syn::Lifetime,
) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;

    if field.use_trait {
        return quote! {
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) = match <#ty as FlattenableRef>::try_ref(__packet_macro_bytes) {
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
    match &field.length_info {
        Some(info @ ExternalLenFieldInfo { .. }) => {
            let count = info.len_from_bytes();
            let ty = &info.ty;
            quote! {
                let __packet_macro_read_len: usize = {
                    let __packet_macro_count = #count;
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

fn fill_vec_with_trait(field: &Field) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        <#ty as FlattenableRef>::fill_vec(&#ident, __packet_macro_bytes);
    }
}

// TODO take FlatSerializeField instead
fn fill_vec_with_field(field: &FlatSerializeField) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    match &field.length_info {
        Some(info) => {
            let count = info.counter_expr();
            let ty = &info.ty;
            quote! {
                unsafe {
                    let __packet_field_count = #count;
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
        None => {
            let ty = &field.ty;
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

fn err_size(field: &FlatSerializeField) -> TokenStream2 {
    let ty = exposed_ty(field);
    if field.use_trait {
        return quote! {
            <#ty as FlattenableRef>::min_len()
        };
    }
    match &field.length_info {
        Some(info @ ExternalLenFieldInfo { .. }) => {
            let count = info.err_size_expr();
            let ty = &info.ty;
            quote! {
                (::std::mem::size_of::<#ty>()
                    * (#count))
            }
        }
        None => quote! { ::std::mem::size_of::<#ty>() },
    }
}

fn exposed_ty(field: &FlatSerializeField) -> TokenStream2 {
    match &field.length_info {
        None => {
            let nominal_ty = &field.ty;
            quote! { #nominal_ty }
        },
        Some(ExternalLenFieldInfo { ty, .. }) => quote! { [#ty] },
    }
}

fn size_fn(field: &FlatSerializeField) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let nominal_ty = &field.ty;
    if field.use_trait {
        return quote! {
            <#nominal_ty as FlattenableRef>::len(&#ident)
        };
    }
    match &field.length_info {
        Some(info) => {
            let ty = &info.ty;
            let count = info.counter_expr();
            quote! {
                (::std::mem::size_of::<#ty>() * (#count))
            }
        }
        None => {
            quote!( ::std::mem::size_of::<#nominal_ty>() )
        }
    }
}


fn per_field_attrs<'a>(
    field_info: &'a Option<ExternalLenFieldInfo>,
    attrs: impl Iterator<Item = &'a PerFieldsAttr> + 'a,
) -> impl Iterator<Item = TokenStream2> + 'a {
    attrs.map(move |attr| match field_info {
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
