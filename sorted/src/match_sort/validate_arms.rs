
use std::ops::{AddAssign};

use proc_macro2::{Span};
use syn::{Arm, Error, Pat, PatPath, PatTupleStruct, Path, Ident, Result, PatIdent, spanned::Spanned, punctuated::Punctuated, token::Colon2, PathSegment};

/// Validate Arm Type
#[derive(Copy, Clone)]
enum ArmTypes {
    Path,
    TupleStruct,
    Ident,
    Wild,
    Empty
}

impl ArmTypes {

    /// returns the valid pattern types
    /// returns Some(ArmType) if one of valid options
    /// returns None if not valid type
    fn pat_type(pat : &Pat) -> Option<Self> {

        if let Pat::Path(_) = pat {
            return Some(ArmTypes::Path);
        }

        if let Pat::TupleStruct(_) = pat {
            return Some(ArmTypes::TupleStruct);
        }

        if let Pat::Ident(_) = pat {
            return Some(ArmTypes::Ident)
        }

        if let Pat::Wild(_) = pat {
            return Some(ArmTypes::Wild)
        }

        None
    }

    /// my version of equality for ArmTypes warning it is not transitive
    fn my_eq(&self, other: &Self) -> bool {
        /*match (self, other) {
            (ArmTypes::Empty, ArmTypes::Empty) => true,
            (ArmTypes::Path, ArmTypes::Path) | (ArmTypes::Path, ArmTypes::Wild) | (ArmTypes::Wild,ArmTypes::Path) => true,
            (ArmTypes::TupleStruct, ArmTypes::TupleStruct) | (ArmTypes::TupleStruct, ArmTypes::Wild) | (ArmTypes::Wild,ArmTypes::TupleStruct) => true,
            (ArmTypes::Struct, ArmTypes::Struct) | (ArmTypes::Struct, ArmTypes::Wild) | (ArmTypes::Wild, ArmTypes::Struct) => true,
            (ArmTypes::Wild, ArmTypes::Wild) => true,
            _ => false
        }*/
        matches!((self, other),
        (ArmTypes::Empty, ArmTypes::Empty) |
        (ArmTypes::Path, ArmTypes::Path) | (ArmTypes::Path, ArmTypes::Wild) | (ArmTypes::Wild,ArmTypes::Path) |
        (ArmTypes::TupleStruct, ArmTypes::TupleStruct) | (ArmTypes::TupleStruct, ArmTypes::Wild) | (ArmTypes::Wild,ArmTypes::TupleStruct) |
        (ArmTypes::Ident, ArmTypes::Ident) | (ArmTypes::Ident, ArmTypes::Wild) | (ArmTypes::Wild, ArmTypes::Ident) |
        (ArmTypes::Wild, ArmTypes::Wild))
    }
}



/// struct contains validated arms
pub struct ValidatedArms<'a> {
    arm_type: ArmTypes,
    arms: &'a [Arm]
}

impl<'a> ValidatedArms<'a> {

