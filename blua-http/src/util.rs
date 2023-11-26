use bytes::{Buf, BufMut, Bytes};
use hyper::body::SizeHint;

pub async fn to_bytes(body: &mut reqwest::Response) -> reqwest::Result<Bytes> {
    // If there's only 1 chunk, we can just return Buf::to_bytes()
    let mut first = if let Some(buf) = body.chunk().await? {
        buf
    } else {
        return Ok(Bytes::new());
    };

    let second = if let Some(buf) = body.chunk().await? {
        buf
    } else {
        return Ok(first.copy_to_bytes(first.remaining()));
    };

    // Don't pre-emptively reserve *too* much.
    let rest = (body
        .content_length()
        .map(|ln| SizeHint::with_exact(ln))
        .unwrap_or_default()
        .lower() as usize)
        .min(1024 * 16);
    let cap = first
        .remaining()
        .saturating_add(second.remaining())
        .saturating_add(rest);
    // With more than 1 buf, we gotta flatten into a Vec first.
    let mut vec = Vec::with_capacity(cap);
    vec.put(first);
    vec.put(second);

    while let Some(buf) = body.chunk().await? {
        vec.put(buf);
    }

    Ok(vec.into())
}
