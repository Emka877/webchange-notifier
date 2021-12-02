use crate::tools::{compare_pages, CompareResult, fetch_remote_page};

const DUMMY_BASE: &'static str = r#"<html>
    <main id="content">
        Base page
    </main>
</html>"#;
const DUMMY_CHANGED: &'static str = r#"<html>
    <main id="content">
        Content changed
    </main>
</html>"#;

#[test]
pub fn local_dummy_page_changed() {
    let compare_result: CompareResult = compare_pages(DUMMY_BASE, DUMMY_CHANGED);
    assert_eq!(compare_result, CompareResult::Different);
}

#[test]
pub fn local_dummy_page_unchanged() {
    let compare_result: CompareResult = compare_pages(DUMMY_BASE, DUMMY_BASE);
    assert_eq!(compare_result, CompareResult::Same);
}

#[tokio::test]
pub async fn page_doesnt_exist() {
    let fantasy_url: String = "https://wow.idontexist.tryagain/".to_owned();
    let reply = fetch_remote_page(fantasy_url).await;
    assert_eq!(reply.is_err(), true);
}
