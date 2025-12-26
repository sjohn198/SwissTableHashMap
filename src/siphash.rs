use std::slice::ChunksExact;

fn sipround(v0: &mut u64, v1: &mut u64, v2: &mut u64, v3: &mut u64) -> () {
    *v0 = v0.wrapping_add(*v1);
    *v1 = (*v1).rotate_left(13);
    *v1 ^= *v0;
    *v0 = (*v0).rotate_left(32);
    *v2 = v2.wrapping_add(*v3);
    *v3 = (*v3).rotate_left(16);
    *v3 ^= *v2;
    *v0 = v0.wrapping_add(*v3);
    *v3 = (*v3).rotate_left(21);
    *v3 ^= *v0;
    *v2 = v2.wrapping_add(*v1);
    *v1 = (*v1).rotate_left(17);
    *v1 ^= *v2;
    *v2 = (*v2).rotate_left(32);
}

/*input_len and outlen args are likely outdated. should remove in actual implementation*/
fn siphash(input: &[u8], input_len: usize, key: &[u8; 16], output: &mut [u8], outlen: usize) -> () {
    assert!(outlen == 8 || outlen == 16);
    let mut v0: u64 = 0x736F6D6570736575;
    let mut v1: u64 = 0x646F72616E646F6D;
    let mut v2: u64 = 0x6c7967656e657261;
    let mut v3: u64 = 0x7465646279746573;

    let k0: u64 = u64::from_le_bytes(key[..8].try_into().unwrap());
    let k1: u64 = u64::from_le_bytes(key[8..].try_into().unwrap());
    v3 ^= k1;
    v2 ^= k0;
    v1 ^= k1;
    v0 ^= k0;

    if outlen == 16 {
        v1 ^= 0xee;
    }

    let mut chunks_iter:ChunksExact<u8> = input.chunks_exact(8);
    for chunk in chunks_iter.by_ref() {
        let m = u64::from_le_bytes(chunk.try_into().unwrap());
        v3 ^= m;
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);
        v0 ^= m;
    }

    let remainder: &[u8] = chunks_iter.remainder();
    let mut b: u64 = (input_len as u64) << 56;
    for (i, &byte) in remainder.iter().enumerate() {
        b |= (byte as u64) << (i * 8);
    }

    v3 ^= b;
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= b;

    if outlen == 16 {
        v2 ^= 0xee;
    } else {
        v2 ^= 0xff;
    }

    for i in 0..3 {
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    }

    b = v0 ^ v1 ^ v2 ^ v3;
    output[0..8].copy_from_slice(&b.to_le_bytes());

    if outlen == 8 {
        return;
    }

    v1 ^= 0xdd;

    for i in 0..3 {
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);        
    }

    b = v0 ^ v1 ^ v2 ^ v3;
    output[8..16].copy_from_slice(&b.to_le_bytes());
}