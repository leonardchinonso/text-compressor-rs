use crate::service::models::part::Part;

pub fn split_into_parts(text: String) -> Vec<Part> {
    let n = text.len();

    const INDEX: usize = 16385; // INDEX is 2^14

    if n < INDEX {
        return vec![Part(0, text)];
    }

    if n < INDEX {
        return vec![Part(0, text)];
    }

    let mut parts = Vec::new();
    let mut ptr = 0usize;

    let (mut l, mut r) = (0usize, INDEX);
    while l < n && r <= n {
        parts.push(Part(ptr, text[l..r].to_string()));
        ptr += 1;
        l = r;
        r += INDEX;
    }

    if l < n {
        parts.push(Part(ptr, text[l..].to_string()));
    }

    parts
}
