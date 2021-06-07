use proc_macro2::Span;
use syn::{Arm, Attribute, Error, ExprMatch, Ident, ItemFn, Meta, Pat, PathArguments, PathSegment, PatTupleStruct, Result};
use syn::visit_mut::{self, VisitMut};

pub fn sorted_check_macro(input_item :&mut ItemFn) -> Result<()> {

    //create visitor
    let mut visitor = MatchSortVisitor::new();

    // check if sorted
    visitor.sorted_check(input_item);

    // get error if there is one and return
    let err = visitor.get_error();

    match err {
        Some(err) => Err(err),
        None => Ok(())
    }
}


/// Struct which implements VisitMut which checks
struct MatchSortVisitor {
    sorted_error:Option<Error>
}

impl MatchSortVisitor {

    /// creates MatchSortVisitor
    fn new() -> Self {
        Self{sorted_error:None}
    }

    /// gets the error if one exists
    fn get_error(self) -> Option<Error>{
        self.sorted_error
    }

    /// check ItemFn if sorted
    fn sorted_check(&mut self, item_fn : &mut ItemFn) {
        self.visit_item_fn_mut(item_fn);
    }

    /// Adds error to self. Extends error if Some
    fn set_error(&mut self, err : Error) {

        match &mut self.sorted_error {
            Some(old_err) => old_err.extend(err),
            None => self.sorted_error = Some(err),
        }
    }
}

impl VisitMut for MatchSortVisitor {

    fn visit_expr_match_mut(&mut self, node: &mut ExprMatch) {

        // check if has sorted attribute and remove
        let index_option = node.attrs.iter().position(is_sorted_attribute);

        if let Some(index) = index_option {
            // from here on we know it has a sorted attribute

            // remove sorted attribute from vec
            node.attrs.remove(index);

            // check if branches of sorted function are sorted
            let arms_sorted: Result<()> = check_arms_sorted(&node.arms);

            // set err on self
            if let Err(err) = arms_sorted {
                self.set_error(err);
            }
        }

        // call normal visit_mut match fn to continue recursion
        visit_mut::visit_expr_match_mut(self, node)
    }

}

/// checks if a given attribute is the expected sorted attribute
fn is_sorted_attribute(attr : &Attribute) -> bool {

    // Parse to Meta
    if let Ok(Meta::Path(path)) = attr.parse_meta(){
        // Know its a path now, check has no leading colon and is the correct length
        if path.leading_colon.is_none() && path.segments.len() == 1 {

            // check path_segment is correct
            if let PathSegment {
                    ident,
                    arguments: PathArguments::None
                } = path.segments.first().unwrap() {

                    // check ident
                    let sorted_ident = Ident::new("sorted", ident.span());
                    if sorted_ident == *ident {
                        // Have sorted attribute
                        return true;
                    }
                }
        }
    }

    false
}

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
        if arms_less_than(&curr_arm.pat, &prev_arm.pat) {
            // current arm less than previous arm => ERROR

            // find pattern which curr_arm should be before
            let after_arm = arms.iter()
                                .find(|&arm| { !arms_less_than(&arm.pat, &curr_arm.pat) })
                                .unwrap();

            

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
}

fn get_pattern_span(pat: &Pat) -> Span {
    if let Pat::TupleStruct( PatTupleStruct{
        attrs:_,
        path,
        pat: _,
    }) = pat {
        return path.segments.last().unwrap().ident.span();
    }
    println!("Pattern Span\n\n\n\n\n");
    unimplemented!()
}

/// If a < b
fn arms_less_than(a : &Pat, b : &Pat) -> bool {

    // Return true if b is wildcard
    if let Pat::Wild(_) = b { return true; }

    //get both tuplestructs
    if let Pat::TupleStruct( PatTupleStruct{
        attrs:_,
        path: a_path,
        pat: _,
    }) = a {
        if let Pat::TupleStruct( PatTupleStruct{
            attrs:_,
            path: b_path,
            pat: _,
        }) = b  {

            // for each path segment find the bigger
            for (a_seg,b_seg) in a_path.segments.iter().zip(b_path.segments.iter()) {

                // if segments equal then skip
                if a_seg == b_seg {continue;}

                return a_seg.ident < b_seg.ident;

            }
        }
    }

    println!("Less than \n \n\n\n\n\n");
    unimplemented!()
}
