    /// Checks if arms are all of the correct type
    pub fn validate_arms(arms : &'a [Arm]) -> Result<ValidatedArms<'a>> {

        // check if no match arms
        if arms.is_empty() {
            let empty_match = ValidatedArms{arm_type: ArmTypes::Empty, arms};
            return Ok(empty_match);
        }

        let mut arms_iter = arms.iter();

        // get first arm
        let first_arm = arms_iter.next().unwrap();

        let match_pattern = ArmTypes::pat_type(&first_arm.pat);

        // if invalid pattern then return appropriate error
        if match_pattern.is_none() {
            let err_span= get_pattern_span(&first_arm.pat);

            let err_msg = "unsupported by #[sorted]";
            let err = Error::new(err_span, err_msg);
            return Err(err)
        }

        let mut match_pattern = match_pattern.unwrap();

        for arm in arms_iter {

            // get current arm type
            let cur_arm_type = ArmTypes::pat_type(&arm.pat);

            // if invalid pattern then return appropriate error
            if cur_arm_type.is_none() {
                let err_span= get_pattern_span(&arm.pat);
                let err_msg = "unsupported by #[sorted]";
                let err = Error::new(err_span, err_msg);
                return Err(err)
            }

            let cur_arm_type = cur_arm_type.unwrap();
            // if pattern doesn't match rest then return appropriate error
            if !ArmTypes::my_eq(&cur_arm_type, &match_pattern) {
                let err_span= get_pattern_span(&arm.pat);
                let err_msg = "This pattern doesn't match the other arms form";
                let err = Error::new(err_span, err_msg);
                return Err(err)
            }

            // if the current match pattern is Wild then replace  this guarantees
            // that if ValidateArms has wild arm_type then all arms must be wild
            if let ArmTypes::Wild = match_pattern {
                match_pattern = cur_arm_type;
            }

        }
        // At this point all arms have the same type of pattern

        Ok( ValidatedArms {
                arm_type: match_pattern,
                arms
            })
    }

    /// Checks if arms is sorted.
    /// Returns OK(()) if sorted
    /// Returns Err(_) with an appropriate error otherwise
    pub fn is_sorted(&self) -> Result<()> {

        // return Ok(()) i.e. sorted if there are no arms
        match self.arm_type {
            ArmTypes::Path => { check_pat_sorted(self.arms) },
            ArmTypes::Ident => { check_pat_sorted(self.arms) },
            ArmTypes::TupleStruct => { check_pat_sorted(self.arms) },
            ArmTypes::Wild => Ok(()), // Only possible if all arms are wild arms
            ArmTypes::Empty =>  Ok(())
        }
    }
}

/// Checks if the arms are sorted and returns the appropriate errors
/// Arms are guaranteed to be at least one.
fn check_pat_sorted(arms : &[Arm]) -> Result<()> {
    let mut patterns_iter = arms.iter().map(|arm| {
        &arm.pat
    });

    // get first pattern
    let prev_pattern = patterns_iter.next().unwrap();

    for curr_pattern in patterns_iter {

        // check prev <= current
        if pat_less_eq_than(prev_pattern, curr_pattern) {
            continue;
        }

        // know curr_pattern is out of order i.e error

        // get arm it should be before
        let after_arm = arms.iter()
                    .find(|&arm| !pat_less_eq_than(&arm.pat, &curr_pattern))
                    .unwrap();

        // get idents
        let cur_ident = get_pattern_ident(&curr_pattern);
        let after_ident = get_pattern_ident(&after_arm.pat);

        // create error msg
        let err_msg = format!("{} should sort before {}", cur_ident, after_ident);
        let err_span = get_pattern_span(curr_pattern);
        let err = Error::new(err_span, err_msg);
        return Err(err);
    }

    Ok(())
}

/// if a <= b where a and b are Pat::Path or Pat::Wild
fn pat_less_eq_than(a:&Pat, b: &Pat) -> bool {

    // check if b is wild
    if let Pat::Wild(_) = b { return true; }

    // Now know right is not wild from here on

    // check if a is wild, return false as means left wild right not
    if let Pat::Wild(_) = a { return false; }

    // Now know both a nd b are not wild

    // Try both are Pat::Path
    if let (Pat::Path( PatPath {
        attrs: _,
        qself: _,
        path: a_path
    }),
    Pat::Path( PatPath {
        attrs: _,
        qself: _,
        path: b_path
    })) = (a,b) {

        // Now have both paths
        let path_ord = path_ord(a_path, b_path);

        match path_ord {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Greater => return false,
        }
    }

    // Try both are Pat::TupleStruct
    if let (Pat::TupleStruct( PatTupleStruct{
        attrs:_,
        path: a_path,
        pat: _,
    }),
    Pat::TupleStruct( PatTupleStruct{
        attrs:_,
        path: b_path,
        pat: _,
    })) = (a,b) {

        // Now have both paths
        let path_ord = path_ord(a_path, b_path);

        match path_ord {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Greater => return false,
        }
    }


    // Try both are Pat::Ident
    if let (Pat::Ident(PatIdent {
        attrs: _,
        by_ref: _,
        mutability: _,
        ident: a_ident,
        subpat: _
    }),
    Pat::Ident(PatIdent {
        attrs: _,
        by_ref: _,
        mutability: _,
        ident: b_ident,
        subpat: _
    })) = (a,b) {

        // Now have both idents
        let ident_ord = Ident::cmp(a_ident, b_ident);

        match ident_ord {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Greater => return false,
        }

    }

    // should be one of the three above cases
    unreachable!()
}

//TODO should be string?
/// gets the appropiate ident. Should only need to be implemented for TupleStructs, Wild and Path
fn get_pattern_ident(pat : &Pat) -> String {

    match pat {
        Pat::Wild(_) => "_".to_string(),
        Pat::TupleStruct(tuple_struct) => {
            format_punct(&tuple_struct.path.segments)
        },
        Pat::Path(path) => {
            format_punct(&path.path.segments)
        }
        Pat::Ident(ident_struct) => ident_struct.ident.to_string(),
        _ => unimplemented!()
    }
}

/// get the span associated with a pattern.
fn get_pattern_span(pat: &Pat) -> Span {

    match pat {
        Pat::Wild(wild) => wild.underscore_token.spans[0],
        Pat::TupleStruct(tuple_struct) => {tuple_struct.path.segments.span()},
        Pat::Path(path) => path.path.segments.span(),
        Pat::Box(pat_box) => pat_box.box_token.span,
        Pat::Ident(pat_ident) => pat_ident.ident.span(),
        Pat::Lit(_) => unimplemented!(),
        Pat::Macro(_) => unimplemented!(),
        Pat::Or(_) => unimplemented!(),
        Pat::Range(_) => unimplemented!(),
        Pat::Reference(pat_ref) => pat_ref.and_token.spans[0],
        Pat::Rest(pat_rest) => pat_rest.dot2_token.spans[0],
        Pat::Slice(pat_slice) => pat_slice.bracket_token.span,
        Pat::Struct(_) => unimplemented!(),
        Pat::Tuple(pat_tuple) => pat_tuple.paren_token.span,
        Pat::Type(pat_typ) => get_pattern_span(pat_typ.pat.as_ref()),
        Pat::Verbatim(_) => unimplemented!(),
        _ => unimplemented!()
    }
}

// TODO
/// if a < b
fn path_ord(a : &Path, b: &Path) -> std::cmp::Ordering {

    // for each path segment find the bigger
    for (a_seg,b_seg) in a.segments.iter().zip(b.segments.iter()) {

        // if segments equal then skip
        if a_seg == b_seg {continue;}

        if a_seg.ident < b_seg.ident {
            // a less than b
            return std::cmp::Ordering::Less;
        }else{
            // a greater than b
            return std::cmp::Ordering::Greater;
        }
    }
    std::cmp::Ordering::Equal
}


/// puts punc as correct format
fn format_punct(punc : &Punctuated<PathSegment, Colon2>)  -> String {

    let mut str = String::new();

    for seg in punc {
        let str_add = format!("{}::",seg.ident);
        str.add_assign(str_add.as_str());
    }
    str.pop();
    str.pop();
    str
}
/*
///Checks if the arms are sorted and returns the appropriate errors
fn check_arms_sorted(arms : &[Arm]) -> Result<()> {

//create iterator
let mut arms_iterator = arms.iter();

// Gets first arm
let prev_arm = arms_iterator.next();

//See if no arms
if prev_arm.is_none() {
    // No arms
    return Ok(());
}
let mut prev_arm= prev_arm.unwrap();

// println!("{:?}", prev_arm.pat);

for curr_arm in arms_iterator {
    if arms_less_than(&curr_arm.pat, &prev_arm.pat)? {
        // current arm less than previous arm => ERROR

        /*
        // find pattern which curr_arm should be before
        //let after_arm = arms.iter()
                            .find(|&arm| { !arms_less_than(&arm.pat, &curr_arm.pat) })
                            .unwrap();

        */

