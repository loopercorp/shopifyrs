use shopifyrs::verify_shopify_request;

#[test]
fn verify_installation_request_works() {
    let url = "https://example.com/?host=YWRtaW4uc2hvcGlmeS5jb20vc3RvcmUvbG9vcGVyY29ycC1kZXY&hmac=b90f7bddd2e7dc9d74d08da49975bc5a84c5e0ae593bd9ed5ef975dbb2db4b3c&shop=example.myshopify.com&timestamp=1698493895";
    let secret = "very strong secert";
    assert_eq!(verify_shopify_request(url, secret), true);
}
