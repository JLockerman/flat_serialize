
use std::collections::{HashMap, HashSet};

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};

use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;

#[proc_macro]
pub fn flat_serialize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as FlatSerializeStruct);
    let ident = input.ident.clone();

    let mut size_fields = HashMap::new();
    let mut use_trait = HashSet::new();

    let field_paths: Vec<Option<ExternalLenFieldInfo>> = input.fields.iter()
        .enumerate()
        .map(|(i, f)| {
            let crate_name = quote::format_ident!("flat_serialize");
            let att_name = quote::format_ident!("use_trait");
            let path = syn::parse_quote!{ #crate_name :: #att_name };
            let uses_trait = f.attrs.iter().any(|att| att.path == path);
            if uses_trait {
                use_trait.insert(i);
                return None
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
                    if let Some(name) = len_field.0.clone() {
                        size_fields.entry(name)
                            .or_insert(f.ident.clone().unwrap());
                    }
                    return Some(ExternalLenFieldInfo {
                        ty: (*array.elem).clone(),
                        len_expr: array.len.clone(),
                    })
                }
            }
            None
        })
        .collect();

    let ref_def = {
        let try_ref = input.fn_try_ref(&size_fields, &field_paths, &use_trait);
        let fill_vec = input.fn_fill_vec(&size_fields, &field_paths, &use_trait);
        let len = input.fn_len(&size_fields, &field_paths, &use_trait);
        let fields: Vec<_> = input.fields.iter().enumerate().flat_map(|(i, f)| {
            let name = f.ident.as_ref().unwrap();
            if size_fields.contains_key(name) {
                return None
            }
            let name = f.ident.as_ref().unwrap();
            if use_trait.contains(&i) {
                let ty = &f.ty;
                Some(quote! { pub #name: #ty, })
            } else {
                let ty = exposed_ty(&field_paths[i], &f.ty);
                Some(quote! { pub #name: &'a #ty, })
            }
        }).collect();
        quote! {
            #[derive(Copy, Clone)]
            pub struct Ref<'a> {
                #(#fields)*
            }

            impl<'a> Ref<'a> {
                #try_ref

                #fill_vec

                #len
            }

            impl<'a> FlatSerialize<'a> for Ref<'a> {
                unsafe fn try_ref(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr>
                where Self: Sized + 'a {
                    Ref::try_ref(bytes)
                }
                fn fill_vec(&self, vec: &mut Vec<u8>) {
                    Ref::fill_vec(self, vec)
                }
                fn len(&self) -> usize {
                    Ref::len(self)
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
        #[allow(non_snake_case)]
        mod #ident {
            use super::*;

            #ref_def

            // #mut_def

            pub unsafe fn try_ref(mut __packet_macro_bytes: &[u8]) -> Result<(Ref<'_>, &[u8]), WrapErr> {
                Ref::try_ref(__packet_macro_bytes)
            }
        }
    };

    // only here for debugging
    // println!("{}", expanded.to_string());

    expanded.into()
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
                    *expr = syn::Expr::Path(syn::ExprPath{
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

// TODO
#[allow(dead_code)]
enum FlatSerialize {
    Struct(FlatSerializeStruct),
    Enum,
}

#[allow(dead_code)]
struct FlatSerializeStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,

}

impl Parse for FlatSerialize {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        //TODO Visibility
        if lookahead.peek(Token![struct]) {
            input.parse().map(FlatSerialize::Struct)
        } else if lookahead.peek(Token![enum]) {
            todo!()
            // input.parse().map(FlatSerialize::Enum)
        } else {
            Err(lookahead.error())
        }
    }

}

impl Parse for FlatSerializeStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let struct_token = input.parse()?;
        let ident = input.parse()?;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(Field::parse_named)?;
        Ok(Self {
            struct_token,
            ident,
            brace_token,
            fields,
        })
    }
}

struct ExternalLenFieldInfo {
    ty: syn::Type,
    len_expr: syn::Expr,
}

impl ExternalLenFieldInfo {
    fn len_from_bytes(&self) -> TokenStream2 {
        let mut lfb = SelfReplacer(|name| syn::parse_quote!{ #name.cloned().unwrap() as usize });
        let mut len = self.len_expr.clone();
        lfb.visit_expr_mut(&mut len);
        quote! { #len }
    }

    fn counter_expr(&self) -> TokenStream2 {
        let mut ce = SelfReplacer(|name| syn::parse_quote!{ (*#name) as usize });
        let mut len = self.len_expr.clone();
        ce.visit_expr_mut(&mut len);
        quote! { #len }
    }

    fn err_size_expr(&self) -> TokenStream2 {
        let mut ese = SelfReplacer(|name| syn::parse_quote!{
            (#name.as_ref().map(|c| **c).unwrap_or(0)) as usize
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

impl FlatSerializeStruct {
    fn fn_try_ref(
        &self,
        size_fields: &HashMap<Ident, Ident>,
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        let break_label = syn::Lifetime::new("'tryref", proc_macro2::Span::call_site());
        let TryRefBody{vars, body, set_fields, err_size } =
            self.fn_try_ref_body(size_fields, field_paths, &use_trait, &break_label);
        quote!{
            #[allow(unused_assignments, unused_variables)]
            #[inline(always)]
            pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
                let __packet_macro_read_len = 0usize;
                #vars
                #break_label: loop {
                    #body
                    let _ref = Ref { #set_fields };
                    return Ok((_ref, __packet_macro_bytes))
                }
                Err(WrapErr::NotEnoughBytes(0 #err_size))
            }
        }
    }
    fn fn_try_ref_body(
        &self,
        size_fields: &HashMap<Ident, Ident>,
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
        break_label: &syn::Lifetime,
    ) -> TryRefBody {
        let (field, ty): (Vec<_>, Vec<_>) = self.fields.iter().enumerate().map(|(i, f)| {
            (&f.ident, exposed_ty(&field_paths[i], &f.ty))
        }).unzip();
        let (field1, ty1) = (field.iter(), ty.iter().enumerate().map(|(i, ty)| {
            if use_trait.contains(&i) {
                quote! { Option<#ty> }
            } else {
                quote! { Option<&#ty> }
            }
        }));
        let field2 = field.iter().filter(|f| !size_fields.contains_key(f.as_ref().unwrap()));
        let field3 = field.iter().filter(|f| !size_fields.contains_key(f.as_ref().unwrap()));

        let vars = quote!( #(let mut #field1: #ty1 = None;)* );
        let try_wrap_fields = self.fields.iter().enumerate().map(|(i, f)|
            try_wrap_field(&f.ident.as_ref().unwrap(), &f.ty, break_label, &field_paths[i], use_trait.contains(&i)));
        let body = quote! ( #(#try_wrap_fields)* );

        let set_fields = quote!( #(#field2: #field3.unwrap()),* );

        let err_size = ty.iter().enumerate().map(|(i, t)| err_size(t, &field_paths[i], use_trait.contains(&i)));
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
        size_fields: &HashMap<Ident, Ident>,
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        let (fields, counters, fill_vec_with) = self.fill_vec_body(size_fields, field_paths, use_trait);
        quote!{
            #[allow(unused_assignments, unused_variables)]
            pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
                __packet_macro_bytes.reserve_exact(self.len());
                let &Ref { #fields } = self;
                #counters
                #fill_vec_with
            }
        }
    }
    fn fill_vec_body(
        &self,
        size_fields: &HashMap<Ident, Ident>,
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> (TokenStream2, TokenStream2, TokenStream2) {
        let counters = size_fields.iter().map(|(s, i)| quote!( let #s = &#i.len(); ));
        let counters = quote!( #(#counters);* );

        //FIXME assert multiple values of counters are equal...
        let fill_vec_with = self.fields.iter().enumerate()
            .map(|(i, f)| {
                if use_trait.contains(&i) {
                    return fill_vec_with_trait(f)
                }
                match size_fields.contains_key(f.ident.as_ref().unwrap()) {
                    true => fill_vec_with_counter(f, &field_paths[i]),
                    false => fill_vec_with_field(f, &field_paths[i]),
                }
            });
        let fill_vec_with = quote!( #(#fill_vec_with);* );

        let field = self.fields.iter()
            .filter(|f| !size_fields.contains_key(f.ident.as_ref().unwrap()))
            .map(|f| f.ident.as_ref().unwrap());
        let fields = quote!( #(#field),* );
        (fields, counters, fill_vec_with)
    }

    fn fn_len(
        &self,
        size_fields: &HashMap<Ident, Ident>,
        field_paths: &[Option<ExternalLenFieldInfo>],
        use_trait: &HashSet<usize>,
    ) -> TokenStream2 {
        let counters = size_fields.iter().map(|(s, i)| quote!( let #s = &#i.len(); ));
        let size = self.fields.iter().enumerate().map(|(i, f)|
            size_fn(&field_paths[i], use_trait.contains(&i), f));
        let field = self.fields.iter()
            .map(|f| f.ident.as_ref().unwrap())
            .filter(|f| !size_fields.contains_key(f));

        quote!{
            #[allow(unused_assignments, unused_variables)]
            pub fn len(&self) -> usize {
                let &Ref { #(#field),* } = self;
                #(#counters);*
                0usize #(+ #size)*
            }
        }
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
                let (__packet_macro_field, __packet_macro_rem_bytes) = match <#ty as FlatSerialize>::try_ref(__packet_macro_bytes) {
                    Ok((f, b)) => (f, b),
                    Err(..) => break #break_label
                };
                let __packet_macro_size = __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                #field = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
        }
    }
    match info {
        Some(info @ ExternalLenFieldInfo{..}) => {
            let count = info.len_from_bytes();
            let ty = &info.ty;
            quote!{
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
        },
        None => {
            quote!{
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
        },
    }
}

fn fill_vec_with_trait(field: &Field) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        <#ty as FlatSerialize>::fill_vec(&#ident, __packet_macro_bytes);
    }
}

fn fill_vec_with_counter(field: &Field, info: &Option<ExternalLenFieldInfo>) -> TokenStream2 {
    if let Some(..) = info {
        let span = field.ident.as_ref().unwrap().span().unwrap();
        return quote_spanned! { span.into()=> compile_error!("cannot use as length")}
    }
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<#ty>();
            let __packet_field_bytes = (*#ident) as #ty;
            let __packet_field_bytes = (&__packet_field_bytes)
                as *const #ty as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
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

fn err_size(ty: &TokenStream2, info: &Option<ExternalLenFieldInfo>, use_trait: bool) -> TokenStream2 {
    if use_trait {
        return quote! {
            <#ty as FlatSerialize>::min_len()
        }
    }
    match info {
        Some(info @ ExternalLenFieldInfo{..}) => {
            let count = info.err_size_expr();
            let ty = &info.ty;
            quote! {
                (::std::mem::size_of::<#ty>()
                    * (#count))
            }
        },
        None => quote!{ ::std::mem::size_of::<#ty>() },
    }
}

fn exposed_ty(info: &Option<ExternalLenFieldInfo>, nominal_ty: &Type) -> TokenStream2 {
    match info {
        None => quote! { #nominal_ty },
        Some(ExternalLenFieldInfo{ty, ..}) => quote! { [#ty] }
    }
}

fn size_fn(
    info: &Option<ExternalLenFieldInfo>,
    use_trait: bool,
    field: &Field,
) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let nominal_ty = &field.ty;
    if use_trait {
        return quote! {
            <#nominal_ty as FlatSerialize>::len(&#ident)
        }
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
