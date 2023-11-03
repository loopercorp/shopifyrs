use std::borrow::Cow;
use url::Url;
use hex;
use hmacsha::HmacSha;
use sha3::Sha3_256;

// https://shopify.dev/docs/apps/auth/oauth/getting-started#step-2-verify-the-installation-request
pub fn verify_shopify_request(url: &str, client_secret: &str) -> bool {
    if let Ok(url) = Url::parse(url) {
        if let Some(query) = url.query() {
            let hmac_pair = url.query_pairs()
                .find_map(|(k, v)| {
                    if k == Cow::Borrowed("hmac") {
                        Some((k.to_string(), v.to_string()))
                    } else {
                        None
                    }
                });

            if let Some(hmac_pair) = hmac_pair {
                let hmac = format!("&hmac={}", hmac_pair.1);
                let query_str_without_hmac = query.replace(&hmac, "");

                let mut hasher = HmacSha::from(client_secret, &query_str_without_hmac, Sha3_256::default());
                let result = hasher.compute_digest();

                return hmac_pair.1 == hex::encode(result);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let url = "https://example.com/?host=YWRtaW4uc2hvcGlmeS5jb20vc3RvcmUvbG9vcGVyY29ycC1kZXY&hmac=b90f7bddd2e7dc9d74d08da49975bc5a84c5e0ae593bd9ed5ef975dbb2db4b3c&shop=example.myshopify.com&timestamp=1698493895";
        let secret = "very strong secert";
        assert_eq!(verify_shopify_request(url, secret), true);
    }
}
