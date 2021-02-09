use std::collections::HashSet;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};

use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token,
    visit::Visit,
    visit_mut::VisitMut,
    Attribute, Expr, Field, Ident, Result, Token, Type,
};

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

fn flat_serialize_struct(input: FlatSerializeStruct) -> TokenStream2 {
    let ident = input.ident.clone();

    let (field_paths, use_trait) = input.metadata();

    let ref_def = {
        let alignment_check = input.alignment_check(quote!(0), &field_paths, &use_trait);
        let try_ref = input.fn_try_ref(&field_paths, &use_trait);
        let fill_vec = input.fn_fill_vec(&field_paths, &use_trait);
        let len = input.fn_len(&field_paths, &use_trait);
        let fields: Vec<_> = input
            .fields
            .iter()
            .enumerate()
            .flat_map(|(i, f)| {
                let name = f.ident.as_ref().unwrap();
                let attrs = filtered_attrs(f.attrs.iter());
                if use_trait.contains(&i) {
                    let ty = &f.ty;
                    Some(quote! { #(#attrs)* pub #name: #ty, })
                } else {
                    let ty = exposed_ty(&field_paths[i], &f.ty);
                    Some(quote! { #(#attrs)* pub #name: &'a #ty, })
                }
            })
            .collect();
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

struct FindSelf(bool);

impl<'ast> Visit<'ast> for FindSelf {
    fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
        self.0 |= i.ident == "self"
    }
}

struct GetLenField(Option<Ident>);

impl<'ast> Visit<'ast> for GetLenField {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        if let syn::Expr::Field(field) = expr {
            if let syn::Expr::Path(path) = &*field.base {
                if path.path.segments[0].ident == "self" {
                    let name = match &field.member {
                        syn::Member::Named(name) => name.clone(),
                        syn::Member::Unnamed(_) => panic!("unnamed fields not supported"),
                    };
                    self.0 = Some(name)
                }
            }
        }
    }
}

struct RemoveSelf(bool, Option<Ident>);

impl VisitMut for RemoveSelf {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if let syn::Expr::Field(field) = expr {
            if let syn::Expr::Path(path) = &mut *field.base {
                if path.path.segments[0].ident == "self" {
                    let name = match &field.member {
                        syn::Member::Named(name) => name.clone(),
                        syn::Member::Unnamed(_) => panic!("unnamed fields not supported"),
                    };
                    if self.0 {
                        assert_eq!(self.1, None);
                        self.1 = Some(name.clone())
                    }
                    *expr = syn::Expr::Path(syn::ExprPath {
                        attrs: Default::default(),
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: Some::<syn::PathSegment>(name.into()).into_iter().collect(),
                        },
                    })
                }
            }
        } else {
            self.0 = false;
            syn::visit_mut::visit_expr_mut(self, expr)
        }
    }
}

enum FlatSerialize {
    Struct(FlatSerializeStruct),
    Enum(FlatSerializeEnum),
}

struct FlatSerializeStruct {
    attrs: Vec<Attribute>,
    ident: Ident,
    fields: Punctuated<Field, Token![,]>,
}

struct FlatSerializeEnum {
    attrs: Vec<Attribute>,
    ident: Ident,
    tag: Field,
    variants: Punctuated<FlatSerializeVariant, Token![,]>,
}

#[allow(dead_code)]
struct FlatSerializeVariant {
    tag_val: Expr,
    body: FlatSerializeStruct,
}

impl Parse for FlatSerialize {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        //TODO Visibility
        if lookahead.peek(Token![struct]) {
            input.parse().map(|mut s: FlatSerializeStruct| {
                s.attrs = attrs;
                FlatSerialize::Struct(s)
            })
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(|mut e: FlatSerializeEnum| {
                e.attrs = attrs;
                FlatSerialize::Enum(e)
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for FlatSerializeStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _struct_token: Token![struct] = input.parse()?;
        let ident = input.parse()?;
        let _brace_token: token::Brace = braced!(content in input);
        let fields = content.parse_terminated(Field::parse_named)?;
        Ok(Self {
            attrs: vec![],
            ident,
            fields,
        })
    }
}

impl Parse for FlatSerializeEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _enum_token: Token![enum] = input.parse()?;
        let ident = input.parse()?;
        let _brace_token: token::Brace = braced!(content in input);
        let tag = Field::parse_named(&content)?;
        let _comma_token: Token![,] = content.parse()?;
        let variants = content.parse_terminated(FlatSerializeVariant::parse)?;
        Ok(Self {
            attrs: vec![],
            ident,
            tag,
            variants,
        })
    }
}

impl Parse for FlatSerializeVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let ident = input.parse()?;
        let _colon_token: Token![:] = input.parse()?;
        let tag_val = input.parse()?;
        let _brace_token: token::Brace = braced!(content in input);
        let fields = content.parse_terminated(Field::parse_named)?;
        Ok(Self {
            tag_val,
            body: FlatSerializeStruct {
                attrs: vec![],
                ident,
                fields,
            },
        })
    }
}

