use super::Read;
use core::cmp;

impl Read for &[u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> super::Result<usize> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }
}
