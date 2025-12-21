use std::{
    io,
    ops::{Deref, DerefMut},
};

use crate::KotoTrace;

/// Wraps any value to implement `KotoTrace` as a no-op
///
/// **Note:** This can lead to memory leaks if the wrapped
/// value participates in reference cycles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, KotoTrace)]
#[koto(memory = crate, trace(ignore))]
pub struct Untrace<T: ?Sized>(pub T);

impl<T: Deref + ?Sized> Deref for Untrace<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DerefMut + ?Sized> DerefMut for Untrace<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Iterator + ?Sized> Iterator for Untrace<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T: DoubleEndedIterator + ?Sized> DoubleEndedIterator for Untrace<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<T: io::Read> io::Read for Untrace<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<T: io::Write> io::Write for Untrace<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<T: io::Seek> io::Seek for Untrace<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}