struct ExternalLenFieldInfo {
    ty: syn::Type,
    len_expr: syn::Expr,
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

type Metadata = (Vec<Option<ExternalLenFieldInfo>>, HashSet<usize>);

impl FlatSerializeEnum {
    fn variants(&self) -> TokenStream2 {
        let id = &self.ident;
        let variants = self.variants.iter().map(|variant| {
            let fields = variant.body.fields.iter().enumerate().flat_map(|(i, f)| {
                let (field_paths, use_trait) = variant.body.metadata();
                let name = f.ident.as_ref().unwrap();
                let attrs = filtered_attrs(f.attrs.iter());
                if use_trait.contains(&i) {
                    let ty = &f.ty;
                    Some(quote! { #(#attrs)* #name: #ty, })
                } else {
                    let ty = exposed_ty(&field_paths[i], &f.ty);
                    Some(quote! { #(#attrs)* #name: &'a #ty, })
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
        let tag_ty = &self.tag.ty;
        let tag_ident = self.tag.ident.as_ref().unwrap();
        let break_label = syn::Lifetime::new("'tryref_tag", proc_macro2::Span::call_site());
        let try_wrap_tag = try_wrap_field(tag_ident, tag_ty, &break_label, &None, false);
        let id = &self.ident;

        let bodies = self.variants.iter().enumerate().map(|(i, v)| {
            let tag_val = &v.tag_val;

            let variant = &v.body.ident;

            let break_label =
                syn::Lifetime::new(&format!("'tryref_{}", i), proc_macro2::Span::call_site());
            let (field_paths, use_trait) = v.body.metadata();
            let TryRefBody {
                vars,
                body,
                set_fields,
                err_size,
            } = v
                .body
                .fn_try_ref_body(&field_paths, &use_trait, &break_label);

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
        let fill_vec_tag = fill_vec_with_field(&self.tag, &None);
        let id = &self.ident;
        let bodies = self.variants.iter().map(|v| {
            let tag_val = &v.tag_val;
            let variant = &v.body.ident;
            let (field_paths, use_trait) = v.body.metadata();

            let (fields, fill_vec_with) = v.body.fill_vec_body(&field_paths, &use_trait);
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
            let (field_paths, use_trait) = v.body.metadata();

            let size = v
                .body
                .fields
                .iter()
                .enumerate()
                .map(|(i, f)| size_fn(&field_paths[i], use_trait.contains(&i), f));
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
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        if !use_trait.is_empty() {
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

        let checks = self.fields.iter().enumerate().map(|(i, f)| {
            use syn::spanned::Spanned;
            use std::mem::replace;
            match &field_paths[i] {
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
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        let break_label = syn::Lifetime::new("'tryref", proc_macro2::Span::call_site());
        let id = &self.ident;
        let TryRefBody {
            vars,
            body,
            set_fields,
            err_size,
        } = self.fn_try_ref_body(field_paths, &use_trait, &break_label);
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
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
        break_label: &syn::Lifetime,
    ) -> TryRefBody {
        let (field, ty): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (&f.ident, exposed_ty(&field_paths[i], &f.ty)))
            .unzip();
        let (field1, ty1) = (
            field.iter(),
            ty.iter().enumerate().map(|(i, ty)| {
                if use_trait.contains(&i) {
                    quote! { Option<#ty> }
                } else {
                    quote! { Option<&#ty> }
                }
            }),
        );
        let field2 = field.iter();
        let field3 = field.iter();

        let vars = quote!( #(let mut #field1: #ty1 = None;)* );
        let try_wrap_fields = self.fields.iter().enumerate().map(|(i, f)| {
            try_wrap_field(
                &f.ident.as_ref().unwrap(),
                &f.ty,
                break_label,
                &field_paths[i],
                use_trait.contains(&i),
            )
        });
        let body = quote! ( #(#try_wrap_fields)* );

        let set_fields = quote!( #(#field2: #field3.unwrap()),* );

        let err_size = ty
            .iter()
            .enumerate()
            .map(|(i, t)| err_size(t, &field_paths[i], use_trait.contains(&i)));
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
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        let id = &self.ident;
        let (fields, fill_vec_with) = self.fill_vec_body(field_paths, use_trait);
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
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> (TokenStream2, TokenStream2) {
        //FIXME assert multiple values of counters are equal...
        let fill_vec_with = self.fields.iter().enumerate().map(|(i, f)| {
            if use_trait.contains(&i) {
                return fill_vec_with_trait(f);
            }
            fill_vec_with_field(f, &field_paths[i])
        });
        let fill_vec_with = quote!( #(#fill_vec_with);* );

        let field = self.fields.iter().map(|f| f.ident.as_ref().unwrap());
        let fields = quote!( #(#field),* );
        (fields, fill_vec_with)
    }

    fn fn_len(
        &self,
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        let size = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| size_fn(&field_paths[i], use_trait.contains(&i), f));
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

    fn metadata(&self) -> Metadata {
        // Set of field-indexes of fields that use trait-based serialization
        let mut use_trait = HashSet::new();

        // Info on how to determine the lengths of variable-length fields
        let field_paths: Vec<Option<ExternalLenFieldInfo>> = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let crate_name = quote::format_ident!("flat_serialize");
                let att_name = quote::format_ident!("flatten");
                let path = syn::parse_quote! { #crate_name :: #att_name };
                let uses_trait = f.attrs.iter().any(|att| att.path == path);
                if uses_trait {
                    use_trait.insert(i);
                    return None;
                }
                if let syn::Type::Array(array) = &f.ty {
                    let mut has_self = FindSelf(false);
                    has_self.visit_expr(&array.len);
                    let FindSelf(has_self) = has_self;
                    // println!("{} | {}", quote!{ #f }, has_self);
                    if has_self {
                        let mut len_field = GetLenField(None);
                        len_field.visit_expr(&array.len);
                        // let mut len = array.len.clone();
                        // let mut remove_self = RemoveSelf(true, None);
                        // remove_self.visit_expr_mut(&mut len);
                        return Some(ExternalLenFieldInfo {
                            ty: (*array.elem).clone(),
                            len_expr: array.len.clone(),
                        });
                    }
                }
                None
            })
            .collect();
        (field_paths, use_trait)
    }
}

fn try_wrap_field(
    field: &Ident,
    ty: &Type,
    break_label: &syn::Lifetime,
    info: &Option<ExternalLenFieldInfo>,
    use_trait: bool,
) -> TokenStream2 {
    if use_trait {
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
                #field = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
        };
    }
    match info {
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
                    #field = Some(__packet_macro_field);
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
                    #field = Some(__packet_macro_field);
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

fn fill_vec_with_field(field: &Field, info: &Option<ExternalLenFieldInfo>) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    match info {
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

fn err_size(
    ty: &TokenStream2,
    info: &Option<ExternalLenFieldInfo>,
    use_trait: bool,
) -> TokenStream2 {
    if use_trait {
        return quote! {
            <#ty as FlattenableRef>::min_len()
        };
    }
    match info {
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

fn exposed_ty(info: &Option<ExternalLenFieldInfo>, nominal_ty: &Type) -> TokenStream2 {
    match info {
        None => quote! { #nominal_ty },
        Some(ExternalLenFieldInfo { ty, .. }) => quote! { [#ty] },
    }
}

fn size_fn(info: &Option<ExternalLenFieldInfo>, use_trait: bool, field: &Field) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let nominal_ty = &field.ty;
    if use_trait {
        return quote! {
            <#nominal_ty as FlattenableRef>::len(&#ident)
        };
    }
    match info {
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

fn filtered_attrs<'a>(attrs: impl Iterator<Item=&'a Attribute>) -> impl Iterator<Item=&'a Attribute> {
    let crate_name = quote::format_ident!("flat_serialize");
    let att_name = quote::format_ident!("flatten");
    let path = syn::parse_quote! { #crate_name :: #att_name };
    attrs.filter(move |a| a.path != path)
}