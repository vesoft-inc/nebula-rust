/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::varint;
use bufsize::SizeCounter;
use bytes::buf::ext::Chain;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::Cursor;

pub trait BufExt: Buf {
    /// Reset buffer back to the beginning.
    fn reset(self) -> Self;

    fn copy_to_bytes(&mut self, len: usize) -> Bytes;
}

impl BufExt for Cursor<Bytes> {
    fn reset(self) -> Self {
        Cursor::new(self.into_inner())
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        // This is available as standard in Bytes 1.0
        let pos = self.position() as usize;
        let end = pos + len;
        // Panics if len is too large (same as Bytes 1.0)
        let bytes = self.get_ref().slice(pos..end);
        self.set_position(end as u64);
        bytes
    }
}

impl<T: AsRef<[u8]> + ?Sized> BufExt for Cursor<&T> {
    fn reset(self) -> Self {
        Cursor::new(self.into_inner())
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        // This is available as standard in Bytes 1.0
        let pos = self.position() as usize;
        let end = pos + len;
        // Panics if len is too large (same as Bytes 1.0)
        let all = self.get_ref().as_ref();
        let bytes = Bytes::copy_from_slice(&all[pos..end]);
        self.set_position(end as u64);
        bytes
    }
}

impl<T: BufExt, U: BufExt> BufExt for Chain<T, U> {
    fn reset(self) -> Self {
        let (a, b) = self.into_inner();
        Chain::new(a.reset(), b.reset())
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        let a = self.first_mut();
        if a.has_remaining() {
            if a.remaining() >= len {
                a.copy_to_bytes(len)
            } else {
                // data is split across the two buffers
                assert!(self.remaining() >= len);
                let mut out = BytesMut::with_capacity(len);
                let mut remaining = len;
                while remaining > 0 {
                    let part = self.bytes();
                    let part_len = part.len().min(remaining);
                    out.copy_from_slice(&part[..part_len]);
                    self.advance(part_len);
                    remaining -= part_len;
                }
                out.freeze()
            }
        } else {
            self.last_mut().copy_to_bytes(len)
        }
    }
}

pub trait BufMutExt: BufMut {
    type Final: Send + 'static;

    fn put_varint_u64(&mut self, v: u64)
    where
        Self: Sized,
    {
        varint::write_u64(self, v)
    }

    fn put_varint_i64(&mut self, v: i64)
    where
        Self: Sized,
    {
        varint::write_u64(self, varint::zigzag(v))
    }

    fn finalize(self) -> Self::Final;
}

impl BufMutExt for BytesMut {
    type Final = Bytes;

    fn finalize(self) -> Self::Final {
        self.freeze()
    }
}

impl BufMutExt for SizeCounter {
    type Final = usize;

    #[inline]
    fn put_varint_u64(&mut self, v: u64) {
        self.put_uint(v, varint::u64_len(v));
    }

    #[inline]
    fn put_varint_i64(&mut self, v: i64) {
        self.put_int(v, varint::u64_len(varint::zigzag(v)));
    }

    #[inline]
    fn finalize(self) -> Self::Final {
        self.size()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bytes_reset() {
        let b = Bytes::from(b"hello, world".to_vec());
        let mut c = Cursor::new(b);

        assert_eq!(c.remaining(), 12);
        assert_eq!(c.get_u8(), b'h');

        c.advance(5);
        assert_eq!(c.remaining(), 6);
        assert_eq!(c.get_u8(), b' ');

        let mut c = c.reset();
        assert_eq!(c.remaining(), 12);
        assert_eq!(c.get_u8(), b'h');
    }

    #[test]
    fn test_empty_bytes_reset() {
        let b = Bytes::from(Vec::new());
        let c = Cursor::new(b);

        assert_eq!(c.remaining(), 0);

        let c = c.reset();
        assert_eq!(c.remaining(), 0);
    }
}
