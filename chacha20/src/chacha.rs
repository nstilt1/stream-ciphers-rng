use cipher::{StreamCipherSeek, SeekNum, OverflowError};
pub use cipher::{
    consts::{U10, U12, U32, U4, U6, U64, U256, U8},
    generic_array::GenericArray,
    BlockSizeUser, IvSizeUser, KeyIvInit, KeySizeUser, StreamCipherCore, StreamCipherCoreWrapper,
    StreamCipherSeekCore, StreamCipher, StreamCipherError, inout::InOutBuf,
};

use crate::{ChaChaCore, Rounds, R20, R8, R12, IETF};

/// Key type used by all ChaCha variants.
pub type Key = GenericArray<u8, U32>;

/// Nonce type used by ChaCha variants.
pub type Nonce = GenericArray<u8, U12>;

/// ChaCha8 stream cipher (reduced-round variant of [`ChaCha20`] with 8 rounds)
pub type ChaCha8 = ChaCha<R8>;

/// ChaCha12 stream cipher (reduced-round variant of [`ChaCha20`] with 12 rounds)
pub type ChaCha12 = ChaCha<R12>;

/// ChaCha20 stream cipher (RFC 8439 version with 96-bit nonce)
pub type ChaCha20 = ChaCha<R20>;

type Buffer = GenericArray<u8, U256>;

pub(crate) type Block = GenericArray<u8, U64>;

pub struct ChaCha<R: Rounds> {
    block: ChaChaCore<R, IETF>,
    buffer: Buffer,
    pos: usize
}

impl<R: Rounds> KeySizeUser for ChaCha<R> {
    type KeySize = U32;
}

impl<R: Rounds> IvSizeUser for ChaCha<R> {
    type IvSize = U12;
}

impl<R: Rounds> BlockSizeUser for ChaCha<R> {
    type BlockSize = U256;
}


impl<R: Rounds> KeyIvInit for ChaCha<R> {
    #[inline]
    fn new(key: &Key, iv: &Nonce) -> Self {
        Self {
            block: ChaChaCore::new(key.as_ref(), iv.as_ref()),
            buffer: Buffer::default(),
            pos: 0
        }
    }
}

impl<R: Rounds> ChaCha<R> {
    pub fn get_block_pos(&self) -> u32 {
        self.block.state[12]
    }
    pub fn set_block_pos(&mut self, pos: u32) {
        self.block.state[12] = pos
    }
}

/// A little macro to impl StreamCipher for a wrapper
// TODO: fill this macro so that it can be called for XChaCha and legacy,
//       and add support for different Variants
macro_rules! impl_stream_cipher {
    ($Name:ident, $Core:ident, $Rounds:ty) => {
        pub type $Name = $Core<$Rounds>;
        impl<R: Rounds> StreamCipher for $Core {
            fn try_apply_keystream_inout(
                &mut self,
                data: InOutBuf<'_, '_, u8>,
            ) -> Result<(), StreamCipherError> {
                Ok(())
            }
        }
    };
}

impl<R: Rounds> ChaCha<R> {
    fn remaining_blocks(&self) -> usize {
        (u32::MAX - self.get_block_pos()) as usize - (self.pos >> 4)
    }
}
/*
impl<R: Rounds> StreamCipherSeek for ChaCha<R> {
    fn try_current_pos<SN: SeekNum>(&self) -> Result<SN, OverflowError> {
        SN::from_block_byte(self.get_block_pos(), self.pos as u8, U8)
    }
    fn try_seek<T: SeekNum>(&mut self, pos: T) -> Result<(), StreamCipherError> {
        
    }
}
*/

impl<R: Rounds> StreamCipher for ChaCha<R> {
    fn try_apply_keystream_inout(
            &mut self,
            mut data: InOutBuf<'_, '_, u8>,
        ) -> Result<(), StreamCipherError> {
            // return error if the counter needs to loop
            // get the amount of full 64-byte blocks
            let blocks_to_increment = (data.len() >> 6)
            // add extra if extra blocks need to be generated, ie
            // if self.pos + (data.len() % 64) > 64
                + ((((data.len() >> 2) & 0x0F) + self.pos) >> 4);

            if blocks_to_increment > self.remaining_blocks() {
                return Err(StreamCipherError);
            }

            let pos = self.pos;
            if pos != 0 {
                let rem = &self.buffer[pos..];
                let n = data.len();
                if n < rem.len() {
                    data.xor_in2out(&rem[..n]);
                    self.pos = pos + n;
                    return Ok(());
                }
                let (mut left, right) = data.split_at(rem.len());
                data = right;
                left.xor_in2out(rem);
            }
    
            let (blocks, mut leftover) = data.into_chunks::<U256>();
            for mut block in blocks {
                self.block.generate(self.buffer.as_mut_ptr());
                block.xor_in2out(&self.buffer);
            }
            self.block.generate(self.buffer.as_mut_ptr());
    
            let n = leftover.len();
            if n != 0 {
                leftover.xor_in2out(&self.buffer[..n]);
            }
            self.pos = n;
    
            Ok(())
    }
}