        // create error

        //get idents
        //get both tuplestructs
        if let Pat::TupleStruct( PatTupleStruct{
            attrs:_,
            path: a_path,
            pat: _,
        }) = &curr_arm.pat {
        if let Pat::TupleStruct( PatTupleStruct{
            attrs:_,
            path: b_path,
            pat: _,
        }) = &after_arm.pat  {

            let (a_ident, b_ident) = (&a_path.segments.last().unwrap().ident, &b_path.segments.last().unwrap().ident);     //segments.last().unwrap().ident;
            let err_msg = format!("{} should sort before {}", a_ident, b_ident);
            let err = Error::new(get_pattern_span(&curr_arm.pat), err_msg);
            return Err(err);
        }}
    }

    prev_arm = curr_arm;
}

Ok(())
}*/



/*
    // TODO
    /// If a < b
    fn arms_less_than(a : &Pat, b : &Pat) -> Result<bool> {

    // Return true if b is wildcard
    if let Pat::Wild(_) = b { return Ok(true); }

    //get both tuplestructs
    if let (Pat::TupleStruct( PatTupleStruct{
                attrs:_,
                path: a_path,
                pat: _,
            }),
            Pat::TupleStruct( PatTupleStruct{
                attrs:_,
                path: b_path,
                pat: _,
            })) = (a,b)
    {
    //return Ok(path_less_than(a_path, b_path))
}

let err = Error::new(Span::call_site(), "AAAA");
Err(err)
}*/


/*

/// checks if fields are sorted
fn _fields_sorted(a : &Punctuated<FieldPat, Comma>) -> Result<()>{

    if a.is_empty() { return Ok(()) }

    let mut fields_iter = a.iter().map(|field| &field.member);

    let prev_field_mem =  fields_iter.next().unwrap();

    let prev_id;

    if let Member::Named(a_id) = prev_field_mem {
        prev_id = a_id;
    }
    else { unimplemented!() }

    // for each path segment find the bigger
    for curr_member in fields_iter {

        // get idents
        if let Member::Named(curr_id) = curr_member {

            //check idents
            if prev_id <= curr_id { continue; }

            // Now know fields are out of order.
            let after_id = a.iter().map(|field| {
                if let Member::Named(id) = &field.member {
                    id
                } else {unimplemented!()}
                }).find(|id|(*id>curr_id))
                .unwrap();

            // create error
            let err_msg = format!("{} should sort before {}", curr_id, after_id);
            let err = Error::new(curr_id.span(), err_msg);
            return Err(err);

        }else {unimplemented!()}


    }
    Ok(())
}*/