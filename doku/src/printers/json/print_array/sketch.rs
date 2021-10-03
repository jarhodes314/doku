mod expand_variants;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn sketch_array(&mut self, ty: &'ty Type, size: Option<usize>) {
        if self.inline {
            self.out.write("[ ");
        } else {
            self.out.writeln("[");
            self.out.inc_indent();
        }

        if self.try_expanding_variants(ty) {
            //
        } else if let Some(example) = self.example() {
            let examples: Vec<_> = example.iter().collect();

            for example in &examples {
                self.nested()
                    .with_ty(ty)
                    .with_example(Some(*example))
                    .print();

                if self.inline {
                    self.out.write(", ");
                } else {
                    self.out.writeln(",");
                }
            }

            if size.map_or(true, |size| examples.len() < size) {
                self.out.write("/* ... */");

                if !self.inline {
                    self.out.ln();
                }
            }
        } else {
            self.nested().with_ty(ty).print();

            if self.inline {
                self.out.write(", /* ... */");
            } else {
                self.out.writeln(",");
                self.out.writeln("/* ... */");
            }
        }

        if self.inline {
            self.out.write(" ]");
        } else {
            self.out.dec_indent();
            self.out.write("]");
        }
    }
}
