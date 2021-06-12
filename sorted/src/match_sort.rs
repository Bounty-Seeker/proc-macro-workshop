use syn::visit_mut::{self, VisitMut};
use syn::{Attribute, Error, ExprMatch, Ident, ItemFn, Meta, PathArguments, PathSegment, Result};

mod validate_arms;

pub fn sorted_check_macro(input_item: &mut ItemFn) -> Result<()> {
    //create visitor
    let mut visitor = MatchSortVisitor::new();

    // check if sorted
    visitor.sorted_check(input_item);

    // get error if there is one and return
    visitor.get_error()
}

/// Struct which implements VisitMut which checks
struct MatchSortVisitor {
    sorted_error: Option<Error>,
}

impl MatchSortVisitor {
    /// creates MatchSortVisitor
    fn new() -> Self {
        Self { sorted_error: None }
    }

    /// gets the error if one exists
    fn get_error(self) -> Result<()> {
        // get error if there is one and return
        let err = self.sorted_error;

        match err {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    /// check ItemFn if sorted
    fn sorted_check(&mut self, item_fn: &mut ItemFn) {
        self.visit_item_fn_mut(item_fn);
    }

    /// Adds error to self. Extends error if Some
    fn set_error(&mut self, err: Error) {
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

            // Check if all arms are valid
            let arms = validate_arms::ValidatedArms::validate_arms(&node.arms[..]);

            match arms {
                Ok(validated_arms) => {
                    // check if branches of sorted function are sorted
                    let arms_sorted: Result<()> = validated_arms.is_sorted();

                    // set err on self
                    if let Err(err) = arms_sorted {
                        self.set_error(err);
                    }
                }
                Err(err) => {
                    // set err n self
                    self.set_error(err);
                }
            }
        }

        // call normal visit_mut match fn to continue recursion
        visit_mut::visit_expr_match_mut(self, node)
    }
}

/// checks if a given attribute is the expected sorted attribute
fn is_sorted_attribute(attr: &Attribute) -> bool {
    // Parse to Meta
    if let Ok(Meta::Path(path)) = attr.parse_meta() {
        // Know its a path now, check has no leading colon and is the correct length
        if path.leading_colon.is_none() && path.segments.len() == 1 {
            // check path_segment is correct
            if let PathSegment {
                ident,
                arguments: PathArguments::None,
            } = path.segments.first().unwrap()
            {
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
