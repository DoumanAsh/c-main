//! C style arguments wrapper

#[derive(Copy, Clone, Debug)]
///Wrapper over C-style arguments
pub struct Args {
    argc: usize,
    argv: *const *const u8,
}

impl Args {
    ///Creates new instance, but verifies that each string inside are UTF-8.
    ///
    ///On error returns pair: `(string index, Utf8Error)`
    ///
    ///The function is safe as long as you pass C style main function arguments.
    pub unsafe fn new(argc: isize, argv: *const *const u8) -> Result<Self, (usize, core::str::Utf8Error)> {
        assert!(argc > 0);
        assert!(!argv.is_null());

        let this = Args {
            argc: argc as usize,
            argv,
        };

        let args = this.as_slice();
        for idx in 0..this.argc {
            let arg = *args.get_unchecked(idx);
            if let Err(error) = crate::c_str_to_rust(arg) {
                return Err((idx, error));
            }
        }

        Ok(this)
    }

    #[inline(always)]
    ///Unchecked version of `Args::new`
    ///
    ///Do it on your own risk
    pub unsafe fn new_unchecked(argc: isize, argv: *const *const u8) -> Self {
        Args {
            argc: argc as usize,
            argv,
        }
    }

    #[inline(always)]
    ///Returns slice of raw C strings
    pub fn as_slice(&self) -> &[*const u8] {
        unsafe {
            core::slice::from_raw_parts(self.argv, self.argc)
        }
    }

    #[inline(always)]
    ///Retrieves string by index.
    ///
    ///No checks, 100% unsafe.
    pub unsafe fn get_str_by_index(&self, index: usize) -> &str {
        let elem = *self.as_slice().get_unchecked(index);
        crate::c_str_to_rust_unchecked(elem)
    }
}

impl<'a> IntoIterator for &'a Args {
    type Item = &'a str;
    type IntoIter = IntoIter<'a>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self,
            index: 0,
        }
    }
}

///Iterator over [Args](struct.Args.html)
///
///Comparing to normal iterators can be iterated back and forth.
pub struct IntoIter<'a> {
    inner: &'a Args,
    index: usize,
}

impl<'a> Iterator for IntoIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.argc {
            return None;
        }

        let elem = unsafe {
            self.inner.get_str_by_index(self.index)
        };
        self.index += 1;
        Some(elem)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.inner.argc - self.index;
        (count, Some(count))
    }

    #[inline(always)]
    fn count(self) -> usize {
        self.inner.argc - self.index
    }
}

