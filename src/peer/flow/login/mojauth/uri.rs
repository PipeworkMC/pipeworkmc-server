use pipeworkmc_data::redacted::Redacted;
use core::ptr;
use openssl::sha::Sha1;
use ethnum::i256;


const MOJAUTH_URL_PREFIX   : &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username=";
const MOJAUTH_URL_SERVERID : &str = "&serverId=";


pub(in super::super) fn build_mojauth_uri(
    server_id            : &str,
    decrypted_secret_key : &Redacted<&[u8]>,
    pkeyder              : &Redacted<Vec<u8>>,
    declared_username    : &str
) -> ([u8; MOJAUTH_URL_PREFIX.len() + 16 + MOJAUTH_URL_SERVERID.len() + 41], usize,) {
    // Build server ID.
    let mut sha = Sha1::new();
    sha.update(server_id.as_bytes());
    sha.update(unsafe { decrypted_secret_key.as_ref() });
    sha.update(unsafe { pkeyder.as_ref() });
    let     sha_in_20 = sha.finish();

    let mut sha_in_32   = [0u8; 32];
    unsafe { ptr::copy_nonoverlapping(sha_in_20.as_ptr(), sha_in_32.as_mut_ptr(), 20); }
    let     sha_in_i256 = i256::from_be_bytes(sha_in_32);
    let mut sha_buf     = [0u8; 40];
    if (sha_in_i256 >= 0) {
        _ = hex::encode_to_slice(sha_in_20, &mut sha_buf);
    } else {
        let neg_sha_in_32 = (-sha_in_i256).to_be_bytes();
        // SAFETY: sha_in_32 bytes has 32 items.
        _ = hex::encode_to_slice(unsafe { neg_sha_in_32.get_unchecked(0..20) }, &mut sha_buf);
    }
    // SAFETY: sha_buf has 40 items.
    let sha_buf = unsafe { sha_buf.get_unchecked((sha_buf.iter().position(|&x| x != b'0').unwrap_or(39))..40) };

    // Build mojauth URI.
    let mut url_buf = [0u8; MOJAUTH_URL_PREFIX.len() + 16 + MOJAUTH_URL_SERVERID.len() + 41];
    let mut url_ptr = 0;
    // SAFETY: url_buf has enough space for `MOJAUTH_URL_PREFIX`, `declared_username`, `MOJAUTH_URL_SERVERID`, and `sha_buf`.
    //         None are written to overlap each other.
    //         declared_username can not be longer than 16 bytes (checked above).
    {
        unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_PREFIX.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_PREFIX.len()); }
        url_ptr += MOJAUTH_URL_PREFIX.len();
        unsafe { ptr::copy_nonoverlapping(declared_username.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), declared_username.len()); }
        url_ptr += declared_username.len();
        unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_SERVERID.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_SERVERID.len()); }
        url_ptr += MOJAUTH_URL_SERVERID.len();
        if (sha_in_i256 < 0) {
            unsafe { url_buf.as_mut_ptr().byte_add(url_ptr).write(b'-'); }
            url_ptr += 1;
        }
        unsafe { ptr::copy_nonoverlapping(sha_buf.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), sha_buf.len()); }
        url_ptr += sha_buf.len();
    }

    (url_buf, url_ptr,)
}
